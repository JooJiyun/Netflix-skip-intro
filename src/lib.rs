mod skip_intro;
pub mod system;

const ELEMENT_NAME_NETFLIX: &str = "넷플릭스";
const ELEMENT_NAME_SKIP_BUTTON: &str = "오프닝 건너뛰기";

const THREAD_NAME_NETFLIX_SKIP_INTRO: &str = "skip_intro_thread";
const THREAD_CHECK_TIME_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1000);
