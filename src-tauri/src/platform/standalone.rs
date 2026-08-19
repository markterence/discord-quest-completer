use std::env;
use std::path::{Path, PathBuf};

use super::PlatformResolver;

pub struct StandaloneResolver;

impl PlatformResolver for StandaloneResolver {
    fn name(&self) -> &'static str {
        "standalone"
    }

    fn resolve_game_folder(&self, app_id: i64, path: &str) -> Result<PathBuf, String> {
        let exe_path = env::current_exe().unwrap_or_default();
        let exe_dir: &Path = exe_path.parent().unwrap_or_else(|| Path::new(""));

        Ok(exe_dir.join("games").join(app_id.to_string()).join(path))
    }
}
