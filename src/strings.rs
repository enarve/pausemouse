use std::time::Duration;

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

pub fn next_break() -> String {
    let work_duration = Duration::from_secs(60);
    format!("Next break in {:?}", work_duration)
}