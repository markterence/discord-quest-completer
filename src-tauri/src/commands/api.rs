use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::Response;
use tauri_plugin_http::reqwest::Error as ReqwestError;
use tauri::ipc::Response as IpcResponse;

const DEV_FLAG_TEST_ERROR: bool = false;


#[tauri::command(rename_all = "snake_case")]
pub async fn fetch_gamelist_gh_mirror() -> Result<IpcResponse, String> {
    // if else for some development flags to test error handling without making an actual network request
    if DEV_FLAG_TEST_ERROR {
        fetch_sample_dns_error().await?;
    }

    let res: Response = reqwest::get("https://markterence.github.io/discord-quest-completer/detectable.json")
        .await
        .map_err(|err| err.to_string())?;

    Ok(IpcResponse::new(res.text().await.unwrap()))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn fetch_gamelist_from_discord() -> Result<IpcResponse, String> {
    let res: Response = reqwest::get("https://discord.com/api/applications/detectable")
        .await
        .map_err(|err: ReqwestError| err.to_string())?;

    Ok(IpcResponse::new(res.text().await.unwrap()))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn fetch_sample_dns_error() -> Result<IpcResponse, String> {
    Ok(Err("test error".to_string())?) 
}
