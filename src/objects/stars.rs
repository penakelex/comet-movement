use gset::Getset;
use iced::Point;
use rand::{RngExt, rngs::ThreadRng};

/// Фоновая звезда
#[derive(Getset)]
pub struct Star {
    /// Позиция относительно размеров окна
    #[getset(get_copy, vis = "pub")]
    relative_point: Point,
    /// Размер
    #[getset(get_copy, vis = "pub")]
    size: f32,
}

impl Star {
    /// Генерация новой звезды
    pub fn generate(rng: &mut ThreadRng) -> Self {
        // Позиция относительно размеров окна
        let relative_point = Point::new(
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
        );
        
        Star {
            relative_point,
            size: rng.random_range(0.5..1.0),
        }
    }
}