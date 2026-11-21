# macOS Intel Build Design

**Date:** 2026-11-21

## Intent
Add a macOS Intel release build to the existing GitHub Actions workflow so the project publishes a Tauri macOS artifact alongside the Windows artifact.

## Goals
- Run the macOS build on an Intel runner (`macos-13`) and target `x86_64-apple-darwin`.
- Build and copy the existing `src-darwin` runner into Tauri resources.
- Produce Tauri macOS bundle artifacts (`.dmg` and `.app`) and upload them for the release job.
- Keep the current Windows build behavior unchanged.

## Non-goals
- Apple code signing or notarization.
- Apple Silicon (`aarch64`) builds.
- Changing the application runtime or macOS runner implementation.

## Constraints
- The workflow is manually dispatched and currently has separate build and release jobs.
- `src-tauri/tauri.macos.conf.json` already maps the macOS runner resource to `data/src-darwin`.
- Tauri bundle resources already include `resources/*`.
- The Intel runner must remain explicit so the artifact architecture is predictable.

## Approaches
1. **Recommended — add a dedicated macOS build job:** build `src-darwin`, copy it into `src-tauri/resources`, run `tauri-action` with the macOS config, and upload the bundle output. This isolates platform-specific setup and preserves the Windows job.
2. Expand the existing matrix to include Windows and macOS. This reduces duplicated YAML but requires more conditionals for runner build commands, artifact paths, and release naming; it is less clear for the current workflow.
3. Build macOS locally and upload a prebuilt artifact. This does not provide reproducible CI builds and is rejected.

## Design
Add a `build-tauri-macos` job on `macos-13`, install pnpm/Node/Rust, download the game list, run `pnpm install`, build `src-darwin` for the native Intel target, copy `src-darwin` to `src-tauri/resources/src-darwin`, and invoke Tauri with `--config src-tauri/tauri.macos.conf.json`. Upload `src-tauri/target/x86_64-apple-darwin/release/bundle/` as a macOS artifact. Make the release job depend on both platform jobs, download both artifacts, and attach the macOS bundle files to the GitHub release without changing the existing Windows ZIP packaging.

The remaining implementation call is the exact release asset naming and copy layout; use architecture-explicit names such as `discord-quest-completer-macos-x64-<version>`.
