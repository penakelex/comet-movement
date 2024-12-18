use iced::{Bottom, Element, Fill};
use iced::Alignment::End;
use iced::alignment::Vertical::Center;
use iced::widget::{button, container, row, text, Text, text_input};

use crate::{Message, SolarSystem};

impl SolarSystem {
    pub fn control_panel(&self, time: String, speed: String, comets_count: u32) -> Element<Message> {
        const PAUSE: &str = "Пауза";
        const PLAY: &str = "Продолжить";

        let text = text!("Масштаб: 1 : ");

        let text_editor = text_input(
            "",
            self.state.settings.scale().value_string(),
        )
            .on_input(Message::ScaleInputChange)
            .width(90);

        let field = row![text, text_editor]
            .align_y(Center);

        let decrease_play_button = button("-")
            .width(40)
            .height(40)
            .on_press(Message::DecreaseSpeed);

        let increase_play_button = button("+")
            .width(40)
            .height(40)
            .on_press(Message::IncreaseSpeed);

        let play_button_text =
            if self.state.settings.running() { PAUSE } else { PLAY };

        let play_button = button(play_button_text)
            .width(120)
            .height(40)
            .on_press(Message::PlayPauseToggle);

        let time_text = Text::new(time);
        let speed_text = Text::new(speed);
        
        let add_comet_button = button("Add")
            .width(60)
            .height(40)
            .on_press(Message::AddComet);
        
        let delete_comet_button = button("Del")
            .width(60)
            .height(40)
            .on_press(Message::DeleteComet(0));
        
        let comets_count_text = Text::new(format!("Комет: {comets_count}"));
        
        let center_system = button("Center")
            .width(120)
            .height(40)
            .on_press(Message::CenterSystem);
        
        let reload = button("Reload")
            .width(120)
            .height(40)
            .on_press(Message::Reload);

        let panel = row![
            reload,
            center_system,
            comets_count_text,
            delete_comet_button,
            add_comet_button,
            time_text,
            decrease_play_button, 
            increase_play_button,
            speed_text,
            play_button,
            field
        ]
            .align_y(Center);

        container(panel)
            .width(Fill)
            .height(Fill)
            .align_x(End)
            .align_y(Bottom)
            .into()
    }
}