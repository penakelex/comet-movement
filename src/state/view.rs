use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use gset::Getset;

use iced::Color;

use crate::objects::planet::Planet;
use crate::util::objects::Object;

/// Состояние UI
#[derive(Getset)]
pub struct ViewState {
    /// Открыты ли меню спутников планет
    satellites_views_opened: HashMap<String, bool>,
    /// Открыта ли меню планет
    #[getset(get_copy, vis = "pub")]
    planets_views_opened: bool,
    /// Открыта ли меню комет
    #[getset(get_copy, vis = "pub")]
    comets_views_opened: bool,
    /// Цвет текста ввода масштаба
    #[getset(get_copy, vis = "pub")]
    scale_input_color: Color,
}

impl ViewState {
    pub fn new(planets: &[Rc<RefCell<Planet>>]) -> Self {
        let planets_satellites_view = planets.iter()
            .map(|planet| (planet.borrow().name().to_string(), false))
            .collect::<HashMap<String, bool>>();

        Self {
            satellites_views_opened: planets_satellites_view,
            planets_views_opened: true,
            comets_views_opened: true,
            scale_input_color: Color::WHITE,
        }
    }
}

impl ViewState {
    #[inline(always)]
    pub fn is_satellites_opened(&self, planet_name: String) -> bool {
        self.satellites_views_opened[&planet_name]
    }

    pub fn toggle_satellites_view(&mut self, planet_name: String) {
        self.satellites_views_opened.entry(planet_name)
            .and_modify(|opened| *opened = !*opened);
    }
}

impl ViewState {
    pub fn toggle_planets_view(&mut self) {
        self.planets_views_opened = !self.planets_views_opened;
    }
}

impl ViewState {
    pub fn toggle_comets_view(&mut self) {
        self.comets_views_opened = !self.comets_views_opened;
    }
}

impl ViewState {
    pub fn set_incorrect_scale_color(&mut self) {
        self.scale_input_color = Color::from_rgb8(255, 0, 0)
    }

    pub fn set_correct_scale_color(&mut self) {
        self.scale_input_color = Color::WHITE;
    }
}