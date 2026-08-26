use std::time::{Instant};
use iced::window;
use tray_icon::menu;

#[derive(Debug, Clone)]
pub enum Message {
    Initialized(window::Id),
    WindowDragged,
    Tick(Instant),
    OpenSettings,
    MenuClicked(menu::MenuId),
    WindowClosed(window::Id),
    WorkDurationChanged(String),
    BreakDurationChanged(String),
    WorkDurationSubmitted,
    BreakDurationSubmitted,
}