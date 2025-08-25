use std::{collections::HashMap, env};
use std::path::Path;
use std::sync::Mutex;
use tauri::State;
use tauri::{path::BaseDirectory, AppHandle, Emitter, Listener, Manager};
use tokio::process::Child;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use crate::event::{ExeProcessPayload, EVT_BACKGROUND_PROCESS_RESULT};

#[derive(serde::Serialize)]
pub struct StopResult {
    pub app_id: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DispatchedItem {
    pub app_id: i64,
    pub executable_name: String,
    pub full_executable_path: String,
    pub pid: Option<u32>,
    #[serde(skip)]
    pub child: Option<Child>
}

#[derive(Debug)]
pub struct InternalDispatchedItem {
    pub app_id: i64,
    pub executable_name: String,
    pub full_executable_path: String,
    pub pid: Option<u32>,
    pub child:  Option<Child>
}
pub struct GameDispatcherState (pub Mutex<HashMap<i64, DispatchedItem>>);

// Launches the executable using tokio::process::Command and waits for the process to exit.
// This way we knows when the process has exited and ping the frontend that the process has exited.
#[tauri::command(rename_all = "snake_case")]
pub async fn launch_executable(
    handle: tauri::AppHandle,
    state: State<'_, GameDispatcherState>,
    path: String,
    executable_name: String,
    path_len: i64,
    app_id: i64,
) -> Result<String, String> {
    let app = handle.clone();

    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let normalized_path = Path::new(&path).to_string_lossy().to_string();

    let game_folder_path = exe_dir
        .join("games")
        .join(app_id.to_string())
        .join(&normalized_path);
    let executable_path = game_folder_path.join(&executable_name);

    let normalized_path_clone = normalized_path.clone();
    let executable_name_clone = executable_name.clone();

    let full_executable_path = Path::new(&normalized_path_clone)
        .join(&executable_name_clone)
        .to_string_lossy()
        .to_string();
    
    // std::process::Command or tokio::process::Command
    let spawn_result = tokio::process::Command::new(&executable_path)
        .current_dir(game_folder_path)
        .spawn();

    match spawn_result {
        Ok(spawned_process) => {
            let pid = spawned_process.id();

            let running_payload = ExeProcessPayload {
                app_id: app_id.to_string(),
                executable_name: executable_name.clone(),
                full_executable_path: full_executable_path.clone(),
                pid: pid,
                running: true,
                status: None,
                error: None,
            };
            // Emit to tell the frontend that the process is running with the PID
            let running_payload_json = serde_json::to_value(&running_payload).unwrap();
            handle
                .emit(EVT_BACKGROUND_PROCESS_RESULT, running_payload_json)
                .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));

            {
                let mut map = state.0.lock().unwrap();
                map.insert(
                    app_id.clone(),
                    DispatchedItem {
                        app_id: app_id.clone(),
                        executable_name: executable_name.clone(),
                        full_executable_path: full_executable_path.clone(),
                        pid: pid.clone(),
                        child: Some(spawned_process),
                    }
                );
            }
 
            tokio::spawn(async move {
                let app_handle = handle.clone();
                let state = app_handle.state::<GameDispatcherState>();
                let child= {
                     let mut map = state.0.lock().unwrap();
                     map.get_mut(&app_id).unwrap()
                        .child
                        .take()
                };
                let exit_code = child.expect("asd").wait().await.expect("Failed to wait on child");
                {
                    let mut map = state.0.lock().unwrap();
                    map.remove(&app_id);
                }
                let exited_payload = ExeProcessPayload {
                    app_id: app_id.to_string(),
                    executable_name: executable_name.clone(),
                    full_executable_path: full_executable_path.clone(),
                    pid: pid,
                    running: false,
                    status: exit_code.code(),
                    error: None,
                };
                // Tell the frontend that the process has exited so it can update the UI
                let exited_payload_json = serde_json::to_value(&exited_payload).unwrap();
                handle
                    .emit(EVT_BACKGROUND_PROCESS_RESULT, exited_payload_json)
                    .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));
            });
            Ok(format!("Executable launched"))
        }
        Err(e) => {
            // When error, no `pid`, `running` is false, and `error` contains the error message
            let error_payload = ExeProcessPayload {
                app_id: app_id.to_string(),
                executable_name: executable_name.clone(),
                full_executable_path: full_executable_path.clone(),
                running: false,
                pid: None,
                status: None,
                error: Some(e.to_string()),
            };
            let error_payload_json = serde_json::to_value(&error_payload).unwrap();
            handle
                .emit("background_process_result", error_payload_json)
                .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));
            Err(format!("Failed to launch executable: {}", e))
        }
    }
}

