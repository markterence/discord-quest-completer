pub const EVT_BACKGROUND_PROCESS_RESULT: &str = "background_process_result";
 
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExeProcessPayload {
    pub app_id: String,
    pub executable_name: String,
    pub full_executable_path: String,
    pub pid: Option<u32>,
    pub running: bool,
    pub status: Option<i32>, 
    pub error: Option<String>,
} 