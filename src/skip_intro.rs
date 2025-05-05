use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use uiautomation::inputs::Mouse;
use uiautomation::Result;
use uiautomation::{UIAutomation, UIElement};

use crate::{
    ELEMENT_NAME_NETFLIX, ELEMENT_NAME_SKIP_BUTTON, THREAD_CHECK_TIME_INTERVAL,
    THREAD_NAME_NETFLIX_SKIP_INTRO,
};

pub fn spawn_skip_intro_thread() -> Sender<()> {
    // only thread terminate event send, receive
    let (event_sender, event_receiver) = channel::<()>();

    let thread_builder = thread::Builder::new().name(THREAD_NAME_NETFLIX_SKIP_INTRO.to_string());
    let thread_handle_result = thread_builder.spawn(move || {
        skip_intro_thread_inner(event_receiver);
    });

    if let Err(e) = thread_handle_result {
        eprintln!("failed spawn skip intro thread : {:?}", e);
    }

    return event_sender;
}

fn skip_intro_thread_inner(receiver: Receiver<()>) {
    loop {
        // receive terminate event only
        if let Ok(_) = receiver.try_recv() {
            println!("received terminate");
            return;
        }

        // click skip intro if exist
        if let Err(e) = click_skip_intro() {
            eprintln!("failed click ckip intro : {:?}", e);
        }

        // sleep
        thread::sleep(THREAD_CHECK_TIME_INTERVAL);
    }
}

fn click_skip_intro() -> Result<()> {
    let netflix_roots = find_netflix_root_elements()?;
    for netflix_root in netflix_roots {
        println!("found netflix tab");
        let skip_buttons = find_netflix_skip_button(&netflix_root)?;
        for skip_button in skip_buttons {
            if skip_button.get_name()? == ELEMENT_NAME_SKIP_BUTTON {
                println!("click skip button");
                click_element(&skip_button)?;
            }
        }
    }

    Ok(())
}

fn find_netflix_root_elements() -> Result<Vec<UIElement>> {
    let automation: UIAutomation = UIAutomation::new()?;
    let root = automation.get_root_element()?;
    Ok(automation
        .create_matcher()
        .from(root)
        .timeout(1000)
        .depth(3)
        .contains_name(ELEMENT_NAME_NETFLIX)
        .find_all()?)
}

fn find_netflix_skip_button(root: &UIElement) -> Result<Vec<UIElement>> {
    let automation: UIAutomation = UIAutomation::new()?;
    Ok(automation
        .create_matcher()
        .from(root.clone())
        .timeout(1000)
        .depth(3)
        .control_type(uiautomation::controls::ControlType::Button)
        .find_all()?)
}

fn click_element(element: &UIElement) -> Result<()> {
    let click_prev_point = Mouse::get_cursor_pos()?;

    element.try_focus();
    let button_point = element.get_clickable_point()?;
    if let Some(point) = button_point {
        let mouse = Mouse::default().move_time(50);
        mouse.click(point)?;
        mouse.move_to(click_prev_point)?;
    }

    Ok(())
}
