use iced::Subscription;

use crate::{Three, Topic};

#[derive(Debug, Clone)]
enum Message {
    Post,
}

impl Three {
    pub fn title(&self) -> String {
        "three".into()
    }

    pub fn subscription(&self) -> Subscription<String> {
        Subscription::batch(self.follows.iter().map(Topic::subscription))
    }
}

impl Topic {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::run()
    }
}
