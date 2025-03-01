use crate::{Three, Topic, subscribe_loop};
use iced::Subscription;

#[derive(Debug, Clone)]
enum Message {
    Post,
}

impl Three {
    pub fn title(&self) -> String {
        "three".into()
    }
}
