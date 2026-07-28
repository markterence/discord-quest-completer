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
    let token_clean = token.trim().trim_matches('"');
    let mut res = client
        .get("https://discord.com/api/v9/quests/@me")
        .header("Authorization", token_clean)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if res.status().as_u16() == 404 {
        res = client
            .get("https://discord.com/api/v10/quests/@me")
            .header("Authorization", token_clean)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;
    }

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


use regex::Regex;
use base64::Engine;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

#[cfg(target_os = "windows")]
fn unprotect_data(data: &[u8]) -> Option<Vec<u8>> {
    use std::ptr::null_mut;
    #[repr(C)]
    struct DATA_BLOB {
        cbData: u32,
        pbData: *mut u8,
    }
    extern "system" {
        fn CryptUnprotectData(
            pDataIn: *mut DATA_BLOB,
            ppszDataDescr: *mut *mut u16,
            pOptionalEntropy: *mut DATA_BLOB,
            pvReserved: *mut std::ffi::c_void,
            pPromptStruct: *mut std::ffi::c_void,
            dwFlags: u32,
            pDataOut: *mut DATA_BLOB,
        ) -> i32;
        fn LocalFree(hMem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    }
    let mut in_blob = DATA_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };
    let mut out_blob = DATA_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };
    let res = unsafe {
        CryptUnprotectData(
            &mut in_blob,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut out_blob,
        )
    };
    if res != 0 && !out_blob.pbData.is_null() {
        let slice = unsafe { std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize) };
        let vec = slice.to_vec();
        unsafe { LocalFree(out_blob.pbData as *mut _) };
        Some(vec)
    } else {
        None
    }
}

fn get_discord_master_key(discord_dir: &Path) -> Option<Vec<u8>> {
    let local_state_path = discord_dir.join("Local State");
    if !local_state_path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(local_state_path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    let encrypted_key_b64 = json.get("os_crypt")?.get("encrypted_key")?.as_str()?;
    let encrypted_key = base64::engine::general_purpose::STANDARD.decode(encrypted_key_b64).ok()?;
    
    if encrypted_key.starts_with(b"DPAPI") {
        #[cfg(target_os = "windows")]
        {
            return unprotect_data(&encrypted_key[5..]);
        }
    }
    None
}

fn decrypt_discord_token(master_key: &[u8], encrypted_token_b64: &str) -> Option<String> {
    let encrypted_bytes = base64::engine::general_purpose::STANDARD.decode(encrypted_token_b64).ok()?;
    if encrypted_bytes.len() < 31 || &encrypted_bytes[0..3] != b"DPAPI" {
        return None;
    }
    let nonce = &encrypted_bytes[3..15];
    let ciphertext = &encrypted_bytes[15..];
    
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    let nonce_arr = Nonce::from_slice(nonce);
    
    let decrypted_bytes = cipher.decrypt(nonce_arr, ciphertext).ok()?;
    String::from_utf8(decrypted_bytes).ok()
}

fn extract_tokens_from_text(text: &str, master_key: Option<&[u8]>, encrypted_tokens: &mut Vec<String>, plain_tokens: &mut Vec<String>) {
    // 1. Decrypt dQw4w9WgXcQ: encrypted tokens first
    if let Some(key) = master_key {
        if let Ok(re_enc) = Regex::new(r"dQw4w9WgXcQ:([A-Za-z0-9+/=]+)") {
            for cap in re_enc.captures_iter(text) {
                if let Some(enc_b64) = cap.get(1) {
                    if let Some(decrypted) = decrypt_discord_token(key, enc_b64.as_str()) {
                        if !encrypted_tokens.contains(&decrypted) {
                            encrypted_tokens.push(decrypted);
                        }
                    }
                }
            }
        }
    }

    // 2. Plain text tokens
    let re_mfa = match Regex::new(r"mfa\.[a-zA-Z0-9_-]{84}") {
        Ok(r) => r,
        Err(_) => return,
    };
    let re_normal = match Regex::new(r"[a-zA-Z0-9_-]{24,32}\.[a-zA-Z0-9_-]{6}\.[a-zA-Z0-9_-]{25,45}") {
        Ok(r) => r,
        Err(_) => return,
    };

    for mat in re_mfa.find_iter(text) {
        let token = mat.as_str().to_string();
        if !plain_tokens.contains(&token) {
            plain_tokens.push(token);
        }
    }
    for mat in re_normal.find_iter(text) {
        let token = mat.as_str().to_string();
        if !plain_tokens.contains(&token) {
            plain_tokens.push(token);
        }
    }
}

fn scan_dir_for_tokens(dir: &Path, master_key: Option<&[u8]>, candidates: &mut Vec<String>) {
    if !dir.exists() {
        return;
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut files: Vec<(std::time::SystemTime, std::path::PathBuf)> = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "log" || ext == "ldb" {
                    let modified = entry.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    files.push((modified, path));
                }
            }
        }
        // Sắp xếp file theo thời gian chỉnh sửa giảm dần (file mới nhất xử lý trước)
        files.sort_by(|a, b| b.0.cmp(&a.0));

        let mut encrypted_tokens = Vec::new();
        let mut plain_tokens = Vec::new();

        for (_, path) in files {
            if let Ok(bytes) = std::fs::read(&path) {
                let text = String::from_utf8_lossy(&bytes);
                extract_tokens_from_text(&text, master_key, &mut encrypted_tokens, &mut plain_tokens);
            }
        }

        // Ưu tiên 100% token mã hóa DPAPI của tài khoản active hiện tại trước!
        for t in encrypted_tokens {
            if !candidates.contains(&t) {
                candidates.push(t);
            }
        }
        for t in plain_tokens {
            if !candidates.contains(&t) {
                candidates.push(t);
            }
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn auto_detect_discord_token() -> Result<String, String> {
    let mut candidates = Vec::new();

    if let Ok(appdata) = env::var("APPDATA") {
        let root = Path::new(&appdata);
        
        let discord_dir = root.join("discord");
        let master_key = get_discord_master_key(&discord_dir);
        scan_dir_for_tokens(&discord_dir.join("Local Storage").join("leveldb"), master_key.as_deref(), &mut candidates);

        let canary_dir = root.join("discordcanary");
        let canary_key = get_discord_master_key(&canary_dir);
        scan_dir_for_tokens(&canary_dir.join("Local Storage").join("leveldb"), canary_key.as_deref(), &mut candidates);

        let ptb_dir = root.join("discordptb");
        let ptb_key = get_discord_master_key(&ptb_dir);
        scan_dir_for_tokens(&ptb_dir.join("Local Storage").join("leveldb"), ptb_key.as_deref(), &mut candidates);
    }

    let client = tauri_plugin_http::reqwest::Client::new();

    for token in candidates {
        let res = client
            .get("https://discord.com/api/v10/users/@me")
            .header("Authorization", &token)
            .send()
            .await;

        if let Ok(response) = res {
            if response.status().is_success() {
                return Ok(token);
            }
        }
    }

    Err("Không tìm thấy tài khoản Discord đang hoạt động trên máy tính.".to_string())
}

#[tauri::command(rename_all = "snake_case")]
async fn open_default_browser(url: String) -> Result<(), String> {
    tauri_plugin_opener::open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open browser: {}", e))
}


#[tauri::command(rename_all = "snake_case")]
async fn fetch_user_profile(token: String) -> Result<String, String> {
    let client = tauri_plugin_http::reqwest::Client::new();
    let token_clean = token.trim().trim_matches('"');
    let res = client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", token_clean)
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
            token_captured_internal,
            open_default_browser,
            auto_detect_discord_token,
            fetch_user_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
