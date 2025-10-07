use tray_icon::TrayIcon;
use tray_icon::TrayIconBuilder;
use tray_icon::Icon;
use tray_icon::menu::{Menu};

pub fn create_tray_icon(tray_menu: Menu, title: &str) -> TrayIcon {

    let icon = Icon::from_resource(1 as u16, None).expect("msg");

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip(title)
        .with_icon(icon)
        .build()
        .expect("Failed to create tray icon")
}
