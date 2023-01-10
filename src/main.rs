use iced::widget::text;
use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("ClockV2.exe")
    }

    fn update(&mut self, _: Self::Message) {}

    fn view(&self) -> Element<Message> {
        text("ClockV2.exe").size(50).into()
    }
}
