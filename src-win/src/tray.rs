use tray_icon::TrayIcon;
use tray_icon::TrayIconBuilder;
use tray_icon::Icon;

pub fn create_tray_icon() -> TrayIcon {
    let mut rgba_data = Vec::with_capacity(32 * 32 * 4);
    for _ in 0..32*32 {
        // White pixel (RGBA: 255, 255, 255, 255)
        rgba_data.push(255); // R
        rgba_data.push(255); // G
        rgba_data.push(255); // B
        rgba_data.push(255); // A
    }
    let icon = Icon::from_rgba(
        rgba_data,
        32,
        32,
    ).expect("Failed to create icon");

    TrayIconBuilder::new()
        .with_tooltip("Runner")
        .with_icon(icon)
        .build()
        .expect("Failed to create tray icon")
}
