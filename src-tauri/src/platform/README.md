# Platform resolvers

> **Note:** this design (multiple resolvers per
> platform/distributor, selected via an `ACTIVE_*_RESOLVER` constant) is a tentative
> decision, not a settled architecture. It may be revisited.

A `PlatformResolver` maps `(app_id, path)` to the folder a game's executable
lives in. `launch_executable` calls `platform::resolve_game_folder(...)`
instead of hardcoding a folder layout, so each storefront/platform can have
its own resolution strategy.

`Standalone` (the app's own `./games/<app_id>/<path>` folder) has exactly one
resolver. Other platforms — currently `Steam` — may end up with **more than
one candidate resolver** (e.g. registry-based vs `libraryfolders.vdf`-based),
where only one is *active* at a time. The rule: adding a new candidate never
requires editing an existing resolver's file.

## Adding a new resolver for an existing platform (e.g. a second Steam resolver)

1. Create a new file, e.g. `steam_vdf.rs`, with its own struct implementing
   `PlatformResolver`:

   ```rust
   use std::path::PathBuf;
   use super::PlatformResolver;

   pub struct SteamVdfResolver;

   impl PlatformResolver for SteamVdfResolver {
       fn name(&self) -> &'static str {
           "steam_vdf"
       }

       fn resolve_game_folder(&self, app_id: i64, path: &str) -> Result<PathBuf, String> {
           // ... parse libraryfolders.vdf, find installdir for app_id, join `path` ...
       }
   }
   ```

   `name()` must be unique among a platform's candidates — it's how `mod.rs`
   picks which one is active.

2. In `mod.rs`:
   - Add `mod steam_vdf;` near the other `mod` declarations.
   - Add it to that platform's list in `candidates()`:
     ```rust
     Platform::Steam => vec![
         Box::new(steam::SteamStubResolver),
         Box::new(steam_vdf::SteamVdfResolver),
     ],
     ```
   - To make it the active one, point the platform's `ACTIVE_*_RESOLVER`
     constant at its `name()`:
     ```rust
     const ACTIVE_STEAM_RESOLVER: &str = "steam_vdf";
     ```

Do not edit any other resolver file to do this — `steam.rs` (or any other
existing candidate) stays untouched. Only `mod.rs` (the registry) and your new
file change.

## Adding a resolver for a brand new platform

1. Add the variant to the `Platform` enum in `mod.rs` (this is also the
   string the frontend sends via `platform: "..."`, matched via
   `#[serde(rename_all = "snake_case")]`):

   ```rust
   pub enum Platform {
       #[default]
       Standalone,
       Steam,
       Epic, // new
   }
   ```

2. Create `epic.rs` with a resolver, same shape as above.

3. In `mod.rs`, add a `mod epic;` declaration, an arm in `candidates()`
   (`Platform::Epic => vec![Box::new(epic::EpicResolver)]`), and an arm in
   `resolve_game_folder`'s `active_name` match. If the platform only ever has
   one resolver, its name can be used directly instead of an
   `ACTIVE_*_RESOLVER` constant (see how `Platform::Standalone` is handled).

## Notes

- Resolvers only resolve a folder — they don't touch the process spawn/kill
  logic in `commands/dispatcher.rs`, which is platform-agnostic.
- A resolver can fail (e.g. Steam library not found); return `Err(String)`
  and it propagates straight back as the `launch_executable` command's error.
  See `steam.rs`'s `SteamStubResolver` for the minimal shape of a
  not-yet-implemented resolver.
