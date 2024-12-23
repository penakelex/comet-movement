use iced::Point;
use rand::Rng;
use rand::rngs::ThreadRng;

/// Фоновая звезда
pub struct Star {
    /// Позиция относительно размеров окна
    pub relative_point: Point,
    /// Размер
    pub size: f32,
}

impl Star {
    /// Генерация новой звезды
    pub fn generate(rng: &mut ThreadRng) -> Self {
        // Позиция относительно размеров окна
        let relative_point = Point::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );
        
        Star {
            relative_point,
            size: rng.gen_range(0.5..1.0),
        }
    }
}