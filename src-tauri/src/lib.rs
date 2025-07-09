// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use once_cell::sync::OnceCell;
use std::env;
use std::path::Path;
use std::sync::Mutex;
use tauri::{path::BaseDirectory, AppHandle, Emitter, Listener, Manager};

mod rpc;
mod runner;
mod commands;

// Global static instance of the Discord client
static DISCORD_CLIENT: OnceCell<Mutex<Option<rpc::Client>>> = OnceCell::new();

fn get_discord_client() -> &'static Mutex<Option<rpc::Client>> {
    DISCORD_CLIENT.get_or_init(|| Mutex::new(None))
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn create_fake_game(
    handle: tauri::AppHandle,
    path: &str,
    executable_name: &str,
    path_len: i64,
    app_id: i64,
) -> Result<String, String> {
    // Must create in the same directory as the executable to avoid permission issues
    // Get the executable directory to look for config file
    let exe_path: std::path::PathBuf = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let normalized_path = Path::new(path).to_string_lossy().to_string();

    let game_folder_path = exe_dir
        .join("games")
        .join(app_id.to_string())
        .join(normalized_path);

    println!("Game folder path: {:?}", game_folder_path);
    println!(
        "Game full path: {:?}",
        game_folder_path.join(executable_name)
    );

    // Ok(format!("Dummy executable copied to: {:?}", target_executable_path))
    match std::fs::create_dir_all(&game_folder_path) {
        Ok(_) => {
            println!("Successfully created directory: {:?}", game_folder_path);
        }
        Err(e) => return Err(format!("Failed to create game folder: {}", e)),
    };
    // copy the dummy executable to the created folder
    // there is a `template.exe` file along the final build.
    let resource_path = handle
        .path()
        .resolve("data/src-win.exe", BaseDirectory::Resource)
        .unwrap_or_default();

    println!("Creating dummy game executable: {:?}", resource_path);
    let dummy_executable_path = exe_dir.join("template.exe");
    let target_executable_path = game_folder_path.join(executable_name);
    match std::fs::copy(&resource_path, &target_executable_path) {
        Ok(_) => Ok(format!(
            "Dummy executable copied to: {:?}",
            target_executable_path
        )),
        Err(e) => Err(format!("Failed to copy dummy executable: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn run_background_process(
    handle: tauri::AppHandle,
    path: String,
    executable_name: String,
    path_len: i64,
    app_id: i64,
) {
    let app = handle.clone();

    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let normalized_path = Path::new(&path).to_string_lossy().to_string();

    let game_folder_path = exe_dir
        .join("games")
        .join(app_id.to_string())
        .join(&normalized_path);
    let executable_path = game_folder_path.join(&executable_name);
    // const DETACHED_PROCESS: u32 = 0x00000008;
    // const CREATE_NO_WINDOW: u32 = 0x08000000; // Hide the window

    let normalized_path_clone = normalized_path.clone();
    let executable_name_clone = executable_name.clone();

    // tauri::async_runtime::spawn(async move {
    let spawn_result = std::process::Command::new(&executable_path)
        .current_dir(game_folder_path) // Set working directory to the game folder
        .spawn();

    match spawn_result {
        Ok(mut child) => {
            let pid = child.id();

            let running_payload = serde_json::json!({
                "app_id": app_id.to_string(),
                "executable_name": executable_name.clone(),
                "full_executable_path": Path::new(&normalized_path_clone)
                    .join(&executable_name_clone)
                    .to_string_lossy()
                    .to_string(),
                "running": true,
                "pid": pid,
            });
            // Emit to tell the frontend that the process is running with the PID
            handle
                .emit("background_process_result", running_payload)
                .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));

            let status = child.wait().expect("Failed to wait on child");
            let exited_payload: serde_json::Value = serde_json::json!({
                "app_id": app_id.to_string(),
                "executable_name": executable_name.clone(),
                "full_executable_path": Path::new(&normalized_path_clone)
                    .join(&executable_name_clone)
                    .to_string_lossy()
                    .to_string(),
                "pid": pid,
                "status": status.code(),
                "running": false,
            });
            handle
                .emit("background_process_result", exited_payload)
                .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));
        }
        Err(e) => {
            // Emit running: false with error, no pid
            let error_payload = serde_json::json!({
                "app_id": app_id.to_string(),
                "executable_name": executable_name.clone(),
                "full_executable_path": Path::new(&normalized_path_clone)
                    .join(&executable_name_clone)
                    .to_string_lossy()
                    .to_string(),
                "pid": null,
                "running": false,
                "error": e.to_string(),
            });
            handle
                .emit("background_process_result", error_payload)
                .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));
        }
    }

    // match status.success() {
    //     true => Ok("Process started successfully".to_string()),
    //     false => Err(format!("Failed to start process. Process exited with status: {:?}", status.code()))
    // }
    // });

    // app.listen("stop_process", move |_| {
    //     println!("Disconnecting from Discord RPC...");
    //     task.abort();
    // });
    // match std::process::Command::new(&executable_path)
    //     .current_dir(game_folder_path) // Set working directory to the game folder
    //     .spawn()
    // {
    //     Ok(_) => Ok("Process started successfully".to_string()),
    //     Err(e) => Err(format!("Failed to start process: {}", e))
    // }
}

