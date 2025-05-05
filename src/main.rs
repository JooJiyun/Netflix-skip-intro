// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tray_icon::menu::MenuEvent;
use winit::event_loop::EventLoop;

use netflix_skip::system::{System, SystemEvent};

fn main() {
    let event_loop = EventLoop::<SystemEvent>::with_user_event().build().unwrap();

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(SystemEvent::SystemTrayEvent(event)) {
            eprintln!("faile proxy send event : {:?}", e.to_string());
        }
    }));

    let mut app = System::new();
    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {:?}", err);
    }
}
