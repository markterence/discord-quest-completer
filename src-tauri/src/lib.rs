// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// import path 
use std::env;
use std::path::Path;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_fake_game_folder(path: &str, executable_name: &str) -> Result<String, String> {
    // Must create in the same directory as the executable to avoid permission issues
    // Get the executable directory to look for config file
    let exe_path = env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("")); 
    let game_folder_path = exe_dir.join("games").join(path);

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
        Ok(_) => Ok(format!("Dummy executable copied to: {:?}", target_executable_path)),
        Err(e) => Err(format!("Failed to copy dummy executable: {}", e))
    }
}

#[tauri::command]
async fn run_background_process(path_to_executable: &str) -> Result<String, String> {
    match std::process::Command::new(path_to_executable)
        .spawn()
    {
        Ok(_) => Ok("Process started successfully".to_string()),
        Err(e) => Err(format!("Failed to start process: {}", e))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_fake_game_folder, 
            run_background_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
