use iced::Point;
use util::time::Time;

use crate::state::caches::StateCache;
use crate::state::settings::{Scale, Settings};
use crate::state::space::SpaceState;
use crate::state::system_position::{CursorPinch, SolarSystemPositionState};

mod caches;
mod canvas;
mod system_position;
mod settings;
mod space;

pub struct State {
    pub cache: StateCache,
    pub time: Time,
    pub settings: Settings,
    pub space: SpaceState,
    pub system_position: SolarSystemPositionState,
    time_before_update: u8,
}

impl State {
    pub fn new() -> State {
        State {
            cache: StateCache::default(),
            time: Time::new(),
            settings: Settings::new(),
            space: SpaceState::new(),
            system_position: SolarSystemPositionState::default(),
            time_before_update: 50,
        }
    }
}

impl State {
    pub fn speed(&self) -> u64 {
        self.settings.speed.value()
    }
}

impl State {
    pub fn update(&mut self) {
        self.space.move_objects();
        self.space.remove_crashed_comets();
        self.time.add_seconds(Settings::SECONDS_PER_TICK.value() as u16);
        if self.time_before_update == 0 {
            self.cache.clear_orbits_and_system();
            self.time_before_update = 100;
        } else {
            self.time_before_update -= 1;
        }
    }

    pub fn change_scale(&mut self, scale_change: i16) {
        let scale = if scale_change.is_positive() {
            match self.settings.scale.value
                .checked_sub((Scale::SCALE_CHANGE_FACTOR as u64) * (scale_change as u64))
            {
                Some(scale) if scale != 0 => scale,
                _ => return
            }
        } else {
            match self.settings.scale.value
                .checked_add((Scale::SCALE_CHANGE_FACTOR as u64) * (-scale_change as u64))
            {
                Some(scale) => scale,
                _ => return
            }
        };
        self.settings.scale.value = scale;

        self.settings.scale.value_string.clear();
        self.settings.scale.value_string.push_str(scale.to_string().as_str());

        self.cache.clear_orbits_and_system();
    }

    pub fn change_position(&mut self, position: Point) {
        self.system_position.move_center_position(position);
        self.cache.clear_orbits_and_system();
    }

    pub fn set_scale_from_input(&mut self, scale_string: String) {
        if let Ok(scale) = scale_string.trim().parse::<u64>() {
            self.settings.scale.value = scale;
            self.cache.clear_orbits_and_system();
        } else {
            //TODO: Возможно, поменять цвет текста на красный, например
        }

        self.settings.scale.value_string.clear();
        self.settings.scale.value_string.push_str(scale_string.as_str());
    }

    pub fn toggle_play_pause(&mut self) {
        self.settings.running = !self.settings.running
    }
    
    pub fn increase_speed(&mut self) {
        self.settings.speed = self.settings.speed.next();
    }
    
    pub fn decrease_speed(&mut self) {
        self.settings.speed = self.settings.speed.previous();
    }
    
    pub fn add_comet(&mut self) {
        self.space.add_new_comet();
    }
    
    pub fn delete_comet(&mut self, index: u8) {
        self.space.delete_comet(index);
    }
    
    pub fn center_system(&mut self) {
        self.system_position.center_system_position();
    }
    
    pub fn on_left_button_pressed(&mut self, position: Point) {
        self.system_position.set_pinch(CursorPinch::Clamped);
        self.system_position.set_cursor_position(position);
    }
    
    pub fn on_left_button_released(&mut self) {
        self.system_position.set_pinch(CursorPinch::NotClamped);
        self.system_position.clear_cursor_position();
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