// I can't find a way to reference the child process while it's being waited by the tokio::spawn or command spawn
// and then stop it, so we need to stop it using OS level commands such as `taskkill` on Windows.
// Similar:
// - [Kill child process while waiting for it](https://stackoverflow.com/questions/35093869/kill-child-process-while-waiting-for-it)
fn stop_dispatched_item(item: &DispatchedItem) -> Result<(), String> {

    let pid = item.pid.ok_or("No PID stored for this dispatched item")?;
    let mut sys = System::new_all();
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true,
        ProcessRefreshKind::nothing().with_exe(UpdateKind::OnlyIfNotSet)
    );

    if let Some(proc_) = sys.process(Pid::from_u32(pid)) {
        let proc_name = proc_.name().to_str()
            .ok_or("Failed to convert process name to string")?
            .to_string();
        // proc_path is just the name of the executable, not the full path
        let proc_path = proc_.exe().unwrap().file_name().unwrap().to_string_lossy().to_string();
        println!("process name: {}, path: {}", proc_name, proc_path);
        if proc_path.eq_ignore_ascii_case(&item.executable_name) {
            // Kill by PID not by name
            let output =  std::process::Command::new("taskkill")
                .arg("/F")
                .arg("/PID")
                .arg(pid.to_string())
                .output()
                .map_err(|e| format!("Failed to execute taskkill: {}", e))?;
            if output.status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Failed to stop process: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        } else {
            Err(format!(
                "Process info mismatch (wanted name: '{}', path: '{}', pid: '{}'), found name: '{}', path: '{}'",
                item.executable_name, item.full_executable_path, pid.to_string(),
                proc_name, proc_path
            ))
        }
    } else {
        Err(format!("Process with PID {} not found or already exited", pid))
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stop_executable(handle: tauri::AppHandle ,state: State<'_, GameDispatcherState>, app_id: &str) -> Result<String, String> {

    println!("Stopping executable with app_id: {}", app_id);
    let app_handle = handle.clone();
    
    let x: i64 = app_id.parse().map_err(|_| "Invalid app_id".to_string())?;
    let item = {
        let s: State<'_, GameDispatcherState> = app_handle.state::<GameDispatcherState>();
         let mut map = s.0.lock().unwrap();
        if let Some(dispatched_item) = map.remove(&x) {
            dispatched_item
        } else {
            return Err(format!("No running process found for app_id: {}", app_id));
        }
    };
    
    stop_dispatched_item(&item)?;

    // Prepare the payload to emit to the frontend
    let stopped_payload = ExeProcessPayload {
        app_id: x.to_string(),
        executable_name: item.executable_name,  
        full_executable_path: item.full_executable_path,
        pid: None,
        running: false,
        status: None,
        error: None,
    };
    let stopped_payload_json = serde_json::to_value(&stopped_payload).unwrap();
    app_handle
        .emit(EVT_BACKGROUND_PROCESS_RESULT, stopped_payload_json)
        .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));

    // Clean-up
    {
        let mut map = state.0.lock().map_err(|_| "Failed to lock dispatcher state".to_string())?;
        map.remove(&x);
    }
    Ok("Executable stopped successfully".to_string())
}