#[tauri::command(rename_all = "snake_case")]
async fn stop_process(exec_name: String) -> Result<(), String> {
    // Stop the process using taskkill command
    let output = std::process::Command::new("taskkill")
        .arg("/F")
        .arg("/IM")
        .arg(exec_name)
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
}

/// Usage: Calling from JS:
/// ```javascript
/// await invoke('connect_to_discord_rpc_3', json, 'connect' | 'disconnect');
#[tauri::command(rename_all = "snake_case")]
fn connect_to_discord_rpc_3(handle: AppHandle, activity_json: String, action: String) {
    let app = handle.clone();

    let event_connecting = "client_connecting";
    let event_connected = "client_connected";
    let event_disconnect = "event_disconnect";
    let event_connect = "event_connect";

    let activity = runner::parse_activity_json(&activity_json).unwrap();

    let connecting_payload = serde_json::json!({
        "app_id": activity.app_id,
    });

    let client_option = {
        let mut client_guard = get_discord_client().lock().unwrap();
        // Take the client out, leaving None in its place
        client_guard.take()
        // MutexGuard is dropped here at the end of scope
    };

    let task = tauri::async_runtime::spawn(async move {
        handle
            .emit(event_connecting, connecting_payload)
            .unwrap_or_else(|e| eprintln!("Failed to emit event: {}", e));

        let client = runner::set_activity(activity_json)
            .await
            .map_err(|e| {
                println!("Failed to set activity: {}", e);
            })
            .unwrap();

        let connected_payload = serde_json::json!({
            "app_id": activity.app_id,
        });

        {
            let mut client_guard = get_discord_client().lock().unwrap();
            *client_guard = Some(client);
        }

        handle
            .emit(event_connected, connected_payload)
            .unwrap_or_else(|e| {
                eprintln!("Failed to emit event: {}", e);
            });

        handle.listen(event_disconnect, move |_| {
            println!("Disconnecting from Discord RPC inner");
            let disconnect_task = tauri::async_runtime::spawn(async move {
                let client_option = {
                    let mut client_guard = get_discord_client().lock().unwrap();
                    // Take the client out, leaving None in its place
                    client_guard.take()
                    // MutexGuard is dropped here at the end of scope
                };
                if let Some(client) = client_option {
                    client.discord.disconnect().await;
                    println!("Disconnected from Discord RPC inner");
                }
            });
            // disconnect_task.abort();
        });
    });

    app.listen(event_disconnect, move |_| {
        println!("Disconnecting from Discord RPC...");
        task.abort();
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_fake_game,
            stop_process,
            connect_to_discord_rpc_3,
            run_background_process,
            commands::detectables::get_embedded_gamelist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
