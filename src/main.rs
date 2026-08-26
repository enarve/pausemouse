#![windows_subsystem = "windows"] // windows-specific: hide default terminal window

mod app;
mod model;
mod messages;
mod menu;
mod strings;
mod constants;

use iced::{Color, Result, daemon, theme};
use embed_plist::embed_info_plist;
use crate::app::{boot, update, view, subscription, title, custom_theme};

fn main() -> Result {
    println!("{}", strings::greeting());
    
    embed_info_plist!("../Info.plist");
    
    daemon(boot, update, view)
        .style(|_state, _theme| theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .subscription(subscription)
        .title(title)
        .theme(custom_theme())
        .run()
}




