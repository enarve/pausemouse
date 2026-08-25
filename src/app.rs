use std::time::{Instant, Duration};

use iced::{Color, Element, Length, Size, Task, border};
use iced::widget::{container, mouse_area};
use iced::window::{self, Mode};

use crate::constants;
use crate::model::{State, Windows, Config, SettingsInputBuffer};
use crate::messages::Message;
use crate::menu::Menu;
use self::State::{Breaking, Working};

pub struct App {
    state: State,
    windows: Windows,
    config: Config,
    menu: Menu,
    _settings_input_buffer: Option<SettingsInputBuffer>,
}

impl Default for App {
    fn default() -> Self {
        let state = Working(Instant::now());
        App {
            state,
            windows: Windows::default(),
            config: Config::default(),
            menu: Menu::default(),
            _settings_input_buffer: Option::None,
        }
    }
}

pub fn boot() -> (App, Task<Message>) {
    let mut app = App::default();
    
    let (tray_handle, settings_id, quit_id)  = Menu::init();
    app.menu.tray_handle = Some(tray_handle);
    app.menu.settings_menu_id = Some(settings_id);
    app.menu.quit_menu_id = Some(quit_id);
    
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

pub fn title(_app: &App, _window_id: window::Id) -> String {
    String::from("Pausemouse")
}

pub fn update(app: &mut App, message: Message) -> iced::Task<Message> {
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
            if let None = app.windows.settings {
                let settings_window_settings = window::Settings {
                    size: Size::new(300.0, 200.0),
                    resizable: false,
                    exit_on_close_request: false,
                    ..Default::default()
                };
                let (settings_id, open_task) = window::open(settings_window_settings);
                app.windows.settings = Some(settings_id);
                open_task.discard()
            } else {
                let id = app.windows.settings.unwrap();
                let maximize = window::minimize(id, false);
                let focus = window::gain_focus(id);
                iced::Task::batch(vec![maximize, focus])
            }
        },
        Message::WindowClosed(id) => {
            if let Some(settings_id) = app.windows.settings {
                if id == settings_id {
                    app.windows.settings = None;
                    return window::close(id); 
                }
            }
            Task::none()
        },
        Message::MenuClicked(clicked_id) => {
            if Some(&clicked_id) == app.menu.settings_menu_id.as_ref() {
                return update(app, Message::OpenSettings);
            } else if Some(&clicked_id) == app.menu.quit_menu_id.as_ref() {
                println!("{}", constants::FAREWELL_MESSAGE);
                std::process::exit(0);
            }
            Task::none()
        },
    }
}

pub fn view(app: &App, window_id: window::Id) -> Element<'_, Message> {
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
        container("Settings")
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
            .into()
    }
}

pub fn subscription(_app: &App) -> iced::Subscription<Message> {
    let ticker = iced::time::every(Duration::from_secs(1)).map(Message::Tick);
    let close = window::close_requests().map(Message::WindowClosed);
    let menu = Menu::subscription();
    iced::Subscription::batch(vec![ticker, close, menu])
}