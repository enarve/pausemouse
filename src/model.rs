use std::time::{Instant};
use iced::window;

use crate::config::Config;

#[derive(Clone)]
pub struct SettingsInputBuffer {
    pub work_duration_input: String,
    pub break_duration_input: String,
}

impl SettingsInputBuffer {
    fn from_config(config: &Config) -> Self {
        Self {
            work_duration_input: config.work_duration.as_secs().to_string(),
            break_duration_input: config.break_duration.as_secs().to_string(),
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