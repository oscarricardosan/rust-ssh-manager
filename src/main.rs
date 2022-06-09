mod views;

use iced::{button, Element, Sandbox, Settings, Container};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Main,
    SeeSshPressed,
}

struct App {
    see_ssh_button: button::State,
    state_app: Message,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self{
            state_app: Message::Main,
            see_ssh_button: button::State::default()
        }
    }

    fn title(&self) -> String {
        String::from("SSH Manager - Savne")
    }

    fn update(&mut self, message: Message) {
        self.state_app= message;
    }

    fn view(&mut self) -> Element<Message> {

        let body= match self.state_app {
            Message::Main=> self.get_main_view(),
            Message::SeeSshPressed=> self.get_ssh_view()
        };

        Container::new(body)
        .into()
    }
}

