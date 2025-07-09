#[tauri::command]
pub fn get_embedded_gamelist() -> String {
    let bytes = include_bytes!("../../data/gamelist.json");
    std::str::from_utf8(bytes).unwrap_or("{}").to_string()
}
