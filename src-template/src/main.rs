use std::env;
use std::thread;
use std::time::Duration;
use std::process;
fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if daemon mode is requested
    let daemon_mode = args.iter().any(|arg| arg == "-d" || arg == "--daemon");
    
    // Get the executable name for display purposes
    let exe_name = env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|name| name.to_string_lossy().to_string()))
        .unwrap_or_else(|| "myrustapp.exe".to_string());
    
    // Print startup message
    println!("Application started. PID: {}", process::id());
    
    if daemon_mode {
        println!("Running in background mode. Process can be terminated by:");
        println!("- Task Manager");
        println!("- taskkill /F /PID {}", process::id());
        println!("- taskkill /F /IM {}", exe_name);
    } else {
        println!("Running in foreground mode. Press Ctrl+C to exit.");
        println!("Run with -d or --daemon flag to run in background mode.");
    }
       
     
    // Main application loop
    println!("Application is now visible in Discord's game activity selector.");
    loop { 
        // Sleep to avoid consuming CPU resources
        thread::sleep(Duration::from_secs(300));
    }
}