mod skip_intro;
mod system;

const ELEMENT_NAME_NETFLIX: &str = "넷플릭스";
const ELEMENT_NAME_SKIP_BUTTON: &str = "오프닝 건너뛰기";

const THREAD_NAME_NETFLIX_SKIP_INTRO: &str = "skip_intro_thread";
const THREAD_CHECK_TIME_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000);

const MOUSE_MOVE_TIME: u64 = 50;

pub fn run_main() {
    let event_loop = winit::event_loop::EventLoop::<system::SystemEvent>::with_user_event()
        .build()
        .unwrap();

    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(system::SystemEvent::SystemTrayEvent(event)) {
            eprintln!("faile proxy send event : {:?}", e.to_string());
        }
    }));

    let mut app = system::System::new();
    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {:?}", err);
    }
}
