use tray_icon::{TrayIconBuilder, menu};
use crate::messages::Message;

pub struct Menu {
    pub tray_handle: Option<tray_icon::TrayIcon>,
    pub settings_menu_id: Option<tray_icon::menu::MenuId>,
    pub quit_menu_id: Option<tray_icon::menu::MenuId>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            tray_handle: None,
            settings_menu_id: None,
            quit_menu_id: None,
        }
    }
}

impl Menu {

    pub fn subscription() -> iced::Subscription<Message> {
        iced::Subscription::run(|| {
            iced::stream::channel(100, |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
                let receiver = tray_icon::menu::MenuEvent::receiver();
                let (tx, mut rx) = iced::futures::channel::mpsc::channel::<Message>(10);
                std::thread::spawn(move || {
                    loop {
                        if let Ok(menu_event) = receiver.recv() {
                            let _ = tx.clone().try_send(Message::MenuClicked(menu_event.id));
                        }
                    }
                });
                use iced::futures::StreamExt;
                while let Some(msg) = rx.next().await {
                    let _ = output.try_send(msg);
                }
            })
        })
    }
    
    pub fn init() -> (tray_icon::TrayIcon, tray_icon::menu::MenuId, tray_icon::menu::MenuId) {
        let tray_menu = menu::Menu::new();
        let settings_item = menu::MenuItem::new("Settings...", true, None);
        let quit_item = menu::MenuItem::new("Quit Pausemouse", true, None);
        
        let settings_id = settings_item.id().clone();
        let quit_id = quit_item.id().clone();
        
        tray_menu.append_items(&[&settings_item, &quit_item]).unwrap();
    
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Pausemouse")
            // TODO: Add icon
            // .with_icon(some_icon)
            .build()
            .unwrap();
    
        (tray, settings_id, quit_id)
    }

}