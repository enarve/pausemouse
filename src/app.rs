use std::time::{Instant, Duration};

use iced::{Color, Element, Length, Size, Task, border, Alignment, Font, Background, Border};
use iced::widget::{container, mouse_area, text, column, row, progress_bar, text_input};
use iced::window::{self, Mode};
use iced::theme::{Palette, Theme};

use crate::strings;
use crate::constants::{MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT, SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT};
use crate::model::{State, Windows, SettingsInputBuffer};
use crate::config::Config;
use crate::messages::Message;
use crate::menu::Menu;
use self::State::{Breaking, Working};

pub struct App {
    state: State,
    windows: Windows,
    config: Config,
    menu: Menu,
    settings_input_buffer: SettingsInputBuffer,
}

impl Default for App {
    fn default() -> Self {
        let state = Working(Instant::now());
        App {
            state,
            windows: Windows::default(),
            config: Config::default(),
            menu: Menu::default(),
            settings_input_buffer: SettingsInputBuffer::default(),
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
        size: Size::new(MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT),
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
    String::from(strings::TITLE)
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
                    size: Size::new(SETTINGS_WINDOW_WIDTH, SETTINGS_WINDOW_HEIGHT),
                    resizable: false,
                    exit_on_close_request: false,
                    ..Default::default()
                };
                let (settings_id, open_task) = window::open(settings_window_settings);
                app.windows.settings = Some(settings_id);
                app.settings_input_buffer = SettingsInputBuffer { work_duration_input: format!("{}", app.config.work_duration.as_secs_f32()), break_duration_input: format!("{}", app.config.break_duration.as_secs_f32()) };
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
                    return Task::batch(vec![
                        window::close(id),
                        Task::done(Message::BreakDurationSubmitted),
                        Task::done(Message::WorkDurationSubmitted),
                    ]);
                }
            }
            Task::none()
        },
        Message::MenuClicked(clicked_id) => {
            if Some(&clicked_id) == app.menu.settings_menu_id.as_ref() {
                return update(app, Message::OpenSettings);
            } else if Some(&clicked_id) == app.menu.quit_menu_id.as_ref() {
                println!("{}", strings::FAREWELL);
                std::process::exit(0);
            }
            Task::none()
        },
        Message::WorkDurationChanged(string) => {
            app.settings_input_buffer.work_duration_input = string;
            Task::none()
        },
        Message::BreakDurationChanged(string) => {
            app.settings_input_buffer.break_duration_input = string;
            Task::none()
        },
        Message::BreakDurationSubmitted => {
            let string = &app.settings_input_buffer.break_duration_input;
            match string.parse::<f32>() {
                Err(_) => (),
                Ok(value) => {
                    if value > 0.0 {
                        app.config.break_duration = Duration::from_secs_f32(value);
                        _ = app.config.write();
                    }
                }
            }
            Task::none()
        },
        Message::WorkDurationSubmitted => {
            let string = &app.settings_input_buffer.work_duration_input;
            match string.parse::<f32>() {
                Err(_) => (),
                Ok(value) => {
                    if value > 0.0 {
                        app.config.work_duration = Duration::from_secs_f32(value);
                        _ = app.config.write();
                    }
                }
            }
            Task::none()
        }
    }
}

pub fn view(app: &App, window_id: window::Id) -> Element<'_, Message> {
    if Some(window_id) == app.windows.main {
        let now = Instant::now();
        let elapsed = if let State::Breaking(time) = app.state { now.duration_since(time).as_secs_f32() } else { 0.0 };
        mouse_area(
            container(
                column![
                    text(strings::BREAK),
                    text(strings::ASCII_ART)
                        .font(Font::MONOSPACE),
                    container (progress_bar(0.0..=app.config.break_duration.as_secs_f32(), elapsed)
                        .girth(15.0)
                        .style(|_theme| {
                                progress_bar::Style {
                                    bar: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 1.0)), 
                                    background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.2)), 
                                    border: Border::default(), 
                                }
                            })
                    )    
                        .padding([20.0, 20.0]),
                    text(strings::next_break(app.config.work_duration))
                ]
                .align_x(Alignment::Center)
            )
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
        container(
            column![
                container(
                text(strings::SETTINGS_TITLE)
                ).padding(5).center_x(Length::Fill),
                container(
                    column![
                        row![
                            text(strings::WORK_SETTING_LABEL),
                            text_input(&format!("{:?}", app.config.work_duration), &app.settings_input_buffer.work_duration_input)
                                .on_input(Message::WorkDurationChanged)
                                .on_submit(Message::WorkDurationSubmitted)
                        ]
                        .align_y(Alignment::Center)
                        .spacing(5),
                        row![
                            text(strings::BREAK_SETTING_LABEL),
                            text_input(&format!("{:?}", app.config.break_duration), &app.settings_input_buffer.break_duration_input)
                                .on_input(Message::BreakDurationChanged)
                                .on_submit(Message::BreakDurationSubmitted)
                        ]
                        .align_y(Alignment::Center)
                        .spacing(5)
                    ]
                    .spacing(10)
                )
                .padding([10, 20])
            ]
        )
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

pub fn custom_theme() -> Theme {
    Theme::custom(
        "Pausemouse",
        Palette {
            primary: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
            
            ..Palette::DARK
        },
    )
}