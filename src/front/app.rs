use crate::Three;
use iced::{
    Center, Element, color,
    widget::{Column, button, column, text},
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Post,
}

impl Three {
    pub fn title(&self) -> String {
        "Three".into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Post => todo!(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        text(self.name.clone())
            .size(20)
            .color(color!(0x0000ff))
            .into()
    }
}
