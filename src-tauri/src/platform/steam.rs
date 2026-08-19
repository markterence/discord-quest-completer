use std::path::PathBuf;

use super::PlatformResolver;

// TODO: Steam library resolution using registry since Steam game folder and its path cant be spoofed?
pub struct SteamStubResolver;

impl PlatformResolver for SteamStubResolver {
    fn name(&self) -> &'static str {
        "steam_stub"
    }

    fn resolve_game_folder(&self, _app_id: i64, _path: &str) -> Result<PathBuf, String> {
        Err("Steam platform resolver not yet implemented".to_string())
    }
}
