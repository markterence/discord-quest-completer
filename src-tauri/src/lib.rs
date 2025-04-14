// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::path::Path;
use std::time::SystemTime;
use std::sync::Mutex;
use once_cell::sync::OnceCell;

mod rpc;

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
async fn connect_to_discord_rpc(app_id: String, discord_state: String) -> Result<(), String> {
    println!("state is: {}", discord_state);
    // convert string to i64
    let app_id_i64: i64 = app_id.parse().map_err(|e| {
        format!("Failed to parse app ID: {}", e)
    })?;
     
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
        } else {
            println!("No Discord client to disconnect");
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_fake_game, 
            connect_to_discord_rpc,
            stop_process,
            run_background_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
