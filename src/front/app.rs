use crate::{
    front::{qrcode::Code, screen::Screen},
    {Three, Ticket, Topic},
};

use std::str::FromStr;
use std::sync::Arc;

use iced::{
    Center, Element, Task, Theme, color,
    widget::{Column, button, center, column, qr_code, row, text, text_input},
};
use iroh::{Endpoint, protocol::Router};
use iroh_gossip::net::{Gossip, GossipReceiver, GossipSender, GossipTopic};
use iroh_gossip::proto::TopicId;
use log::debug;

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

        use Message::*;
        match message {
            Init => {
                task = Task::perform(Three::iroh_init(self.secret_key.clone()), Message::InitDone)
            }
            InitDone((endpoint, gossip, router)) => {
                self.enpoint = Some(endpoint);
                // self.gossip.replace(gossip);
                self.router = Some(router);
            }
            NameChanged(name) => {
                (self.name = name);
            }
            NextPressed => {
                if let Some(screen) = self.screen.next() {
                    debug!("next screen: {screen:?}");
                    (self.screen = screen);
                }
            }
            SelectPage(screen) => {
                self.screen = screen;
            }
            Post => todo!(),
            Refresh => todo!(),
            Refreshed(_) => todo!(),
            Closed => todo!(),
            FriendInput(friend) => self.friend_input = friend,
            FriendSubmit => {
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
            SubscribeTopic(arc) => {}
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
        use Screen::*;
        let screen = match self.screen {
            Welcome => self.welcome(),
            Feed => self.feed(),
            Code => self.code(),
            Stats => todo!(),
            Friends => self.view_friends(),
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

        use Message::*;
        let screen_selector = row![
            button("Feed").on_press(SelectPage(Screen::Feed)),
            button("Code").on_press(SelectPage(Screen::Code)),
            button("Friends").on_press(SelectPage(Screen::Friends)),
            button("stat").on_press(SelectPage(Screen::Stats))
        ];

        let qr = qr_code(&self.qr_code.data);

        let content: Element<_> = match self.screen {
            Screen::Welcome => column![screen, controls]
                .max_width(540)
                .spacing(20)
                .padding(20)
                .into(),
            Screen::Code => column![screen, qr, screen_selector]
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

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
