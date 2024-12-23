use iced::Point;

use crate::state::caches::StateCache;
use crate::state::config::Config;
use crate::state::redraw::RedrawState;
use crate::state::settings::Settings;
use crate::state::space::SpaceState;
use crate::state::system_position::{CursorPinch, SolarSystemPositionState};
use crate::state::view::ViewState;
use crate::util::time::Time;

mod caches;
mod canvas;
mod config;
mod system_position;
mod settings;
mod space;
mod redraw;
mod view;

/// Состояния программы
pub struct State {
    pub view: ViewState,
    /// Кеши
    pub cache: StateCache,
    /// Время
    pub time: Time,
    /// Настройки пользователя
    pub settings: Settings,
    /// Космические объекты
    pub space: SpaceState,
    /// Позиция Солнечной системы
    pub system_position: SolarSystemPositionState,
    /// Файл конфигурации
    pub config: Config,
    /// Повторная отрисовка
    pub redraw: RedrawState,
}

impl State {
    pub fn new() -> State {
        let config = Config::new();

        let settings = Settings::new(
            config.seconds_per_tick().value() as u16,
            config.default_scale(),
        );

        let redraw = RedrawState::new(config.ticks_between_redraws());

        let space = SpaceState::new(
            config.path_to_solar_system_values(),
            config.path_to_assets(),
            config.background_stars_count(),
            config.maximum_number_of_comets(),
        );

        let view = ViewState::new(space.planets());

        State {
            view,
            cache: StateCache::default(),
            time: Time::new(),
            settings,
            space,
            system_position: SolarSystemPositionState::default(),
            config,
            redraw,
        }
    }
}

impl State {
    fn step(&self) -> u32 {
        self.settings.scale().value() / self.config.step_formation() + 1
    }
}

impl State {
    /// Обновление при тике
    pub fn update(&mut self) {
        self.space.move_objects(self.settings.speed().value());
        self.space.remove_crashed_comets();
        self.time.add_seconds(self.settings.speed().value().value() as u16);
        if self.redraw.redraw_on_tick() {
            self.cache.clear_system();
        }
    }

    /// Изменение масштаба
    pub fn change_scale(&mut self, scale_change: i16) {
        let scale_change_factor = self.config.base_scale_change_factor()
            * (self.settings.scale().value() / self.config.step_formation() + 1);

        let scale =
            // Приближение
            if scale_change.is_positive() {
                match self.settings.scale().value()
                    .checked_sub(scale_change_factor * (scale_change as u32))
                {
                    Some(scale) if scale != 0 => scale, // Удовлетворительное изменение масштаба
                    _ => {
                        self.view.set_incorrect_scale_color();
                        return;
                    }
                }
            } else { // Уменьшение
                match self.settings.scale().value()
                    .checked_add(scale_change_factor * (-scale_change as u32))
                {
                    Some(scale) => scale, // Удовлетворительное изменение масштаба
                    _ => {
                        self.view.set_incorrect_scale_color();
                        return;
                    }
                }
            };

        self.settings.scale_mut().set(scale);

        self.cache.clear_system();
    }

    /// Изменение позиции Солнечной системы
    pub fn change_position(&mut self, position: Point) {
        self.system_position.move_center_position(position);
        self.cache.clear_system();
    }

    /// Изменение масштаба по вводу
    pub fn set_scale_from_input(&mut self, scale_string: String) {
        match scale_string.trim().parse::<u32>() {
            Ok(scale) => { // Удовлетворительное изменение масштаба
                self.settings.scale_mut().set_value(scale);
                self.cache.clear_system();
                self.view.set_correct_scale_color();
            }

            _ => self.view.set_incorrect_scale_color(),
        }

        self.settings.scale_mut().set_string_value(scale_string);
    }

    /// При нажатии на кнопку запуска симуляции
    pub fn toggle_play_pause(&mut self) {
        self.settings.toggle_running();
    }

    /// Увеличение скорости
    pub fn increase_speed(&mut self) {
        self.settings.speed_mut().set_next();
    }

    /// Уменьшение скорости
    pub fn decrease_speed(&mut self) {
        self.settings.speed_mut().set_previous();
    }

    /// Добавление новой кометы
    pub fn add_comet(&mut self) {
        self.space.add_new_comet();
    }

    /// Удаление кометы
    pub fn delete_comet(&mut self, index: u8) {
        self.space.delete_comet(index);
    }

    /// Центрирование системы на Солнце
    pub fn center_system(&mut self) {
        self.system_position.center_system_position();
        self.cache.clear_system();
    }

    /// При нажатии на левую кнопку мыши
    pub fn on_left_button_pressed(&mut self, position: Point) {
        self.system_position.set_pinch(CursorPinch::Clamped);
        self.system_position.set_cursor_position(position);
    }

    /// Когда отпускается левая кнопка мыши
    pub fn on_left_button_released(&mut self) {
        self.system_position.set_pinch(CursorPinch::NotClamped);
        self.system_position.clear_cursor_position();
    }

    /// Перезагрука симуляции
    pub fn reload(&mut self) {
        self.time.restart();
        self.settings.reload(
            self.config.seconds_per_tick().value() as u16,
            self.config.default_scale(),
        );
        self.space.reload();
        self.system_position.reload();
        self.redraw.reload();
        self.cache.clear_all();
    }

    /// Нажание на меню планет
    pub fn planets_view_toggle(&mut self) {
        self.view.toggle_planets_view();
    }

    /// Нажатие на меню комет
    pub fn comets_view_toggle(&mut self) {
        self.view.toggle_comets_view();
    }

    /// Нажатие на меню спутников планеты
    pub fn satellites_view_toggle(&mut self, planet_name: String) {
        self.view.toggle_satellites_view(planet_name);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}