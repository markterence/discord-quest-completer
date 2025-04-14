#![windows_subsystem = "windows"]
use minifb::{Key, ScaleMode, Window, WindowOptions, Scale};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrA, SetWindowLongPtrA, ShowWindow, GWL_EXSTYLE, SW_HIDE, SW_MINIMIZE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW};
const WIDTH: usize = 200;
const HEIGHT: usize = 200;
fn main() {
    let mut window = Window::new(
            "Discord Activity (runner)",
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
    let hwnd: HWND = HWND(window.get_window_handle());
    unsafe {
        let style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        let new_style = (style & !WS_EX_APPWINDOW.0 as isize) | WS_EX_TOOLWINDOW.0 as isize;
        SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_style);
        let _ = ShowWindow(hwnd, SW_MINIMIZE);
        // let _ = ShowWindow(hwnd, SW_HIDE);
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {  
        window.update()
    }
}