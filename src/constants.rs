// Constants
pub const ASCII_ART: &str = r#" _  _
(o)(o)--.
 \../ (  )hjw
 m\/m--m'`--."#;

pub const FAREWELL_MESSAGE: &str = "Pausemouse ran away... See you next time!";

// Compound greeting message
pub fn greeting_message() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("{}\n\nPausemouse v{} is running!\n", ASCII_ART, version)
}