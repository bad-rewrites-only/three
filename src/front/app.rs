use crate::Three;
use crate::front::screen::Screen;

use std::sync::Arc;

use iced::{
    Center, Element, Task, color,
    widget::{Column, button, center, column, row, text, text_input},
};
use iroh::{Endpoint, protocol::Router};
use iroh_gossip::net::Gossip;
use log::debug;

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    InitDone((Endpoint, Gossip, Router)),
    SelectPage(Screen),
    NameChanged(String),
    NextPressed,
    Post,
    Refresh,
    Refreshed(Arc<Result<(), anyhow::Error>>),
    Closed,
}

impl Three {
    pub fn title(&self) -> String {
        format!("{} - Iced", self.screen)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        debug!("update: {message:?}");

        let mut task = Task::none();

        match message {
            Message::Init => {
                task = Task::perform(Three::iroh_init(self.secret_key.clone()), Message::InitDone)
            }
            Message::InitDone((endpoint, gossip, router)) => {
                self.enpoint = Some(endpoint);
                self.gossip = Some(gossip);
                self.router = Some(router);
            }
            Message::NameChanged(name) => {
                (self.name = name);
            }
            Message::NextPressed => {
                if let Some(screen) = self.screen.next() {
                    debug!("next screen: {screen:?}");
                    (self.screen = screen);
                }
            }
            Message::SelectPage(screen) => {
                self.screen = screen;
            }
            Message::Post => todo!(),
            Message::Refresh => todo!(),
            Message::Refreshed(_) => todo!(),
            Message::Closed => todo!(),
        }

        task
    }

    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::Welcome => true,
            Screen::Feed => false,
            _ => true,
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
            Screen::Code => todo!(),
            Screen::Stats => todo!(),
            Screen::Friends => self.view_friends(),
        };

        let page_selector = row![
            button("feed").on_press(Message::SelectPage(Screen::Feed)),
            button("code").on_press(Message::SelectPage(Screen::Code)),
            button("fren").on_press(Message::SelectPage(Screen::Friends)),
            button("stat").on_press(Message::SelectPage(Screen::Stats))
        ];

        let content: Element<_> = column![screen, controls, page_selector]
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

    fn view_friends(&self) -> Column<Message> {
        let friends_input = text_input(
            "friendless",
            self.friends
                .iter()
                .map(|f| format!("{f:?}"))
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        );
        // if self.friends.is_empty() {
        //     text("waiting for friends...")
        // } else {
        // }

        column![friends_input]
    }
}
