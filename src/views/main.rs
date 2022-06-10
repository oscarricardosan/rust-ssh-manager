use iced::{Alignment, Button, Column, Length, Row, Text};
use crate::{App, Message};
use crate::views::general::{get_savne_title, get_welcome_message};

impl App {

    pub fn get_main_view(&mut self)-> Column<'_, Message>{

        Column::new()
        .width(Length::Fill)
        .padding(10)
        .push(
        Column::new()
            .width(Length::Fill)
            .push(
            Row::new()
                .width(Length::Fill)
                .push(
                    Row::new()
                        .width(Length::Fill)
                        .push(get_welcome_message())
                )
                .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                    Column::new()
                        .width(Length::Fill)
                        .align_items(Alignment::End)
                        .push(
                            Button::new(&mut self.see_ssh_button, Text::new("Ver Mi llave SSH").size(16))
                                .on_press(Message::SeeSshPressed)
                        )
                    )
                )
            )
        )
        .push(
            Column::new()
                .width(Length::Fill)
                .push(get_savne_title())
        )
    }
}