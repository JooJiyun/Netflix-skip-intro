use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use uiautomation::inputs::Mouse;
use uiautomation::Result;
use uiautomation::{UIAutomation, UIElement};

use crate::{
    ELEMENT_NAME_NETFLIX, ELEMENT_NAME_SKIP_BUTTON, MOUSE_MOVE_TIME, THREAD_CHECK_TIME_INTERVAL,
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
            println!("found button : {:?}", skip_button);
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
        .depth(10)
        .control_type(uiautomation::controls::ControlType::Button)
        .find_all()?)
}

fn click_element(element: &UIElement) -> Result<()> {
    let click_prev_point = Mouse::get_cursor_pos()?;

    element.try_focus();
    let button_point = element.get_clickable_point()?;
    if let Some(point) = button_point {
        let mouse = Mouse::default().move_time(MOUSE_MOVE_TIME);
        mouse.click(point)?;
        mouse.move_to(click_prev_point)?;
    }

    Ok(())
}

// use uiautomation::UITreeWalker;

// fn print_all_element() -> Result<()> {
//     // get root ui
//     let automation: UIAutomation = UIAutomation::new()?;
//     let root = automation.get_root_element()?;
//     let walker = automation.create_tree_walker()?;

//     // get process root ui element
//     let process_ui_roots = automation
//         .create_matcher()
//         .from(root)
//         .timeout(10000)
//         .find_all()?;

//     // get text
//     for process_ui_root in process_ui_roots {
//         print_element_recursive(&walker, &process_ui_root, 1)?;
//     }

//     Ok(())
// }

// fn print_element_recursive(walker: &UITreeWalker, element: &UIElement, level: usize) -> Result<()> {
//     println!("{:?} {:?}", level, element);

//     // 자식 탐색
//     if let Ok(child) = walker.get_first_child(&element) {
//         print_element_recursive(walker, &child, level + 1)?;

//         let mut next = child;
//         while let Ok(sibling) = walker.get_next_sibling(&next) {
//             print_element_recursive(walker, &sibling, level + 1)?;
//             next = sibling;
//         }
//     }

//     Ok(())
// }
