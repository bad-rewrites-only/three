use iced::Center;
use iced::widget::{Column, button, column, text};

pub fn run() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
//use crate::{Three, Topic, subscribe_loop};
//use iced::Subscription;
//
//#[derive(Debug, Clone)]
//enum Message {
//    Post,
//}
//
//impl Three {
//    pub fn title(&self) -> String {
//        "three".into()
//    }
//
//    pub fn subscription(&self) -> Subscription<String> {
//        Subscription::batch(self.follows.iter().map(Topic::subscription))
//    }
//}
//
//impl Topic {
//    fn subscription(&self) -> Subscription<Message> {
//        Subscription::run(subscribe_loop(self.receiver.unwrap()))
//    }
//}
