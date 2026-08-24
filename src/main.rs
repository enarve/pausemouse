#![windows_subsystem = "windows"] // windows-specific: hide default terminal window

mod app;
mod model;
mod messages;
mod menu;

use iced::{daemon, theme, Color, Result};
use embed_plist::embed_info_plist;
use crate::app::{boot, update, view, subscription, title};

fn main() -> Result {
    embed_info_plist!("../Info.plist");
    
    daemon(boot, update, view)
        .style(|_state, _theme| theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .subscription(subscription)
        .title(title)
        .run()
}


