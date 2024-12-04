use std::time::Duration;

use iced::{application, Element, Fill, Point, Subscription, Theme};
use iced::time::every;
use iced::widget::{canvas, stack};

use crate::state::State;

mod objects;
mod state;
mod views;

pub fn main() -> iced::Result {
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
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    PositionChange(Point),
    ScaleChange(i16),
    ScaleInputChange(String),
    LeftButtonPressed(Point),
    LeftButtonReleased,
    PlayPauseToggle,
    Tick,
    IncreaseSpeed,
    DecreaseSpeed,
    AddComet,
    DeleteComet(u8),
    CenterSystem,
}

impl SolarSystem {
    fn update(&mut self, message: Message) {
        match message {
            Message::PositionChange(position) => self.state.change_position(position),

            Message::ScaleChange(scale_change) => self.state.change_scale(scale_change),

            Message::Tick => self.state.update(),

            Message::ScaleInputChange(scale_string) =>
                self.state.set_scale_from_input(scale_string),

            Message::PlayPauseToggle => self.state.toggle_play_pause(),

            Message::IncreaseSpeed => self.state.increase_speed(),

            Message::DecreaseSpeed => self.state.decrease_speed(),

            Message::AddComet => self.state.add_comet(),

            Message::DeleteComet(index) => self.state.delete_comet(index),

            Message::CenterSystem => self.state.center_system(),
            
            Message::LeftButtonPressed(position) => self.state.on_left_button_pressed(position),
            
            Message::LeftButtonReleased => self.state.on_left_button_released(),
        }
    }

    fn view(&self) -> Element<Message> {
        let solar_system = canvas(&self.state)
            .width(Fill)
            .height(Fill);

        let panel = self.control_panel(
            self.state.time.to_string(),
            self.state.settings.speed.to_string(),
            self.state.space.comets_count() as u32,
        );

        stack![
            solar_system,
            panel
        ]
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Moonfly
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.state.settings.running {
            every(Duration::from_micros(self.state.speed()))
                .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn title(&self) -> String {
        String::from("Comet movement")
    }
}
