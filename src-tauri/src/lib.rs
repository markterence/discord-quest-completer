// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::fmt::format;
use std::path::Path;
use std::time::SystemTime;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tauri::{async_runtime::{block_on}, AppHandle, Emitter, Listener};
use tokio::sync::oneshot::channel;


mod rpc;
mod runner;

#[derive(Deserialize)]
struct WatchingActivityParams {
    app_id: String,
    details: Option<String>,
    state: Option<String>,
    #[serde(rename = "largeImageKey")]
    large_image_key: Option<String>,
    #[serde(rename = "largeImageText")]
    large_image_text: Option<String>,
    timestamp: Option<i64>,
    activity_kind: Option<i32>,
}

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
async fn create_fake_game(path: &str, executable_name: &str, path_len: i64, app_id: i64) -> Result<String, String> {
    // Must create in the same directory as the executable to avoid permission issues
    // Get the executable directory to look for config file
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let normalized_path = Path::new(path).to_string_lossy().to_string();

    let game_folder_path = exe_dir.join("games").join(app_id.to_string()).join(normalized_path);

    println!("Game folder path: {:?}", game_folder_path);
    println!("Game full path: {:?}", game_folder_path.join(executable_name));

    // Ok(format!("Dummy executable copied to: {:?}", target_executable_path))
    match std::fs::create_dir_all(&game_folder_path) {
        Ok(_) => {
            println!("Successfully created directory: {:?}", game_folder_path);
        },
        Err(e) => return Err(format!("Failed to create game folder: {}", e))
    };
    // copy the dummy executable to the created folder
    // there is a `template.exe` file along the final build. 
    let dummy_executable_path = exe_dir.join("template.exe");
    let target_executable_path = game_folder_path.join(executable_name);
    match std::fs::copy(&dummy_executable_path, &target_executable_path) {
        Ok(_) => {
            Ok(format!("Dummy executable copied to: {:?}", target_executable_path))
        },
        Err(e) => Err(format!("Failed to copy dummy executable: {}", e))
    }
    
}

