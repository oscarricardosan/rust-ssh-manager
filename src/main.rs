mod views;
mod lib;

use iced::{button, Element, Sandbox, Settings, Container, text_input, window};
use crate::lib::copy_ssh_pub_key_to_clipboard;

pub fn main() -> iced::Result {
    App::run(Settings{
        window: window::Settings {
            size: (400, 600),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    Main,
    SeeSshPressed,
    CopySshKey,
}

struct App {
    main_button: button::State,
    see_ssh_button: button::State,
    copy_ssh_button: button::State,

    state_app: Message,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self{
            see_ssh_button: button::State::default(),
            main_button: button::State::default(),
            copy_ssh_button: button::State::default(),

            state_app: Message::Main,
        }
    }

    fn title(&self) -> String {
        String::from("SSH Manager - Savne")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CopySshKey=> {
                copy_ssh_pub_key_to_clipboard();
                self.state_app= Message::SeeSshPressed;
            }
            _=> {
                self.state_app= message;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        let body= match self.state_app {
            Message::Main=> self.get_main_view(),
            Message::SeeSshPressed=> self.get_ssh_view(),
            _=> {self.get_main_view()}
        };

        Container::new(body)
        .max_width(600)
        .into()
    }
}

