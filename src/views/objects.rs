use std::cell::RefCell;
use std::rc::Rc;

use iced::alignment::Vertical;
use iced::widget::container::Style;
use iced::widget::{
    Column, button, column, container, image, row,
    scrollable, text,
};
use iced::{Background, Border, Color, Element, Fill};

use crate::objects::comet::Comet;
use crate::objects::planet::Planet;
use crate::util::objects::{Object, ObjectView};
use crate::{Message, SolarSystem};

impl SolarSystem {
    /// Панель с информацией объектами и управлением ими
    pub fn objects_panel(&self) -> Element<'_, Message> {
        let sun = self.sun();
        let planets = self.planets();
        let comets = self.comets();

        scrollable(
            column![sun, planets, comets].spacing(20),
        )
        .width(240)
        .into()
    }
}

impl SolarSystem {
    /// Карточка Солнца
    fn sun(&self) -> Element<'_, Message> {
        self.object_card(self.state.space.sun().clone())
    }
}

impl SolarSystem {
    /// Набор карт комет
    fn comets(&self) -> Element<'_, Message> {
        let is_opened =
            self.state.view.comets_views_opened();

        let comets_naming: Element<_> = button(text!(
            "Кометы({count}) {opened}",
            opened = if is_opened { '▲' } else { '▼' },
            count = self.state.space.comets_count()
        ))
        .width(Fill)
        .height(Fill)
        .on_press(Message::CometsViewToggle)
        .into();

        let add_comet_button: Element<_> = button("+")
            .width(40)
            .height(Fill)
            .on_press(Message::AddComet)
            .into();

        let comets_naming_table: Element<_> = container(
            row![comets_naming, add_comet_button]
                .spacing(1),
        )
        .style(|_| Self::container_background_style())
        .width(Fill)
        .height(30)
        .into();

        if !is_opened {
            return comets_naming_table;
        }

        let comets_view = Column::with_children(
            self.state
                .space
                .comets()
                .iter()
                .enumerate()
                .map(|(index, comet)| {
                    self.comet_card(
                        comet.clone(),
                        index as u8,
                    )
                }),
        );

        container(
            column![comets_naming_table, comets_view]
                .spacing(2),
        )
        .width(Fill)
        .into()
    }
}

impl SolarSystem {
    /// Набор карт планет
    fn planets(&self) -> Element<'_, Message> {
        let is_opened =
            self.state.view.planets_views_opened();

        let planets_naming: Element<_> = container(
            button(text(format!(
                "Планеты {}",
                if is_opened { '▲' } else { '▼' }
            )))
            .width(Fill)
            .height(Fill)
            .on_press(Message::PlanetsViewToggle),
        )
        .style(|_| Self::container_background_style())
        .width(Fill)
        .height(30)
        .into();

        if !is_opened {
            return planets_naming;
        }

        let planets_view = Column::with_children(
            self.state.space.planets().iter().map(
                |planet| {
                    self.planet_with_satellite(
                        planet.clone(),
                    )
                },
            ),
        )
        .spacing(20);

        column![planets_naming, planets_view]
            .width(Fill)
            .into()
    }

    /// Карта планеты и карты её спутников
    fn planet_with_satellite(
        &self,
        planet: Rc<RefCell<Planet>>,
    ) -> Element<'_, Message> {
        let planet_card = self.object_card(planet.clone());

        let satellites_count =
            planet.borrow().satellites().len();

        if satellites_count == 0 {
            return planet_card;
        }

        let is_opened =
            self.state.view.is_satellites_opened(
                planet.borrow().name().to_string(),
            );

        let satellites_naming: Element<_> = container(
            button(text(format!(
                "Спутники {planet_name} {opened}",
                planet_name = planet.borrow().name(),
                opened =
                    if is_opened { '▲' } else { '▼' }
            )))
            .width(Fill)
            .height(Fill)
            .on_press(
                Message::SatellitesViewToggle(
                    planet.borrow().name().to_string(),
                ),
            ),
        )
        .center_x(Fill)
        .center_y(Fill)
        .height(30)
        .into();

        if !is_opened {
            return container(
                iced::widget::column![
                    planet_card,
                    satellites_naming
                ]
                .spacing(2),
            )
            .width(Fill)
            .into();
        }

        let mut satellites =
            Column::with_capacity(satellites_count)
                .spacing(2);

        for satellite in planet.borrow().satellites() {
            satellites = satellites
                .push(self.object_card(satellite.clone()));
        }

        container(
            column![
                planet_card,
                satellites_naming,
                satellites
            ]
            .spacing(2),
        )
        .width(Fill)
        .into()
    }
}

impl SolarSystem {
    /// Карта объекта
    fn object_card(
        &self,
        object: Rc<RefCell<dyn ObjectView>>,
    ) -> Element<'_, Message> {
        let (image, name, velocity) =
            self.object_attributes(object.clone());

        let description: Element<_> =
            container(column![name, velocity].spacing(2))
                .center_y(Fill)
                .padding(4)
                .into();

        container(row![image, description])
            .style(|_| Self::container_background_style())
            .width(Fill)
            .height(100)
            .into()
    }

    /// Карта кометы
    fn comet_card(
        &self,
        comet: Rc<RefCell<Comet>>,
        comet_index: u8,
    ) -> Element<'_, Message> {
        let (image, name, velocity) =
            self.object_attributes(comet.clone());

        let remove_comet_button: Element<_> = button("-")
            .width(30)
            .height(30)
            .on_press(Message::DeleteComet(comet_index))
            .into();

        let comet_naming = row![name, remove_comet_button]
            .align_y(Vertical::Center)
            .spacing(2);

        let description: Element<_> = container(
            column![comet_naming, velocity].spacing(2),
        )
        .center_y(Fill)
        .padding(4)
        .into();

        container(row![image, description])
            .style(|_| Self::container_background_style())
            .width(Fill)
            .height(100)
            .into()
    }

    /// Аттрибуты объекта (картинка, название, скорость)
    fn object_attributes(
        &self,
        object: Rc<RefCell<dyn ObjectView>>,
    ) -> (
        Element<'_, Message>,
        Element<'_, Message>,
        Element<'_, Message>,
    ) {
        let object = object.borrow();

        let image: Element<_> = container(Element::from(
            image(object.image_view())
                .height(100)
                .width(100),
        ))
        .padding(4)
        .center_y(Fill)
        .into();

        let name: Element<_> = text(object.name_view())
            .size(18)
            .color(Color::WHITE)
            .into();

        let velocity: Element<_> =
            text(object.velocity_view())
                .size(14)
                .color(Color::WHITE)
                .into();

        (image, name, velocity)
    }
}

impl SolarSystem {
    /// Фоновый цвет контейнера
    fn container_background_style() -> Style {
        Style {
            background: Some(Background::Color(
                Self::background_color(),
            )),
            border: Border {
                color: Color::WHITE,
                width: 0.5,
                ..Border::default()
            },
            ..Style::default()
        }
    }
}
