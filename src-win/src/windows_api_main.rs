#![windows_subsystem = "windows"]
use windows::Win32::Foundation::{HWND, HINSTANCE, WPARAM, LPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrA, SetWindowLongPtrA, ShowWindow, CreateWindowExA, RegisterClassExA,
    DefWindowProcA, GetMessageA, TranslateMessage, DispatchMessageA, PostQuitMessage,
    GWL_EXSTYLE, SW_HIDE, SW_SHOWNOACTIVATE, WS_EX_APPWINDOW, WS_EX_LAYERED, 
    WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_CHILD, WS_VISIBLE, WINDOW_EX_STYLE,
    WS_OVERLAPPEDWINDOW, WNDCLASSEXA, MSG, WM_DESTROY, WM_KEYDOWN
};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::PCSTR;
use std::ffi::CString;
use std::env;
mod tray;
use tray::create_tray_icon;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

#[derive(Debug)]
struct Config {
    title: String,
    start_minimized: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: "Discord Quest Completer (runner)".to_string(),
            start_minimized: false,
        }
    }
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();
    
    let mut i = 1; // Skip program name
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                if i + 1 < args.len() {
                    config.title = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --title requires a value");
                    i += 1;
                }
            }
            "--tray" => {
                config.start_minimized = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    
    config
}

// Window procedure for handling messages
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            if wparam.0 == VK_ESCAPE.0 as usize {
                PostQuitMessage(0);
            }
            DefWindowProcA(hwnd, msg, wparam, lparam)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

fn create_window(title: &str) -> Option<HWND> {
    unsafe {
        let hinstance = GetModuleHandleA(None).ok()?;
        
        let class_name = CString::new("DiscordQuestCompleter").ok()?;
        let window_title = CString::new(title).ok()?;
        
        let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: Default::default(),
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: HINSTANCE(hinstance.0),
            hIcon: Default::default(),
            hCursor: Default::default(),
            hbrBackground: Default::default(),
            lpszMenuName: PCSTR::null(),
            lpszClassName: PCSTR(class_name.as_ptr() as *const u8),
            hIconSm: Default::default(),
        };
        
        if RegisterClassExA(&wc) == 0 {
            return None;
        }
        
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            PCSTR(class_name.as_ptr() as *const u8),
            PCSTR(window_title.as_ptr() as *const u8),
            WS_OVERLAPPEDWINDOW,
            100, 100, // x, y position
            WIDTH as i32, HEIGHT as i32, // width, height
            None, // Parent window
            None, // Menu
            Some(HINSTANCE(hinstance.0)), // Instance handle
            None, // Additional application data
        );
        
        match hwnd {
            Ok(hwnd) if !hwnd.0.is_null() => Some(hwnd),
            _ => None,
        }
    }
}

fn create_label(parent_hwnd: HWND, text: &str) -> Option<HWND> {
    unsafe {
        let class_name = CString::new("STATIC").ok()?;
        let window_text = CString::new(text).ok()?;
        
        let label_hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0), // Extended window style
            PCSTR(class_name.as_ptr() as *const u8), // Class name
            PCSTR(window_text.as_ptr() as *const u8), // Window text
            WS_CHILD | WS_VISIBLE, // Window style
            10,  // x position
            10,  // y position  
            180, // width
            30,  // height
            Some(parent_hwnd), // Parent window
            None, // Menu
            None, // Instance handle
            None, // Additional application data
        );
        
        match label_hwnd {
            Ok(hwnd) if !hwnd.0.is_null() => Some(hwnd),
            _ => None,
        }
    }
}

fn main() {
    let config = parse_args();
    
    // Create the tray icon first
    let _tray = create_tray_icon();
    
    // Create a native Windows window
    let hwnd = create_window(&config.title).expect("Unable to create the window");
    
    // Create a Windows label to display the title
    let _label_hwnd = create_label(hwnd, &config.title);
    
    unsafe { 
        if config.start_minimized {
            // Only modify window styles when starting minimized
            let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
            let new_ex_style = (ex_style & !WS_EX_APPWINDOW.0 as isize) |
                WS_EX_TOOLWINDOW.0 as isize | // Make the window a tool window (so it doesn't show in the taskbar)
                WS_EX_TRANSPARENT.0 as isize | // Make the window transparent
                WS_EX_LAYERED.0 as isize; // WS_EX_LAYERED make the window layered (for transparency)
            
            SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_ex_style);
            let _ = ShowWindow(hwnd, SW_HIDE);
        } else {
            // For normal window, just show it without modifying styles
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        }
    }

    // Message loop
    unsafe {
        let mut msg = MSG::default();
        loop {
            let result = GetMessageA(&mut msg, None, 0, 0);
            if result.0 == 0 || result.0 == -1 {
                break;
            }
            
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}