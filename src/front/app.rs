use crate::front::qrcode::Code;
use crate::front::screen::Screen;
use crate::{Three, Ticket, Topic};
use std::str::FromStr;
use std::sync::Arc;

use eframe::egui::accesskit::NodeId;
use iced::Renderer;
use iced::{
    Center, Element, Task, Theme, color,
    widget::{Column, QRCode, button, center, column, image, qr_code, row, text, text_input},
};

use ::image::imageops;
use iroh::{Endpoint, protocol::Router};
use iroh_gossip::net::{Gossip, GossipReceiver, GossipSender, GossipTopic};
use iroh_gossip::proto::TopicId;
use log::debug;

use super::qrcode;

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    InitDone((Endpoint, Gossip, Router)),
    SelectPage(Screen),
    FriendInput(String),
    FriendSubmit,
    SubscribeTopic(Arc<(GossipSender, GossipReceiver)>),
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
                // self.gossip.replace(gossip);
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
            Message::FriendInput(friend) => self.friend_input = friend,
            Message::FriendSubmit => {
                let Ticket { topic, peers } = Ticket::from_str(&self.friend_input).unwrap();
                self.friend_input.clear();

                // let peer_ids = peers.iter().map(|p| p.node_id).collect();

                // task = Task::perform(
                //     self.gossip.unwrap().subscribe_and_join(topic, peer_ids),
                //     |gt| {
                //         let (sender, receiver) = gt.unwrap().split();
                //         Message::SubscribeTopic(Arc::new((sender, receiver)))
                //     },
                // )
            }
            Message::SubscribeTopic(arc) => {}
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
        let screen = match self.screen {
            Screen::Welcome => self.welcome(),
            Screen::Feed => self.feed(),
            Screen::Code => self.code(),
            Screen::Stats => todo!(),
            Screen::Friends => self.view_friends(),
        };

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

        let screen_selector = row![
            button("Feed").on_press(Message::SelectPage(Screen::Feed)),
            button("code").on_press(Message::SelectPage(Screen::Code)),
            button("Friends").on_press(Message::SelectPage(Screen::Friends)),
            button("stat").on_press(Message::SelectPage(Screen::Stats))
        ];

        let qr = qr_code(&Code::new("test_str").qr_code).into();

        let code = Code::new("test_str");
        let content: Element<_> = match self.screen {
            Screen::Welcome => column![screen, controls]
                .max_width(540)
                .spacing(20)
                .padding(20)
                .into(),
            Screen::Code => column![screen, qr, controls]
                .max_width(540)
                .spacing(20)
                .padding(20)
                .into(),
            _ => column![screen, controls, screen_selector]
                .max_width(540)
                .spacing(20)
                .padding(20)
                .into(),
        };

        content
    }

    fn welcome(&self) -> Column<Message> {
        Self::container("Welcome!").push("Welcome to Three")
    }

    fn feed(&self) -> Column<Message> {
        Self::container("Feed").push("TODO: Feed")
    }

    fn code(&self) -> Column<'_, Message> {
        Self::container("Code").push("TODO: Code")
    }

    fn container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20)
    }

    fn view_friends(&self) -> Column<Message> {
        let friends = self
            .topics
            .iter()
            .map(|f| format!("{f:?}"))
            .chain([self.friend_input.clone()])
            .collect::<Vec<String>>()
            .join(", ");

        let friends_input = text_input("friendless", &friends)
            .on_input(Message::FriendInput)
            .on_submit(Message::FriendSubmit);
        let submit = button("add").on_press(Message::FriendSubmit);

        column![friends_input, submit]
    }
}
