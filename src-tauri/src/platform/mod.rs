use std::path::PathBuf;

mod standalone;
mod steam;

pub trait PlatformResolver {
    fn name(&self) -> &'static str;
    fn resolve_game_folder(&self, app_id: i64, path: &str) -> Result<PathBuf, String>;
}

#[derive(serde::Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    #[default]
    Standalone,
    Steam,
}

fn candidates(platform: &Platform) -> Vec<Box<dyn PlatformResolver>> {
    match platform {
        Platform::Standalone => vec![Box::new(standalone::StandaloneResolver)],
        Platform::Steam => vec![Box::new(steam::SteamStubResolver)],
    }
}

// Change this to switch which resolver is active for a platform with multiple candidates.
const ACTIVE_STEAM_RESOLVER: &str = "steam_stub";

pub fn resolve_game_folder(platform: &Platform, app_id: i64, path: &str) -> Result<PathBuf, String> {
    let active_name = match platform {
        Platform::Standalone => "standalone",
        Platform::Steam => ACTIVE_STEAM_RESOLVER,
    };
    candidates(platform)
        .into_iter()
        .find(|r| r.name() == active_name)
        .ok_or_else(|| format!("No resolver named '{}' registered for this platform", active_name))?
        .resolve_game_folder(app_id, path)
}
