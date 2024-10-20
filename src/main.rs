use std::time::Duration;
use iced::{application, Element, Fill, Subscription, Theme};
use iced::time::every;
use iced::widget::canvas;

mod objects;
mod state;
mod time;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    application(
        SolarSystem::title,
        SolarSystem::update,
        SolarSystem::view,
    )
        .subscription(SolarSystem::subscription)
        .theme(SolarSystem::theme)
        .antialiasing(true)
        .run()
}

#[derive(Default)]
struct SolarSystem {
    state: state::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick
}

impl SolarSystem {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.state.update();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        canvas(&self.state)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Moonfly
    }

    fn subscription(&self) -> Subscription<Message> {
        every(Duration::from_millis(50/3))
            .map(|_| Message::Tick)
    }

    fn title(&self) -> String {
        String::from("Comet movement")
    }
}