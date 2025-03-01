use crate::Three;

#[derive(Debug, Clone)]
enum Message {
    Post,
}

impl Three {
    pub fn title(&self) -> String {
        "three".into()
    }
}
