#![windows_subsystem = "windows"] 

use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, SIZE, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DispatchMessageA, GetClientRect, GetMessageA, GetWindowLongPtrA, PostQuitMessage, RegisterClassA, SetWindowLongPtrA, SetWindowPos, ShowWindow, TranslateMessage, CW_USEDEFAULT, GWL_EXSTYLE, HMENU, MSG, SWP_NOZORDER, SW_HIDE, SW_SHOWNOACTIVATE, SW_SHOWNORMAL, WINDOW_EX_STYLE, WINDOW_STYLE, WM_COMMAND, WM_CTLCOLORSTATIC, WM_DESTROY, WM_SIZE, WNDCLASSA, WS_CHILD, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_OVERLAPPEDWINDOW, WS_VISIBLE
};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::Graphics::Gdi::{GetDC, GetStockObject, GetTextExtentPoint32A, ReleaseDC, SetBkMode, HDC, NULL_BRUSH, TRANSPARENT};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA};
use windows::core::{PCSTR};
use std::ffi::CString;
use std::env;

mod tray;
use tray::create_tray_icon;

const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;
const LINK_BUTTON_ID: i32 = 1001;

static mut TITLE_LABEL: Option<HWND> = None;
static mut LINK_LABEL: Option<HWND> = None;

#[derive(Debug)]
struct Config {
    title: String,
    start_minimized: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            title: "Discord Quest Completer".to_string(),
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

fn calculate_text_width(hwnd: HWND, text: &str) -> i32 {
    unsafe {
        let hdc = GetDC(Some(hwnd));
        if hdc.is_invalid() {
            return 200; // fallback width
        }
        
        let text_bytes = text.as_bytes();
        let mut size = SIZE::default();
        
        let result = GetTextExtentPoint32A(
            hdc,
            text_bytes,
            &mut size
        );
        
        let _ = ReleaseDC(Some(hwnd), hdc);
        
        if result.as_bool() {
            size.cx + 20 // Add 20px padding
        } else {
            200 // fallback width
        }
    }
}


fn create_label(parent_hwnd: HWND, text: &str, instance: HINSTANCE, x: i32, y:i32, w:Option<i32>, h:i32) -> Option<HWND> {
    unsafe {
        let class_name = CString::new("STATIC").ok()?;
        let window_text = CString::new(text).ok()?;
         // Calculate width dynamically if not provided
        let width = w.unwrap_or_else(|| calculate_text_width(parent_hwnd, text));
        let label_hwnd: Result<HWND, windows::core::Error> = CreateWindowExA(
            WS_EX_TRANSPARENT, // Transparent background
            PCSTR(class_name.as_ptr() as *const u8), // Class name
            PCSTR(window_text.as_ptr() as *const u8), // Window text
            WS_CHILD | WS_VISIBLE, // Basic window style
            x,  // x position
            y,  // y position
            width, // width
            h,  // height
            Some(parent_hwnd), // Parent window
            None, // Menu
            Some(instance), // Instance handle
            None, // Additional application data
        );
        
        match label_hwnd {
            Ok(hwnd) if !hwnd.0.is_null() => Some(hwnd),
            _ => None,
        }
    }
}

fn create_link_label(parent_hwnd: HWND, text: &str, instance: HINSTANCE) -> Option<HWND> {
    unsafe {
        let class_name = CString::new("BUTTON").ok()?;
        let window_text = CString::new(text).ok()?;
        
        let label_hwnd = CreateWindowExA(
            WS_EX_TRANSPARENT, // Transparent background
            PCSTR(class_name.as_ptr() as *const u8), // Class name
            PCSTR(window_text.as_ptr() as *const u8), // Window text
            WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x00000000), // BS_PUSHBUTTON style
            10,    // x position (will be updated)
            HEIGHT - 80, // y position (bottom-left, 40 pixels from bottom)
            150,   // width
            25,    // height
            Some(parent_hwnd), // Parent window
            Some(HMENU(LINK_BUTTON_ID as _)), // Menu/ID
            Some(instance), // Instance handle
            None, // Additional application data
        );
        
        match label_hwnd {
            Ok(hwnd) if !hwnd.0.is_null() => Some(hwnd),
            _ => None,
        }
    }
}

