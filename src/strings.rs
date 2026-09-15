use std::time::Duration;
use humantime;

// Strings

pub const TITLE: &str = "Pausemouse";

pub const ASCII_ART: &str = r#" _  _
(o)(o)--.
 \../ (  )hjw
 m\/m--m'`--."#;

pub const FAREWELL: &str = "Pausemouse ran away... See you next time!";

pub const BREAK: &str = "Time for a pause!";

// Compound greeting message
pub fn greeting() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("{}\n\nPausemouse v{} is running!\n", ASCII_ART, version)
}

pub fn next_break(duration: Duration) -> String {
    format!("Next break in {}", humantime::format_duration(duration))
}

pub const SETTINGS_TITLE: &str = "Time settings in seconds";
pub const WORK_SETTING_LABEL: &str = "Work";
pub const BREAK_SETTING_LABEL: &str = "Break";