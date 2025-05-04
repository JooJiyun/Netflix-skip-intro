// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder, TrayIconEvent,
};
use winit::{application::ApplicationHandler, event_loop::EventLoop};

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

struct Application {
    tray_icon: Option<TrayIcon>,
    quit_tray_item: MenuItem,
    switch_tray_item: CheckMenuItem,
}

impl Application {
    fn new() -> Application {
        Application {
            tray_icon: None,
            quit_tray_item: MenuItem::new("exit", true, None),
            switch_tray_item: CheckMenuItem::new("skip intro", true, true, None),
        }
    }

    fn new_tray_icon(&mut self) -> TrayIcon {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/app-icon.ico");
        let icon = load_icon(std::path::Path::new(path));

        // menu
        let menu = Menu::new();
        if let Err(err) = menu.append_items(&[&self.quit_tray_item, &self.switch_tray_item]) {
            println!("{err:?}");
        }

        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_icon(icon)
            .with_title("netflix-skip-intro")
            .build()
            .unwrap()
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if winit::event::StartCause::Init == cause {
            self.tray_icon = Some(self.new_tray_icon());
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::MenuEvent(menu_event) => {
                if self.quit_tray_item.id() == menu_event.id() {
                    std::process::exit(0);
                } else if self.switch_tray_item.id() == menu_event.id() {
                    self.switch_tray_item
                        .set_checked(self.switch_tray_item.is_checked());
                }
            }
            UserEvent::TrayIconEvent(_) => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    // set a tray event handler that forwards the event and wakes up the event loop
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::TrayIconEvent(event));
    }));
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let mut app = Application::new();

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {:?}", err);
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
