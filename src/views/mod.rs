use iced::{Background, Border, Color, Element, Fill};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text, text_input};

use crate::{Message, SolarSystem};

mod objects;

impl SolarSystem {
    /// Главная панель управления симуляцией
    pub fn control_panel(&self) -> Element<Message> {
        let panels = column![
            self.time_panel(),
            row![
                self.play_toggle_panel(),
                self.speed_panel(),
            ]
                .spacing(4),
            self.reload_and_center_panel(),
            self.scale_panel(),
        ]
            .align_x(Horizontal::Right)
            .spacing(4);

        container(
            row![panels, self.objects_panel()]
                .align_y(Vertical::Bottom)
                .spacing(2)
        )
            .align_right(Fill)
            .align_bottom(Fill)
            .into()
    }
}

impl SolarSystem {
    /// Отображение времени
    fn time_panel(&self) -> Element<Message> {
        text(self.state.time.to_string())
            .size(20)
            .color(Color::WHITE)
            .into()
    }
}

impl SolarSystem {
    /// Регулирование масштаба
    fn scale_panel(&self) -> Element<Message> {
        let scale_text: Element<_> = text!("1:")
            .size(18)
            .color(Color::WHITE)
            .into();

        let scale_input: Element<_> = text_input("", self.state.settings.scale().value_string())
            .size(18)
            .style(|_, _| {
                text_input::Style {
                    border: Border {
                        width: 0.,
                        ..Border::default()
                    },
                    background: Background::Color(Self::background_color()),
                    icon: Color::WHITE,
                    placeholder: Color::WHITE,
                    value: Color::WHITE,
                    selection: Self::background_color(),
                }
            })
            .on_input(Message::ScaleInputChange)
            .into();
        
        container(
            row![scale_text, scale_input]
                .align_y(Vertical::Center)
        )
            .style(|_| {
                container::Style {
                    background: Some(
                        Background::Color(Self::background_color())
                    ),
                    ..Default::default()
                }
            })
            .width(200)
            .into()
    }
}

impl SolarSystem {
    /// Запуск/остановка симуляции
    fn play_toggle_panel(&self) -> Element<Message> {
        let is_running = self.state.settings.is_running();

        button(if is_running { "Стоп" } else { "Старт" })
            .width(if is_running { 60 } else { 70 })
            .height(40)
            .on_press(Message::PlayPauseToggle)
            .into()
    }
}

impl SolarSystem {
    /// Изменение и отображение скорости
    fn speed_panel(&self) -> Element<Message> {
        let increase_speed_button: Element<_> = button("+")
            .width(40)
            .height(40)
            .on_press(Message::IncreaseSpeed)
            .into();

        let decrease_speed_button: Element<_> = button("-")
            .width(40)
            .height(40)
            .on_press(Message::DecreaseSpeed)
            .into();
        
        let speed_value: Element<_> = text(self.state.settings.speed().to_string())
            .size(18)
            .width(40)
            .color(Color::WHITE)
            .center()
            .into();

        row![decrease_speed_button, increase_speed_button, speed_value]
            .spacing(4)
            .align_y(Vertical::Center)
            .into()
    }
}

impl SolarSystem {
    /// Перезагрузка симуляции и центрирование системы на Солнце
    fn reload_and_center_panel(&self) -> Element<Message> {
        let reload_button: Element<_> = button("Перезагрузить")
            .width(130)
            .height(40)
            .on_press(Message::Reload)
            .into();

        let center_button: Element<_> = button("Центр")
            .width(70)
            .height(40)
            .on_press(Message::CenterSystem)
            .into();

        row![reload_button, center_button]
            .spacing(4)
            .into()
    }
}

impl SolarSystem {
    /// Фоновый цвет
    fn background_color() -> Color {
        Color::from_rgba8(0, 0, 128, 0.4)
    }
}