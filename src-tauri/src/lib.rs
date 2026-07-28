// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use once_cell::sync::OnceCell;
use std::env;
use std::path::Path;
use std::sync::Mutex;
use tauri::{path::BaseDirectory, AppHandle, Emitter, Listener, Manager};

mod rpc;
mod runner;

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
    name: &str,
    path: &str,
    executable_name: &str,
    path_len: i64,
    app_id: i64,
) -> Result<String, String> {
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let normalized_path = Path::new(path).to_string_lossy().to_string();

    let game_folder_path = exe_dir
        .join("games")
        .join(app_id.to_string())
        .join(normalized_path);
    let executable_path = game_folder_path.join(executable_name);
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;

    let mut cmd = std::process::Command::new(&executable_path);
    cmd.args(["--title", name, "--tray"])
       .current_dir(game_folder_path);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    match cmd.spawn() {
        Ok(_) => Ok("Process started successfully".to_string()),
        Err(e) => Err(format!("Failed to start process: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn create_steam_appmanifest(
    steam_appid: &str,
    name: &str,
) -> Result<String, String> {
    let mut possible_paths = Vec::new();

    if let Ok(program_files_x86) = env::var("ProgramFiles(x86)") {
        possible_paths.push(std::path::PathBuf::from(program_files_x86).join("Steam").join("steamapps"));
    }
    if let Ok(program_files) = env::var("ProgramFiles") {
        possible_paths.push(std::path::PathBuf::from(program_files).join("Steam").join("steamapps"));
    }
    possible_paths.push(std::path::PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps"));
    possible_paths.push(std::path::PathBuf::from("C:\\Steam\\steamapps"));
    possible_paths.push(std::path::PathBuf::from("D:\\Steam\\steamapps"));
    possible_paths.push(std::path::PathBuf::from("E:\\Steam\\steamapps"));

    let content = format!(
        r#""AppState"
{{
	"appid"		"{}"
	"Universe"		"1"
	"name"		"{}"
	"StateFlags"		"4"
	"installdir"		"{}"
}}
"#,
        steam_appid, name, name
    );

    let mut created = false;
    let mut last_err = String::new();

    for path in possible_paths {
        if path.exists() {
            let manifest_path = path.join(format!("appmanifest_{}.acf", steam_appid));
            match std::fs::write(&manifest_path, &content) {
                Ok(_) => {
                    created = true;
                    println!("Successfully created Steam manifest: {:?}", manifest_path);
                }
                Err(e) => {
                    last_err = e.to_string();
                }
            }
        }
    }

    if created {
        Ok("Steam manifest created successfully".to_string())
    } else {
        Err(format!("Could not create Steam manifest file. Error: {}", last_err))
    }
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

#[tauri::command(rename_all = "snake_case")]
async fn fetch_gamelist_gh_mirror() -> tauri::ipc::Response {
    let res = tauri_plugin_http::reqwest::get("https://markterence.github.io/discord-quest-completer/detectable.json").await;
    tauri::ipc::Response::new(res.unwrap().text().await.unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn fetch_gamelist_from_discord() -> tauri::ipc::Response {
    let res = tauri_plugin_http::reqwest::get("https://discord.com/api/applications/detectable").await;
    tauri::ipc::Response::new(res.unwrap().text().await.unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn fetch_user_quests(token: String) -> Result<String, String> {
    let client = tauri_plugin_http::reqwest::Client::new();
    let res = client
        .get("https://discord.com/api/v10/users/@me/quests")
        .header("Authorization", token.trim())
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Discord API error status: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    Ok(text)
}

#[tauri::command(rename_all = "snake_case")]
async fn open_discord_login_window(handle: AppHandle) -> Result<(), String> {
    if let Some(win) = handle.get_webview_window("discord_login") {
        let _ = win.set_focus();
        return Ok(());
    }

    let init_script = r#"
        (function() {
            function checkToken() {
                try {
                    let token = null;
                    if (window.webpackChunkdiscord_app) {
                        window.webpackChunkdiscord_app.push([[Math.random()],{},e=>{
                            for(const c of Object.values(e.c)) {
                                if(c?.exports?.default?.getToken) {
                                    let t = c.exports.default.getToken();
                                    if (t && typeof t === 'string' && t.length > 20) {
                                        token = t;
                                    }
                                }
                            }
                        }]);
                    }
                    if (!token && window.localStorage) {
                        for (let i = 0; i < window.localStorage.length; i++) {
                            let key = window.localStorage.key(i);
                            if (key && key.includes('token')) {
                                let val = window.localStorage.getItem(key);
                                if (val) {
                                    let cleaned = val.replace(/^"|"$/g, '');
                                    if (cleaned.length > 20) {
                                        token = cleaned;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if (token) {
                        window.__TAURI_INTERNALS__.invoke('token_captured_internal', { token: token });
                    }
                } catch(e) {}
            }
            setInterval(checkToken, 1500);
        })();
    "#;

    let window_res = tauri::WebviewWindowBuilder::new(
        &handle,
        "discord_login",
        tauri::WebviewUrl::External("https://discord.com/login".parse().map_err(|e| format!("{}", e))?),
    )
    .title("Log in with Discord")
    .inner_size(850.0, 650.0)
    .initialization_script(init_script)
    .build();

    match window_res {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open login window: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn token_captured_internal(handle: AppHandle, token: String) -> Result<(), String> {
    let payload = serde_json::json!({
        "token": token
    });
    let _ = handle.emit("discord_token_captured", payload);
    if let Some(win) = handle.get_webview_window("discord_login") {
        let _ = win.close();
    }
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_fake_game,
            stop_process,
            connect_to_discord_rpc_3,
            run_background_process,
            fetch_gamelist_gh_mirror,
            fetch_gamelist_from_discord,
            create_steam_appmanifest,
            fetch_user_quests,
            open_discord_login_window,
            token_captured_internal
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
