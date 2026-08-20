#![windows_subsystem = "windows"] // windows-specific: hide default terminal window

use std::println;
use std::time::{Instant, Duration};

use iced::widget::{container, mouse_area};
use iced::{Color, Element, Length, Size, Task, border};
use iced::window::{self, Mode};

use crate::State::{Breaking, Working};

const WORK_DURATION: Duration = Duration::from_secs(15); // short period for development
const BREAK_DURATION: Duration = Duration::from_secs(10);

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size::new(250.0, 100.0),
        position: window::Position::Centered,
        decorations: false,
        transparent: true,
        resizable: false,
        visible: false,
        level: window::Level::AlwaysOnTop,
        ..Default::default()
    };
    
    iced::application(App::default, update, view)
        .window(window_settings)
        .style(|_state, _theme| iced::theme::Style {
            background_color: Color::TRANSPARENT, // Removes default canvas layer
            text_color: Color::WHITE,
        })
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    WindowDragged,
    Tick(Instant)
}

enum State {
    Working(Instant),
    Breaking(Instant)
}

struct App {
    state: State
}

impl Default for App {
    fn default() -> Self {
        let state = Working(Instant::now());
        App { state }
    }
}

fn update(app: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::WindowDragged => {
            window::oldest().and_then(window::drag)
        },
        Message::Tick(now) => {
            let task = match app.state {
                
                Working(start_time) => {
                    let elapsed = now.duration_since(start_time);
                    
                    if elapsed > WORK_DURATION {
                        // show mouse
                        app.state = Breaking(Instant::now());
                        window::oldest().and_then(|id| window::set_mode(id, Mode::Windowed))
                            
                    } else {
                        println!("{:?} Working...", elapsed);
                        Task::none()
                    }
                    
                },
                Breaking(start_time) => {
                    let elapsed = now.duration_since(start_time);
                    if elapsed > BREAK_DURATION {
                        // hide mouse
                        app.state = Working(Instant::now());
                        window::oldest().and_then(|id| window::set_mode(id, Mode::Hidden))
                    } else {
                        Task::none()
                    }
                    
                }
            };
            task
        }
    }
}

fn view(_app: &App) -> Element<'_, Message> {
    mouse_area(
        container("Time for a pause!")
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgba(0.15, 0.15, 0.15, 0.9).into()), 
            text_color: Some(Color::WHITE),
            border: border::Border {
                radius: border::Radius::from(15.0), 
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Default::default()
        })
    )
    .on_press(Message::WindowDragged)
    .into()
}

fn subscription(_app: &App) -> iced::Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(Message::Tick)
}