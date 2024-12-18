use gset::Getset;

use crate::state::settings::scale::Scale;
use crate::state::settings::speed::Speed;

pub mod speed;
pub mod scale;

/// Настройки пользователя
#[derive(Getset)]
pub struct Settings {
    /// Скорость симуляции
    #[getset(get, vis = "pub")]
    #[getset(get_mut, vis = "pub", name = "speed_mut")]
    speed: Speed,
    /// Масштаб
    #[getset(get, vis = "pub")]
    #[getset(get_mut, vis = "pub", name = "scale_mut")]
    scale: Scale,
    /// Работает ли симуляция
    #[getset(get_copy, vis = "pub")]
    is_running: bool,
}

impl Settings {
    pub fn new(base_time_between_ticks: u16, default_scale: u64) -> Self {
        Self {
            speed: Speed::new(base_time_between_ticks),
            scale: Scale::new(default_scale),
            is_running: false,
        }
    }
}

impl Settings {
    pub fn toggle_running(&mut self) {
        self.is_running = !self.is_running;
    }
}

impl Settings {
    pub fn reload(&mut self, base_time_between_ticks: u16, default_scale: u64) {
        self.speed.reload(base_time_between_ticks);
        self.scale.reload(default_scale);
        self.is_running = false;
    }
}