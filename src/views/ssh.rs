use std::fs;
use std::process::Command;
use iced::{alignment, Alignment, Button, Column, Length, Row, Text};
use crate::{App, Message};
use crate::lib::get_ssh_pub_key;
use crate::views::general::{get_savne_title, get_welcome_message};

impl App {


    pub fn get_ssh_view(&mut self)-> Column<'_, Message>{

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
                            Button::new(&mut self.main_button, Text::new("Volver").size(16))
                                .on_press(Message::Main)
                        )
                    )
                )
            )
        )
        .push(
        Column::new()
            .width(Length::Fill)
            .height(Length::Units(50))
        )
        .push(
        Column::new()
            .width(Length::Fill)
            .push(
            Row::new()
                .width(Length::Fill)
                .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                    Text::new("Tu llave SSH es: ")
                        .width(Length::Fill)
                        .size(20)
                        .color([0.5, 0.5, 0.5])
                        .horizontal_alignment(alignment::Horizontal::Left)
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
                        Button::new(&mut self.copy_ssh_button, Text::new("Copiar").size(12))
                            .on_press(Message::CopySshKey)
                        )
                    )
                )
            )
        )
        .push(
        Column::new()
            .width(Length::Fill)
            .push(
            Text::new(get_ssh_pub_key())
                .width(Length::Fill)
                .size(21)
                .horizontal_alignment(alignment::Horizontal::Left)
            )
        )
        .push(
            Column::new()
                .width(Length::Fill)
                .push(get_savne_title())
        )
    }
}