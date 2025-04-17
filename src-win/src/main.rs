#![windows_subsystem = "windows"]
use minifb::{Key, ScaleMode, Window, WindowOptions, Scale,};
use windows::Win32::Foundation::{COLORREF, HWND};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrA, SetLayeredWindowAttributes, SetWindowLongA, SetWindowLongPtrA, ShowWindow, GWL_EXSTYLE, GWL_STYLE, LWA_ALPHA, SW_HIDE, SW_MINIMIZE, SW_SHOWMINNOACTIVE, SW_SHOWNOACTIVATE, WS_CAPTION, WS_EX_APPWINDOW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU};

mod tray;
use tray::create_tray_icon;
#[path = "text-renderer.rs"]
mod text_renderer;
use text_renderer::TextRenderer;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
fn main() {
    // // Create the tray icon first
    let _tray = create_tray_icon();
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

    let text = TextRenderer::new(WIDTH, HEIGHT, 2);

    let hwnd: HWND = HWND(window.get_window_handle());
    unsafe { 
        // let new_style = (style & !WS_EX_APPWINDOW.0 as isize) | WS_EX_TOOLWINDOW.0 as isize; 
        // SetWindowLongA(hwnd, GWL_EXSTYLE, (WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as i32);
        // SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_style);
        // delay before minimizing
        let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        let new_ex_style = (ex_style & !WS_EX_APPWINDOW.0 as isize) |
            WS_EX_TOOLWINDOW.0 as isize | // Make the window a tool window (so it doesn't show in the taskbar)
            WS_EX_TRANSPARENT.0 as isize; // Make the window transparent

        SetWindowLongPtrA(hwnd, GWL_EXSTYLE, new_ex_style);
    

        // Also modify the normal window style to remove the caption/title
        let style = GetWindowLongPtrA(hwnd, GWL_STYLE);
        let new_style = style & 
            !(WS_CAPTION.0 as isize) &     // Remove caption (title bar)
            !(WS_SYSMENU.0 as isize) &     // Remove system menu
            !(WS_MINIMIZEBOX.0 as isize) & // Remove minimize box
            !(WS_MAXIMIZEBOX.0 as isize);  // Remove maximize box
        
        SetWindowLongPtrA(hwnd, GWL_STYLE, new_style);
        // let _ = ShowWindow(hwnd, SW_MINIMIZE);
        // window.
        // let _ = ShowWindow(hwnd, SW_HIDE);
        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
    }
    // unsafe {
    //     SetWindowLongA(hwnd, GWL_EXSTYLE, (WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as i32);
    // } 
    // let ckey = COLORREF(0);

    // unsafe { 
    //     let _ = SetLayeredWindowAttributes(hwnd, ckey , 255, LWA_ALPHA);
    // };
    
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    while window.is_open() && !window.is_key_down(Key::Escape) {  
        // for i in 0..WIDTH * HEIGHT {
        //     buffer[i] = 0x00000000;  // Almost fully transparent (0x01 alpha)
        // }
        // window.update_with_buffer(&buffer, WIDTH, HEIGHT)
        // .expect("Unable to update the window");
        text.draw(&mut buffer, (20, HEIGHT - 20), "This is a dummy game window");
        window.update()
    }
}