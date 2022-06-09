use iced::{Alignment, alignment, Button, Column, Length, Row, Text};
use crate::{App, Message};

impl App {

    pub fn get_main_view(&mut self)-> Column<'_, Message>{

        Column::new()
            .width(Length::Fill)
            .padding(10)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Row::new()
                            .width(Length::Fill)
                            .push(
                                Column::new()
                                    .width(Length::Fill)
                                    .align_items(Alignment::Start)
                                    .push(
                                        Text::new("Bienvenido a SSH Manager")
                                            .width(Length::Fill)
                                            .size(16)
                                            .color([0.5, 0.5, 0.5])
                                            .horizontal_alignment(alignment::Horizontal::Left)
                                    )
                            )
                    ).push(
                    Row::new()
                        .width(Length::Fill)
                        .push(
                            Column::new()
                                .width(Length::Fill)
                                .align_items(Alignment::Center)
                                .push(
                                    Text::new("Savne SAS")
                                        .size(20)
                                        .color([0.34, 0.46, 0.88])
                                        .horizontal_alignment(alignment::Horizontal::Center)
                                )
                        )
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
    }
}