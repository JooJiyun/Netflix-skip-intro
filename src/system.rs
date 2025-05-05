use std::sync::mpsc::Sender;

use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuItem},
    TrayIcon, TrayIconBuilder,
};
use winit::application::ApplicationHandler;

use crate::skip_intro::spawn_skip_intro_thread;

pub enum SystemEvent {
    SystemTrayEvent(tray_icon::menu::MenuEvent),
}

pub struct System {
    tray_icon: Option<TrayIcon>,

    quit_tray_item: MenuItem,
    switch_tray_item: CheckMenuItem,

    skip_into_thread_terminate_sender: Option<Sender<()>>,
}

impl System {
    pub fn new() -> System {
        System {
            tray_icon: None,
            quit_tray_item: MenuItem::new("exit", true, None),
            switch_tray_item: CheckMenuItem::new("skip intro", true, true, None),
            skip_into_thread_terminate_sender: Some(spawn_skip_intro_thread()),
        }
    }

    fn new_tray_icon(&mut self) -> TrayIcon {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/app-icon.ico");
        let icon = load_tray_icon(std::path::Path::new(path));

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

impl ApplicationHandler<SystemEvent> for System {
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

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: SystemEvent) {
        match event {
            SystemEvent::SystemTrayEvent(menu_event) => {
                if self.quit_tray_item.id() == menu_event.id() {
                    std::process::exit(0);
                } else if self.switch_tray_item.id() == menu_event.id() {
                    println!("system tray switch toggle");
                    // handling thread
                    if self.switch_tray_item.is_checked() {
                        let sender = spawn_skip_intro_thread();
                        self.skip_into_thread_terminate_sender = Some(sender);
                    } else {
                        if let Some(sender) = &self.skip_into_thread_terminate_sender {
                            if let Err(e) = sender.send(()) {
                                eprintln!("failed send terminate event : {:?}", e);
                            }
                        }
                    }
                    // toggle tray item checked state
                    self.switch_tray_item
                        .set_checked(self.switch_tray_item.is_checked());
                }
            }
        }
    }
}

fn load_tray_icon(path: &std::path::Path) -> tray_icon::Icon {
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