// Helper function to reposition the link label to bottom-left
fn position_link_label(parent_hwnd: HWND) {
    unsafe {
        if let Some(link_hwnd) = LINK_LABEL {
            let mut rect = RECT::default();
            if GetClientRect(parent_hwnd, &mut rect).is_ok() {
                let client_height = rect.bottom - rect.top;
                // Position 10 pixels from left, 30 pixels from bottom
                let _ = SetWindowPos(
                    link_hwnd,
                    None,
                    10,                    // x position
                    client_height - 40,    // y position (40 pixels from bottom)
                    0, 0,                  // width/height (ignored with SWP_NOSIZE)
                    SWP_NOZORDER | windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE,
                );
            }
        }
    }
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
        WM_CTLCOLORSTATIC => {
            // Make static controls (labels) have transparent backgrounds
            let hdc = HDC(wparam.0 as *mut _);
            SetBkMode(hdc, TRANSPARENT);
            LRESULT(GetStockObject(NULL_BRUSH).0 as isize)
        }
        WM_SIZE => {
            // Reposition the link label when window is resized
            position_link_label(hwnd);
            DefWindowProcA(hwnd, msg, wparam, lparam)
        } 
        WM_COMMAND => { 
            let control_id = wparam.0 & 0xFFFF;
            if control_id == LINK_BUTTON_ID as usize { 
                let url = windows::core::w!("https://github.com/markterence/discord-quest-completer");
                let operation = windows::core::w!("open");
                let _ = ShellExecuteW(
                    None,
                    operation, 
                    url,
                    None,
                    None,
                    SW_SHOWNORMAL,
                );
            }
            LRESULT(0)
        } 
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

fn create_native_window(title: &str) -> Result<(HWND, HINSTANCE), Box<dyn std::error::Error>> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        let class_name = CString::new("DiscordQuestCompleter")?;
        let window_title = CString::new(title)?;

        // Create a white background brush
        let brush = windows::Win32::Graphics::Gdi::CreateSolidBrush(
            COLORREF(0x00FFFFFF) // White
        );

        let wc = WNDCLASSA {
            lpfnWndProc: Some(window_proc),
            hInstance: HINSTANCE(instance.0),
            lpszClassName: PCSTR(class_name.as_ptr() as *const u8),
            hbrBackground: brush, // Set background brush
            ..Default::default()
        };

        RegisterClassA(&wc);

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            PCSTR(class_name.as_ptr() as *const u8),
            PCSTR(window_title.as_ptr() as *const u8),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIDTH,
            HEIGHT,
            None,
            None,
            Some(HINSTANCE(instance.0)),
            None,
        ); 
        match hwnd {
            Ok(hwnd) if !hwnd.0.is_null() => Ok((hwnd, HINSTANCE(instance.0))),
            _ => Err("Failed to create window".into()),
        }
    }
}

fn main() {
    let config = parse_args();
    
    let tray_menu = tray_icon::menu::Menu::new();
    let quit_i = tray_icon::menu::MenuItem::new("Quit", true, None);
    let show_i = tray_icon::menu::MenuItem::new("Show", true, None);
    let hide_i = tray_icon::menu::MenuItem::new("Hide", true, None);

    let _tray_menu = tray_menu.append_items(&[
        &show_i,
        &hide_i,
        &tray_icon::menu::PredefinedMenuItem::separator(),
        &quit_i
    ]);

    let _tray = create_tray_icon(tray_menu, &config.title);
 

    // Create native Windows window
    let (hwnd, instance) = match create_native_window(&config.title) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to create window: {}", e);
            return;
        }
    };
    
    // Create a Windows label to display the title at the top
    let title_label_hwnd = create_label(hwnd, &config.title, instance,10,  // x position
            40,  // y position  
           None,
            20,  // height
    );

    let _app_label_hwnd = create_label(hwnd, "Discord Quest Completer", instance, 10, 10, None, 20);
    
        let _app_label_hwnd = create_label(hwnd, "This program is part of the Discord Quest Completer", instance, 10, 60, None, 20);
    

    // Create a link label anchored to the bottom-left
    let link_label_hwnd = create_link_label(hwnd, "Source on Github", instance);
    
    // Store control references in static variables
    unsafe {
        TITLE_LABEL = title_label_hwnd;
        LINK_LABEL = link_label_hwnd;
    }
    
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
        
        // Windows message loop
        let mut msg = MSG::default();
        loop {
            // Handle tray event
            if let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
                if event.id == quit_i.id() {
                    PostQuitMessage(0);
                }

                if event.id == show_i.id() {
                    // Always restore window styles for normal display when showing from tray
                    let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
                    let new_ex_style = (ex_style & !(WS_EX_TOOLWINDOW.0 as isize | WS_EX_TRANSPARENT.0 as isize | WS_EX_LAYERED.0 as isize)) |
                        WS_EX_APPWINDOW.0 as isize; // Restore WS_EX_APPWINDOW to show in taskbar
                    
                    SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_ex_style);
                    let _ = ShowWindow(hwnd, SW_SHOWNORMAL);
                    let _ = windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow(hwnd);
                }

                if event.id == hide_i.id() {
                    // Hide window to tray (similar to --tray startup behavior)
                    let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
                    let new_ex_style = (ex_style & !WS_EX_APPWINDOW.0 as isize) |
                        WS_EX_TOOLWINDOW.0 as isize | // Make the window a tool window (so it doesn't show in the taskbar)
                        WS_EX_TRANSPARENT.0 as isize | // Make the window transparent
                        WS_EX_LAYERED.0 as isize; // WS_EX_LAYERED make the window layered (for transparency)
                    
                    SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_ex_style);
                    let _ = ShowWindow(hwnd, SW_HIDE);
                }
            }

            let ret = GetMessageA(&mut msg, None, 0, 0);
            if ret.0 == 0 || ret.0 == -1 {
                break;
            }
            
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}