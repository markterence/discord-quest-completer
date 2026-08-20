// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use once_cell::sync::OnceCell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{path::BaseDirectory, AppHandle, Emitter, Listener, Manager};

mod rpc;
mod runner;
mod commands;
mod event;
mod process_kill;
mod platform;

// Global static instance of the Discord client
static DISCORD_CLIENT: OnceCell<Mutex<Option<rpc::Client>>> = OnceCell::new();

fn get_discord_client() -> &'static Mutex<Option<rpc::Client>> {
    DISCORD_CLIENT.get_or_init(|| Mutex::new(None))
}

fn runner_resource_name() -> &'static str {
    #[cfg(target_os = "windows")]
    let runner_name = "data/src-win.exe";

    #[cfg(target_os = "linux")]
    let runner_name = "data/src-linux";

    #[cfg(target_os = "macos")]
    let runner_name = "data/src-darwin";

    runner_name
}

fn is_app_bundle(executable_name: &str) -> bool {
    cfg!(target_os = "macos") && executable_name.ends_with(".app")
}

fn bundle_binary_name(app_name: &str) -> String {
    app_name
        .strip_suffix(".app")
        .unwrap_or(app_name)
        .to_string()
}

fn game_folder_path(exe_dir: &Path, path: &str, app_id: i64) -> PathBuf {
    let normalized_path = Path::new(path).to_string_lossy().to_string();

    exe_dir
        .join("games")
        .join(app_id.to_string())
        .join(normalized_path)
}

fn resolve_runner_template(handle: &AppHandle) -> Result<PathBuf, String> {
    handle
        .path()
        .resolve(runner_resource_name(), BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve runner template: {}", e))
}

fn make_macos_app_bundle(
    app_bundle_path: &Path,
    app_name: &str,
    display_name: &str,
    runner_template: &Path,
) -> Result<PathBuf, String> {
    let binary_name = bundle_binary_name(app_name);
    let macos_dir = app_bundle_path.join("Contents/MacOS");
    fs::create_dir_all(&macos_dir)
        .map_err(|e| format!("Failed to create app bundle directories: {}", e))?;

    let target_executable_path = macos_dir.join(&binary_name);
    fs::copy(runner_template, &target_executable_path)
        .map_err(|e| format!("Failed to copy dummy executable: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&target_executable_path)
            .map_err(|e| format!("Failed to read executable permissions: {}", e))?
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&target_executable_path, permissions)
            .map_err(|e| format!("Failed to set executable permissions: {}", e))?;
    }

    let info_plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDisplayName</key>
    <string>{display_name}</string>
    <key>CFBundleExecutable</key>
    <string>{binary_name}</string>
    <key>CFBundleIdentifier</key>
    <string>me.markterence.discordquestcompleter.dummy</string>
    <key>CFBundleName</key>
    <string>{display_name}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
</dict>
</plist>
"#
    );

    fs::write(app_bundle_path.join("Contents/Info.plist"), info_plist)
        .map_err(|e| format!("Failed to write Info.plist: {}", e))?;

    Ok(target_executable_path)
}

fn launch_executable_path(game_folder_path: &Path, executable_name: &str) -> PathBuf {
    if is_app_bundle(executable_name) {
        game_folder_path
            .join(executable_name)
            .join("Contents/MacOS")
            .join(bundle_binary_name(executable_name))
    } else {
        game_folder_path.join(executable_name)
    }
}

fn stop_process_name(exec_name: &str) -> String {
    if is_app_bundle(exec_name) {
        bundle_binary_name(exec_name)
    } else {
        exec_name.to_string()
    }
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
    display_name: Option<String>,
) -> Result<String, String> {
    let _path_len = path_len;
    // Must create in the same directory as the executable to avoid permission issues
    // Get the executable directory to look for config file
    let exe_path: std::path::PathBuf = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let game_folder_path = game_folder_path(exe_dir, path, app_id);

    println!("Game folder path: {:?}", game_folder_path);
    println!(
        "Game full path: {:?}",
        game_folder_path.join(executable_name)
    );

    match fs::create_dir_all(&game_folder_path) {
        Ok(_) => {
            println!("Successfully created directory: {:?}", game_folder_path);
        }
        Err(e) => return Err(format!("Failed to create game folder: {}", e)),
    };

    let resource_path = resolve_runner_template(&handle)?;
    println!("Creating dummy game executable from: {:?}", resource_path);

    if is_app_bundle(executable_name) {
        let app_bundle_path = game_folder_path.join(executable_name);
        let bundle_display_name = display_name
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| bundle_binary_name(executable_name));
        let target_executable_path = make_macos_app_bundle(
            &app_bundle_path,
            executable_name,
            &bundle_display_name,
            &resource_path,
        )?;
        return Ok(format!(
            "Dummy app bundle created at: {:?}",
            target_executable_path
        ));
    }

    let target_executable_path = game_folder_path.join(executable_name);
    match fs::copy(&resource_path, &target_executable_path) {
        Ok(_) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&target_executable_path)
                    .map_err(|e| format!("Failed to get file metadata: {}", e))?
                    .permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&target_executable_path, perms)
                    .map_err(|e| format!("Failed to set executable permissions: {}", e))?;
            }

            Ok(format!(
                "Dummy executable copied to: {:?}",
                target_executable_path
            ))
        }
        Err(e) => Err(format!("Failed to copy dummy executable: {}", e)),
    }
}

