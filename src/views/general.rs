use iced::{alignment, Alignment, Column, Length, Row, Text};
use crate::{Message};

    pub fn get_welcome_message()-> Column<'static, Message>{

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

    }
    pub fn get_savne_title()-> Column<'static, Message>{

        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Row::new().height(Length::Units(20))
            )
            .push(
                Text::new("Savne SAS")
                    .size(20)
                    .color([0.34, 0.46, 0.88])
                    .horizontal_alignment(alignment::Horizontal::Center)
            )
    }
