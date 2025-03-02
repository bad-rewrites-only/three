use std::sync::Arc;

use iced::Task;
use iced::{
    Center, Element, color,
    widget::{Column, button, center, column, text, text_input},
};
use iroh::protocol::Router;
use log::debug;

use crate::Three;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    Feed,
}

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    InitDone(Router),
    NameChanged(String),
    NextPressed,
    Post,
    Refresh,
    Refreshed(Arc<Result<(), anyhow::Error>>),
    Closed,
}

impl Three {
    pub fn title(&self) -> String {
        let screen = match self.screen {
            Screen::Welcome => "Welcome",
            Screen::Feed => "Feed",
        };

        format!("{} - Iced", screen)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        debug!("update: {message:?}");
        match message {
            Message::Init => {
                Task::perform(Three::iroh_init(self.secret_key.clone()), Message::InitDone)
            }
            Message::InitDone(_) => Task::none(),
            Message::NameChanged(name) => (self.name = name).into(),
            //Message::NextPressed => {
            //    if let Some(screen) = self.screen.next() {
            //        self.screen = screen;
            //    }
            //}
            Message::Post => todo!(),
            // Message::Startup => Task::perform(),
            Message::Refresh => {
                todo!()
            }
            _ => todo!(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let text_input = text_input("Enter name...", &self.name)
            .on_input(Message::NameChanged)
            .padding(10)
            .size(20);
        let next = button("Next").on_press(Message::NextPressed);

        let content = column![text_input, next]
            .spacing(20)
            .padding(20)
            .max_width(600);
        center(content).into()
    }
}
