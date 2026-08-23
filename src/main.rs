#![windows_subsystem = "windows"] // windows-specific: hide default terminal window

mod app;
mod model;
mod messages;
mod menu;

use crate::app::App;

fn main() -> iced::Result {
    embed_plist::embed_info_plist!("../Info.plist");
    iced::daemon(App::boot, App::update, App::view)
        .style(|_state, _theme| iced::theme::Style {
            background_color: iced::Color::TRANSPARENT, // Removes default canvas layer
            text_color: iced::Color::WHITE,
        })
        .subscription(App::subscription)
        .title(App::title)
        .run()
}


