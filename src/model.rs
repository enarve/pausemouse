use std::time::{Instant, Duration};
use iced::window;

pub struct Config {
    pub work_duration: Duration,
    pub break_duration: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_duration: Duration::from_secs(15),
            break_duration: Duration::from_secs(10)
        }
    }
}

pub struct SettingsInputBuffer {
    pub _work_duration_input: String,
    pub _break_duration_input: String,
}

impl SettingsInputBuffer {
    fn from_config(config: &Config) -> Self {
        Self {
            _work_duration_input: config.work_duration.as_secs().to_string(),
            _break_duration_input: config.break_duration.as_secs().to_string(),
        }
    }
}

impl Default for SettingsInputBuffer {
    fn default() -> Self {
        SettingsInputBuffer::from_config(&Config::default())
    }
}

pub enum State {
    Working(Instant),
    Breaking(Instant)
}

pub struct Windows {
    pub main: Option<window::Id>,
    pub settings: Option<window::Id>,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            main: None,
            settings: None,
        }
    }
}