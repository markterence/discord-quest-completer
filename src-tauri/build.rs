use std::path::PathBuf;

fn main() {
    // Get the target OS
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    // Build the Tauri application
    tauri_build::build();
    
    // Check if the platform-specific runner exists
    let mut resource_path = PathBuf::from("resources");
    
    match target_os.as_str() {
        "windows" => {
            resource_path.push("src-win.exe");
        }
        "linux" => {
            resource_path.push("src-linux");
        }
        "macos" => {
            resource_path.push("src-macos");
        }
        _ => {
            // Unknown OS, just continue
            return;
        }
    }
    
    // Tell cargo to rerun if the resource file changes
    if resource_path.exists() {
        println!("cargo:rerun-if-changed={}", resource_path.display());
    } else {
        eprintln!("Warning: Platform runner not found at {}", resource_path.display());
        eprintln!("Make sure to run the appropriate build:runner script for your platform");
    }
}