use iced::{
    Center, Element, color,
    widget::{Column, button, column, text},
};
use iced::{Subscription, Task};

use crate::Three;

#[derive(Debug)]
pub enum Message {
    Post,
    Refreshed(Result<(), anyhow::Error>),
    Closed,
}

impl Three {
    pub fn title(&self) -> String {
        "Three".into()
    }

    pub fn update(&mut self, message: Message)
    // -> Task<Message>
    {
        match message {
            Message::Post => todo!(),
            // Message::Startup => Task::perform(),
            _ => todo!(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        text(self.name.clone())
            .size(20)
            .color(color!(0x0000ff))
            .into()
    }
}
