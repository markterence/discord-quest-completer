// Cross-platform "kill by PID" adapter.
//
// `pid` is always the OS-native PID returned by `tokio::process::Command::spawn()`
// for the process we launched. On Linux, if a game is ever launched through Wine,
// this would be the PID of the wine host process rather than the inner Windows PE —
// killing it is still correct for tearing down what we spawned, but a full Wine-launch
// story (invoking `wine <exe>` and cleaning up whatever process tree Wine spawns) is a
// separate concern from this adapter.

#[cfg(target_os = "windows")]
pub fn kill_process(pid: u32) -> Result<(), String> {
    let output = std::process::Command::new("taskkill")
        .arg("/F")
        .arg("/PID")
        .arg(pid.to_string())
        .output()
        .map_err(|e| format!("Failed to execute taskkill: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to stop process: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn kill_process(pid: u32) -> Result<(), String> {
    let output = std::process::Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .output()
        .map_err(|e| format!("Failed to execute kill: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to stop process: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
