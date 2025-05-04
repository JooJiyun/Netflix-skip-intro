// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tray_item::{IconSource, TrayItem};

fn main() {
    let mut tray = TrayItem::new("My App", IconSource::Resource("appicon")).unwrap();

    tray.add_menu_item("종료", || {
        std::process::exit(0);
    })
    .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
