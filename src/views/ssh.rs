use iced::{alignment, Column, Length, Text};
use crate::{App, Message};

impl App {


    pub fn get_ssh_view(&mut self)-> Column<'_, Message>{

        Column::new()
            .push(
                Text::new("Hola")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(alignment::Horizontal::Left)
            )
    }
}