#[cfg(target_os = "macos")]
fn launch_macos_app_bundle(app_bundle_path: &Path, title: &str) -> Result<(), String> {
    let mut command = std::process::Command::new("open");
    command
        .arg("-n")
        .arg("-g")
        .arg("-a")
        .arg(app_bundle_path)
        .arg("--args")
        .arg("--title")
        .arg(title);

    command
        .spawn()
        .map_err(|e| format!("Failed to launch app bundle with open: {}", e))?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn run_background_process(
    name: &str,
    path: &str,
    executable_name: &str,
    path_len: i64,
    app_id: i64,
) -> Result<String, String> {
    let _path_len = path_len;
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let game_folder_path = game_folder_path(exe_dir, path, app_id);

    if is_app_bundle(executable_name) {
        #[cfg(target_os = "macos")]
        {
            let bundle_path = game_folder_path.join(executable_name);
            launch_macos_app_bundle(&bundle_path, name)?;
            return Ok("App bundle launched successfully".to_string());
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = name;
            return Err("App bundle launches are only supported on macOS".to_string());
        }
    }

    let executable_path = launch_executable_path(&game_folder_path, executable_name);
    
    let mut cmd = std::process::Command::new(&executable_path);
    cmd.args(["--title", name])
       .current_dir(game_folder_path);
    
    // Platform-specific process spawning
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0); // Create new process group on Unix
    }
    
    match cmd.spawn() {
        Ok(_) => Ok("Process started successfully".to_string()),
        Err(e) => Err(format!("Failed to start process: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn stop_process(exec_name: String) -> Result<(), String> {
    let process_name = stop_process_name(&exec_name);

    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg(&process_name)
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

    #[cfg(target_os = "macos")]
    {
        if is_app_bundle(&exec_name) {
            let bundle_pattern = format!("{}/Contents/MacOS", exec_name);
            let output = std::process::Command::new("pkill")
                .arg("-f")
                .arg(&bundle_pattern)
                .output()
                .map_err(|e| format!("Failed to execute pkill: {}", e))?;

            if output.status.success() {
                return Ok(());
            }
        }

        let output = std::process::Command::new("pkill")
            .arg("-x")
            .arg(&process_name)
            .output()
            .map_err(|e| format!("Failed to execute pkill: {}", e))?;

        if output.status.success() || output.status.code() == Some(1) {
            Ok(())
        } else {
            Err(format!(
                "Failed to stop process: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("pkill")
            .arg("-f")
            .arg(&exec_name)
            .output()
            .map_err(|e| format!("Failed to execute pkill: {}", e))?;

        if output.status.success() || output.status.code() == Some(1) {
            // pkill returns 1 if no processes were killed, which is fine
            Ok(())
        } else {
            Err(format!(
                "Failed to stop process: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

/// Usage: Calling from JS:
/// ```javascript
/// await invoke('connect_to_discord_rpc_3', json, 'connect' | 'disconnect');
#[tauri::command(rename_all = "snake_case")]
fn connect_to_discord_rpc_3(handle: AppHandle, activity_json: String, action: String) {
    let _action = action;
    let app = handle.clone();

    let event_connecting = "client_connecting";
    let event_connected = "client_connected";
    let event_disconnect = "event_disconnect";
    let _event_connect = "event_connect";

    let activity = runner::parse_activity_json(&activity_json).unwrap();

    let connecting_payload = serde_json::json!({
        "app_id": activity.app_id,
    });

    let _client_option = {
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
            let _disconnect_task = tauri::async_runtime::spawn(async move {
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .manage(commands::dispatcher::GameDispatcherState(Mutex::new(
                std::collections::HashMap::new(),
        )))
        .invoke_handler(tauri::generate_handler![
            greet,
            create_fake_game,
            stop_process,
            connect_to_discord_rpc_3,
            run_background_process,
            commands::api::fetch_gamelist_gh_mirror,
            commands::api::fetch_gamelist_from_discord,
            commands::dispatcher::launch_executable,
            commands::dispatcher::stop_executable
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}