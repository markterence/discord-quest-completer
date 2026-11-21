# macOS Intel Build Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the manual GitHub Actions release workflow with a reproducible macOS Intel Tauri build and publish its DMG and zipped app bundle alongside the existing Windows release asset.

**Architecture:** Keep the existing Windows job unchanged and add a dedicated `build-tauri-macos` job on `macos-13`. The macOS job explicitly builds the `src-darwin` runner and Tauri target `x86_64-apple-darwin`, uploads the generated bundle as an artifact, and makes the release job depend on both platform jobs. The release job stages the macOS `.dmg` and `.app` as separately named assets without changing the Windows ZIP layout.

**Tech Stack:** GitHub Actions YAML, `pnpm`, Rust/Cargo, Tauri CLI/action, macOS `ditto`.

**Spec:** `docs/superpowers/specs/2026-11-21-macos-intel-build-design.md`

## Global Constraints

- Run the macOS build on `macos-13`.
- Target `x86_64-apple-darwin`; do not add Apple Silicon builds.
- Build and copy `src-darwin` to `src-tauri/resources/src-darwin` before the Tauri build.
- Use `src-tauri/tauri.macos.conf.json` for the macOS Tauri build.
- Preserve the existing Windows build and Windows ZIP packaging behavior.
- Do not add Apple code signing or notarization.
- Keep the workflow manually dispatched via `workflow_dispatch`.

## Review Focus

- **Job dependency:** the release job must wait for both platform builds; test by checking `needs` contains both job IDs.
- **Architecture drift:** the macOS job must run on `macos-13` and pass `x86_64-apple-darwin` to both runner and Tauri builds; test by inspecting the workflow commands and runner declaration.
- **Runner resource omission:** the Tauri bundle must contain `resources/src-darwin`; test by checking the explicit copy destination and the macOS resource config invocation.
- **Artifact path mismatch:** the upload path must use the target-specific Tauri output directory; test by checking the path includes `target/x86_64-apple-darwin/release/bundle`.
- **Release asset loss:** the release action must receive Windows plus macOS assets; test by checking the final `files` list includes the Windows ZIP, macOS DMG, and zipped `.app`.

---

### Task 1: Add and publish the macOS Intel CI build

**Files:**
- Modify: `.github/workflows/build-release.yaml` — add the dedicated macOS build job and extend release staging/upload.
- Test: `.github/workflows/build-release.yaml` — validate YAML/action structure with `actionlint` when available, plus whitespace and targeted content checks.

**Interfaces:**
- Consumes: Existing `workflow_dispatch`, Windows `build-tauri` job, `src-darwin`, `src-tauri/tauri.macos.conf.json`, and Tauri bundle output.
- Produces: Artifact `tauri-macos-release` containing `src-tauri/target/x86_64-apple-darwin/release/bundle/`; release assets named under `discord-quest-completer-macos-x64-${version}`.

- [ ] **Step 1: Record the current workflow validation baseline**

Run:

```bash
git diff --check -- .github/workflows/build-release.yaml
```

Expected: command exits successfully before the workflow change.

- [ ] **Step 2: Add the dedicated `build-tauri-macos` job**

Modify `.github/workflows/build-release.yaml` after the existing Windows build job. Configure:

- `runs-on: macos-13`.
- The same checkout, pnpm setup, Node LTS setup, Rust stable setup, Cargo registry/index caching, game-list download, frontend dependency installation, and Tauri action pattern used by the current workflow.
- `rustup target add x86_64-apple-darwin`.
- `cargo build --release --target x86_64-apple-darwin` from `src-darwin`.
- Copy `src-darwin/target/x86_64-apple-darwin/release/src-darwin` to `src-tauri/resources/src-darwin`.
- Run Tauri with `--target x86_64-apple-darwin --config src-tauri/tauri.macos.conf.json`.
- Upload `src-tauri/target/x86_64-apple-darwin/release/bundle/` under the artifact name `tauri-macos-release`.

Do not edit the existing Windows build commands or its artifact name.

- [ ] **Step 3: Make release staging wait for and download both artifacts**

Modify the `release` job so `needs` includes both `build-tauri` and `build-tauri-macos`. Add a second artifact download for `tauri-macos-release` into `tauri-macos-release`, leaving the existing Windows download unchanged.

- [ ] **Step 4: Stage macOS release assets without changing Windows packaging**

In the release preparation step, keep the current Windows directory and ZIP commands intact. Add a directory named `discord-quest-completer-macos-x64-${version}` and stage:

- The generated macOS DMG from `tauri-macos-release/dmg/`.
- A ZIP of the generated `.app` from `tauri-macos-release/macos/`, created with macOS `ditto --c` so the app bundle metadata and resource forks are preserved.

Export `mac_build_release_dir` to `GITHUB_ENV` for later steps. Use shell globs that fail when the expected DMG or app bundle is absent rather than silently creating an incomplete release.

- [ ] **Step 5: Attach the macOS assets to the GitHub release**

Extend the release action's `files` list to include the existing Windows ZIP plus the staged macOS DMG and zipped `.app`. Keep the existing release tag, draft/prerelease behavior, disclaimer, and Windows environment variables unchanged. Include the macOS directory name in the artifact upload step so the generated macOS assets are retained as workflow artifacts too.

- [ ] **Step 6: Validate the workflow and diff**

Run:

```bash
if command -v actionlint >/dev/null 2>&1; then
  actionlint .github/workflows/build-release.yaml
else
  ruby -e 'require "yaml"; YAML.load_file(".github/workflows/build-release.yaml")'
fi
git diff --check
git diff -- .github/workflows/build-release.yaml
```

Expected:

- `actionlint` passes when installed; otherwise Ruby parses the YAML successfully.
- `git diff --check` exits successfully.
- The diff shows one new `macos-13` build job, both build jobs in release `needs`, a macOS artifact download/staging path, and macOS files in the release upload list.

- [ ] **Step 7: Commit the workflow change**

```bash
git add .github/workflows/build-release.yaml
git commit -m "ci: add macOS Intel release build"
```

Expected: one commit containing only the workflow implementation. Do not commit generated build artifacts or secrets.
