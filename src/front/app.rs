use crate::Three;
use crate::front::screen::Screen;

use std::sync::Arc;

use iced::{
    Center, Element, Task, color,
    widget::{Column, button, center, column, row, text, text_input},
};
use iroh::protocol::Router;
use log::debug;

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
            Message::NameChanged(name) => {
                (self.name = name);
                ().into()
            }
            Message::NextPressed => {
                if let Some(screen) = self.screen.next() {
                    debug!("next screen: {screen:?}");
                    (self.screen = screen);
                }
                Task::none()
            }

            //Message::Post => todo!(),
            //Message::Startup => Task::perform(),
            //Message::Refresh => todo!(),
            _ => Task::none(),
        }
    }

    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::Welcome => true,
            Screen::Feed => false,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let controls = row![]
            .push_maybe((self.screen == Screen::Welcome).then(|| {
                text_input("Enter name...", &self.name)
                    .on_input(Message::NameChanged)
                    .padding(10)
                    .size(20)
            }))
            .push_maybe(
                self.can_continue()
                    .then(|| button("Next").on_press(Message::NextPressed)),
            );

        let screen = match self.screen {
            Screen::Welcome => self.welcome(),
            Screen::Feed => self.feed(),
        };

        let content: Element<_> = column![screen, controls,]
            .max_width(540)
            .spacing(20)
            .padding(20)
            .into();

        content
    }

    fn welcome(&self) -> Column<Message> {
        Self::container("Welcome!").push("Welcome to Three")
    }

    fn feed(&self) -> Column<Message> {
        Self::container("Feed").push("TODO: Feed")
    }

    fn container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20)
    }
}
