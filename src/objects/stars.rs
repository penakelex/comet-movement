use iced::Point;

#[derive(Debug)]
pub struct Star {
    pub point: Point,
    pub size: f32,
}

impl Star {
    pub fn new(point_x: f32, point_y: f32, size: f32) -> Self {
        Star {
            point: Point::new(point_x, point_y),
            size,
        }
    }
}