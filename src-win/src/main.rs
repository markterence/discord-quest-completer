#![windows_subsystem = "windows"]
use minifb::{Key, ScaleMode, Window, WindowOptions, Scale,};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrA, SetWindowLongPtrA, ShowWindow, GWL_EXSTYLE, GWL_STYLE, SW_SHOWNOACTIVATE, WS_CAPTION, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU};
mod tray;
use tray::create_tray_icon;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
fn main() {
    // // Create the tray icon first
    let _tray = create_tray_icon();
    let mut window = Window::new(
            "Discord Rich Presence Simulator (runner)",
            WIDTH,
            HEIGHT,
            WindowOptions {
                transparency: true,
                borderless: true,
                none: true,
                title: false,
                resize: false,
                scale: Scale::X1,
                scale_mode: ScaleMode::Center,
                topmost: false,
        }
    ).expect("Unable to create the window");
    window.set_target_fps(15); 

    #[allow(unused_variables, unused_mut)]
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    
    let hwnd: HWND = HWND(window.get_window_handle());
    unsafe { 
        let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);

        let new_ex_style = (ex_style & !WS_EX_APPWINDOW.0 as isize) |
            WS_EX_TOOLWINDOW.0 as isize | // Make the window a tool window (so it doesn't show in the taskbar)
            WS_EX_TRANSPARENT.0 as isize; // Make the window transparent
            WS_EX_LAYERED.0 as isize; 
        // WS_EX_LAYERED make the window layered (for transparency) Sometimes adding this makes discord not detect the game

        SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_ex_style);
    
        // Also modify the normal window style to remove the caption/title
        let style = GetWindowLongPtrA(hwnd, GWL_STYLE);
        let new_style = style & 
            !(WS_CAPTION.0 as isize) &     // Remove caption (title bar)
            !(WS_SYSMENU.0 as isize) &     // Remove system menu
            !(WS_MINIMIZEBOX.0 as isize) & // Remove minimize box
            !(WS_MAXIMIZEBOX.0 as isize);  // Remove maximize box
        
        SetWindowLongPtrA(hwnd, GWL_STYLE, new_style);

        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    }

    // Can't make transparent work with minifb, might check other libraries
    while window.is_open() && !window.is_key_down(Key::Escape) {  
        window.update();
    }
}