#[tauri::command(rename_all = "snake_case")]
async fn run_background_process(path: &str, executable_name: &str, path_len: i64, app_id: i64)-> Result<String, String> {
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(""));

    let normalized_path = Path::new(path).to_string_lossy().to_string();

    let game_folder_path = exe_dir.join("games").join(app_id.to_string()).join(normalized_path);
    let executable_path = game_folder_path.join(executable_name);
    match std::process::Command::new(&executable_path)
        .current_dir(game_folder_path) // Set working directory to the game folder
        .arg("--foreground") // Optional: pass a foreground flag if your app supports it
        .spawn()
    {
        Ok(_) => Ok("Process started successfully".to_string()),
        Err(e) => Err(format!("Failed to start process: {}", e))
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
        Err(format!("Failed to stop process: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn set_activity(activity_json: String, state: String) -> Result<(), String> {
    // Parse the JSON string into our struct
    let params: WatchingActivityParams = serde_json::from_str(&activity_json)
        .map_err(|e| format!("Failed to parse activity JSON: {}", e))?;
    
    let app_id_i64: i64 = params.app_id.parse().map_err(|e| {
        format!("Failed to parse app ID: {}", e)
    })?;
    println!("activity_json is: {}", activity_json);
    println!("state is: {}", state);

    // if state is disconnect then disconnect the client and return
    if state == "disconnect" {
        let client_option = {
            let mut client_guard = get_discord_client().lock().unwrap();
            // Take the client out, leaving None in its place
            client_guard.take()
            // MutexGuard is dropped here at the end of scope
        };
        
        // Now use the client without holding the MutexGuard
        if let Some(client) = client_option {
            client.discord.disconnect().await;
        } else {
            println!("No Discord client to disconnect");
        }
        
        return Ok(());
    }

    let client = rpc::make_client(app_id_i64, rpc::ds::Subscriptions::ACTIVITY).await;

    // Build the activity with optional fields from JSON
    let mut rp: discord_sdk::activity::ActivityBuilder = rpc::ds::activity::ActivityBuilder::default();

    if let Some(activity_kind) = params.activity_kind {
        if activity_kind == 0 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Playing);
        } else if activity_kind == 2 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Listening);
        } else if activity_kind == 3 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Watching);
        } else if activity_kind == 5 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Competing);
        }
    } else {
        rp = rp.kind(rpc::ds::activity::ActivityKind::Playing);
    }
    
    let details = params.details.clone().unwrap_or_default();
    if !details.is_empty() {
        print!("Details is: {}", details);
        rp = rp.details(&details);
    }
    if let Some(state) = params.state {
        rp = rp.state(&state);
    }

    if let Some(timestamp) = params.timestamp {
        rp = rp.start_timestamp(
            timestamp as i64
        );
    }

    if let Some(large_image_key) = params.large_image_key {
       
        rp = rp.assets(
            rpc::ds::activity::Assets::default()
            .large(&large_image_key, params.large_image_text.as_deref())
        );
    }
    
    client.discord.update_activity(rp).await.map_err(|e| {
        format!("Failed to update activity: {}", e)
    })?;

    {
        let mut client_guard = get_discord_client().lock().unwrap();
        *client_guard = Some(client);
    }
    
    Ok(())
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

    // if let Some(client) = client_option {
    //     client.discord.disconnect().await;
    //     println!("Disconnected since we have an existing client"); 
    // }
 
 
    let task = tauri::async_runtime::spawn(async move {
        handle.emit(event_connecting, connecting_payload).unwrap_or_else(|e| {
            eprintln!("Failed to emit event: {}", e)
        });

        let client = runner::set_activity(activity_json).await.map_err(|e| {
            println!("Failed to set activity: {}", e);
        }).unwrap(); 
        
        let connected_payload = serde_json::json!({
            "app_id": activity.app_id, 
        });
        
        {
            let mut client_guard = get_discord_client().lock().unwrap();
            *client_guard = Some(client);
        }

        handle.emit(event_connected, connected_payload).unwrap_or_else(|e| {
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
                let _ = tx.send(());
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
async fn connect_to_discord_rpc_2(app_id: String, discord_state: String) -> Result<String, String> {
    println!("state is: {}", discord_state);
    // convert string to i64
    let app_id_i64: i64 = app_id.parse().map_err(|e| {
        format!("Failed to parse app ID: {}", e)
    })?;

    // Handle disconnect request
    if discord_state == "disconnect" {
        let client_option = {
            let mut client_guard = get_discord_client().lock().unwrap();
            // Take the client out, leaving None in its place
            client_guard.take()
        };
        
        if let Some(client) = client_option {
            client.discord.disconnect().await;
            println!("Disconnected from Discord RPC");
            return Ok("Disconnected from Discord RPC".to_string());
        } else {
            println!("No Discord client to disconnect"); 
            return Ok("No Discord client to disconnect".to_string());
        }
    }
     
    // Handle connect request
    if discord_state == "connect" {
        // First check if we already have an active client and disconnect it
        let has_existing_client = {
            let client_guard = get_discord_client().lock().unwrap();
            client_guard.is_some()
        };
        
        // If we have an existing client, disconnect it first
        if has_existing_client {
            let client_option = {
                let mut client_guard = get_discord_client().lock().unwrap();
                client_guard.take()
            };
            
            if let Some(client) = client_option {
                println!("Disconnecting existing Discord RPC client before connecting with new app ID");
                client.discord.disconnect().await;
            }
        }
        
        // Create a new client with the new app ID
        println!("Connecting to Discord RPC with app ID: {}", app_id);
        let client = rpc::make_client(app_id_i64, rpc::ds::Subscriptions::ACTIVITY).await;
        
        // Set initial activity
        let rp = rpc::ds::activity::ActivityBuilder::default()
            .kind(rpc::ds::activity::ActivityKind::Playing)
            .start_timestamp(SystemTime::now());
        
        client.discord.update_activity(rp).await.map_err(|e| {
            format!("Failed to update activity: {}", e)
        })?;
    
        // Store the new client
        {
            let mut client_guard = get_discord_client().lock().unwrap();
            *client_guard = Some(client);
        }
        
        return Ok(format!("Connected to Discord RPC with app ID: {}", app_id));
    }

    Err("Invalid discord_state parameter. Must be 'connect' or 'disconnect'".to_string())
}

#[tauri::command(rename_all = "snake_case")]
async fn connect_to_discord_rpc(app_id: String, discord_state: String) -> Result<String, String> {
    println!("state is: {}", discord_state);
    // convert string to i64
    let app_id_i64: i64 = app_id.parse().map_err(|e| {
        format!("Failed to parse app ID: {}", e)
    })?;

    let client_option = {
        let mut client_guard = get_discord_client().lock().unwrap();
        // Take the client out, leaving None in its place
        client_guard.take()
        // MutexGuard is dropped here at the end of scope
    };
    
    // Now use the client without holding the MutexGuard
    if let Some(client) = client_option {
        client.discord.clear_activity().await;
        println!("Disconnected from Discord RPC");
        return Ok(format!("Disconnected from Discord RPC"));
    }
     
    if discord_state == "connect" {
        let client = rpc::make_client(app_id_i64, rpc::ds::Subscriptions::ACTIVITY).await;
        
        println!("Updating Activity with app ID: {}", app_id);
        let rp = rpc::ds::activity::ActivityBuilder::default()
            .kind(rpc::ds::activity::ActivityKind::Playing)
            .start_timestamp(SystemTime::now());
        
        // Update activity directly (not in a task)
        client.discord.update_activity(rp).await.map_err(|e| {
            format!("Failed to update activity: {}", e)
        })?;
    
        // IMPORTANT: Create a new scope to limit the MutexGuard's lifetime
        {
            let mut client_guard = get_discord_client().lock().unwrap();
            *client_guard = Some(client);
        }
        // No MutexGuard in the async scope
    }

    if discord_state == "disconnect" {
        // Create a new option to store the client temporarily
        let client_option = {
            let mut client_guard = get_discord_client().lock().unwrap();
            // Take the client out, leaving None in its place
            client_guard.take()
            // MutexGuard is dropped here at the end of scope
        };
        
        // Now use the client without holding the MutexGuard
        if let Some(client) = client_option {
            client.discord.disconnect().await;
            println!("Disconnected from Discord RPC");
            return Ok(format!("Disconnected from Discord RPC"));
        } else {
            println!("No Discord client to disconnect"); 
            return Ok(format!("No Discord client to disconnect"));
        }
    }

    Ok(format!("Connected to Discord RPC with app ID: {}", app_id))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_fake_game, 
            connect_to_discord_rpc,
            set_activity,
            stop_process,
            connect_to_discord_rpc_2,
            connect_to_discord_rpc_3,
            run_background_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
