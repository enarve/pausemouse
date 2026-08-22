#![windows_subsystem = "windows"] // windows-specific: hide default terminal window

use std::time::{Instant, Duration};

use iced::widget::{container, mouse_area};
use iced::{Color, Element, Length, Size, Task, border};
use iced::window::{self, Mode};

use crate::State::{Breaking, Working};

fn main() -> iced::Result {
    let daemon_settings = iced::Settings {
        ..Default::default()
    };

    iced::daemon(boot, update, view)
        .settings(daemon_settings)
        .style(|_state, _theme| iced::theme::Style {
            background_color: Color::TRANSPARENT, // Removes default canvas layer
            text_color: Color::WHITE,
        })
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Initialized(window::Id),
    WindowDragged,
    Tick(Instant),
    OpenSettings,
    CloseSettings,
}

struct Config {
    work_duration: Duration,
    break_duration: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_duration: Duration::from_secs(15),
            break_duration: Duration::from_secs(10)
        }
    }
}

struct SettingsInputBuffer {
    work_duration_input: String,
    break_duration_input: String,
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

enum State {
    Working(Instant),
    Breaking(Instant)
}

struct Windows {
    main: Option<window::Id>,
    settings: Option<window::Id>,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            main: None,
            settings: None,
        }
    }
}

struct App {
    state: State,
    windows: Windows,
    config: Config,
    settings_input_buffer: Option<SettingsInputBuffer>,
}

impl Default for App {
    fn default() -> Self {
        let state = Working(Instant::now());
        App {
            state,
            windows: Windows::default(),
            config: Config::default(),
            settings_input_buffer: Option::None,
        }
    }
}

fn boot() -> (App, Task<Message>) {
    let app = App::default();
    
    let main_window_settings = window::Settings {
        size: Size::new(250.0, 100.0),
        position: window::Position::Centered,
        decorations: false,
        transparent: true,
        resizable: false,
        visible: false,
        level: window::Level::AlwaysOnTop,
        ..Default::default()
    };
    let (main_id, open_task) = window::open(main_window_settings);
    let initial_task = open_task.map(move |_| Message::Initialized(main_id));
    
    (app, initial_task)
}

fn update(app: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::Initialized(main_id) => {
            app.windows.main = Some(main_id);
            Task::none()
        },
        Message::WindowDragged => {
            window::drag(app.windows.main.unwrap())
        },
        Message::Tick(now) => {
            let task = match app.state {
                
                Working(start_time) => {
                    let elapsed = now.duration_since(start_time);
                    println!("{:?}", elapsed);
                    
                    if elapsed > app.config.work_duration {
                        // show mouse
                        app.state = Breaking(Instant::now());
                        window::set_mode(app.windows.main.unwrap(), Mode::Windowed)
                    } else {
                        Task::none()
                    }
                    
                },
                Breaking(start_time) => {
                    let elapsed = now.duration_since(start_time);
                    if elapsed > app.config.break_duration {
                        // hide mouse
                        app.state = Working(Instant::now());
                        window::set_mode(app.windows.main.unwrap(), Mode::Hidden)
                    } else {
                        Task::none()
                    }
                    
                }
            };
            task
        },
        Message::OpenSettings => {
            Task::none()
        },
        Message::CloseSettings => {
            Task::none()
        }
    }
}

fn view(app: &App, window_id: window::Id) -> Element<'_, Message> {
    if Some(window_id) == app.windows.main {
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
    } else {
        "Settings"
            .into()
    }
}

fn subscription(_app: &App) -> iced::Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(Message::Tick)
}
