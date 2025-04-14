use minifb::{Key, ScaleMode, Window, WindowOptions, Scale};
// use tray_icon::TrayIconBuilder;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrA, SetWindowLongPtrA, ShowWindow, GWL_EXSTYLE, SW_MINIMIZE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW};
const WIDTH: usize = 200;
const HEIGHT: usize = 200;
fn main() {
    //  With Tray-icon deps, final binary size is 161 KB
    // Without Tray-icon deps, final binary size is 136 KB
    // let mut rgba_data = Vec::with_capacity(32 * 32 * 4);
    // for _ in 0..32*32 {
    //     // White pixel (RGBA: 255, 255, 255, 255)
    //     rgba_data.push(255); // R
    //     rgba_data.push(255); // G
    //     rgba_data.push(255); // B
    //     rgba_data.push(255); // A
    // }
    // let icon = tray_icon::Icon::from_rgba(
    //     rgba_data,
    //     32,
    //     32,
    // ).expect("Failed to create icon");

    // Build the tray icon
    // let _tray_icon = TrayIconBuilder::new()
    //     .with_tooltip("Runner")
    //     .with_icon(icon)
    //     .build()
    //     .expect("Failed to create tray icon");
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
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {  
        window.update()
    }
}