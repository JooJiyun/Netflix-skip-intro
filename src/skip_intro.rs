use uiautomation::Result;
use uiautomation::{UIAutomation, UIElement};

const NETFLIX_ELEMENT_NAME: &str = "넷플릭스";
const NETFLIX_SKIP_BUTTON_NAME: &str = "오프닝 건너뛰기";

pub fn click_skip_intro() {
    let automation: UIAutomation = UIAutomation::new().expect("failed create automation");
    let netflix_roots = find_netflix_root_elements(&automation).expect("failed find netflix roots");
    for netflix_root in netflix_roots {
        let skip_buttons = find_netflix_skip_button(&automation, &netflix_root)
            .expect("failed find netflix skip buttons");
        for skip_button in skip_buttons {
            skip_button.click().expect("failed click buttons");
        }
    }
}

fn find_netflix_root_elements(automation: &UIAutomation) -> Result<Vec<UIElement>> {
    let root = automation.get_root_element()?;
    Ok(automation
        .create_matcher()
        .from(root)
        .timeout(1000)
        .depth(3)
        .contains_name(NETFLIX_ELEMENT_NAME)
        .find_all()?)
}

fn find_netflix_skip_button(automation: &UIAutomation, root: &UIElement) -> Result<Vec<UIElement>> {
    Ok(automation
        .create_matcher()
        .from(root.clone())
        .timeout(1000)
        .depth(3)
        .control_type(uiautomation::controls::ControlType::Button)
        .contains_name(NETFLIX_SKIP_BUTTON_NAME)
        .find_all()?)
}
