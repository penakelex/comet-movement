use std::time::Duration;

use iced::{
    Element, Fill, Point, Subscription, Theme, application,
    time::every,
    widget::{canvas, stack},
};

use crate::state::State;

mod objects;
mod state;
mod util;
mod views;

pub fn main() -> iced::Result {
    application(
        SolarSystem::new,
        SolarSystem::update,
        SolarSystem::view,
    )
    .title(SolarSystem::title)
    .subscription(SolarSystem::subscription)
    .theme(SolarSystem::theme)
    .antialiasing(true)
    .run()
}

struct SolarSystem {
    state: State,
}

// TODO: Разбить на несколько перечислений (возможно, сделать ещё и несколько обработчиков)
#[derive(Debug, Clone)]
enum Message {
    PositionChange(Point),
    ScaleChange(i16),
    ScaleInputChange(String),
    LeftButtonPressed(Point),
    LeftButtonReleased,
    PlayPauseToggle,
    PlanetsViewToggle,
    CometsViewToggle,
    SatellitesViewToggle(String),
    Tick,
    IncreaseSpeed,
    DecreaseSpeed,
    AddComet,
    DeleteComet(u8),
    CenterSystem,
    Reload,
}

impl SolarSystem {
    fn new() -> Self {
        Self { state: State::new() }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PositionChange(position) => {
                self.state.change_position(position)
            }

            Message::ScaleChange(scale_change) => {
                self.state.change_scale(scale_change)
            }

            Message::Tick => self.state.update(),

            Message::ScaleInputChange(scale_string) => self
                .state
                .set_scale_from_input(scale_string),

            Message::PlayPauseToggle => {
                self.state.toggle_play_pause()
            }

            Message::IncreaseSpeed => {
                self.state.increase_speed()
            }

            Message::DecreaseSpeed => {
                self.state.decrease_speed()
            }

            Message::AddComet => self.state.add_comet(),

            Message::DeleteComet(index) => {
                self.state.delete_comet(index)
            }

            Message::CenterSystem => {
                self.state.center_system()
            }

            Message::LeftButtonPressed(position) => {
                self.state.on_left_button_pressed(position)
            }

            Message::LeftButtonReleased => {
                self.state.on_left_button_released()
            }

            Message::Reload => self.state.reload(),

            Message::PlanetsViewToggle => {
                self.state.planets_view_toggle()
            }

            Message::CometsViewToggle => {
                self.state.comets_view_toggle()
            }

            Message::SatellitesViewToggle(planet_name) => {
                self.state
                    .satellites_view_toggle(planet_name)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let solar_system =
            canvas(&self.state).width(Fill).height(Fill);

        let panel = self.control_panel();

        stack![solar_system, panel]
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Moonfly
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.state.settings.is_running() {
            every(Duration::from_nanos(
                self.state
                    .config
                    .time_between_ticks_in_nanos()
                    as u64,
            ))
            .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn title(&self) -> String {
        String::from("Comet movement")
    }
}
