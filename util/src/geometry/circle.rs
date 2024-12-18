use iced::Point;

/// Структура круга (окружности)
pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Self {
            center,
            radius,
        }
    }
}

/// Проверка на возможные пересечения двух кругов (окружностей)
pub fn is_circles_have_common_points(circle1: &Circle, circle2: &Circle) -> bool {
    let distance = (circle2.center.x - circle1.center.x)
        .hypot(circle2.center.y - circle1.center.y);
    circle1.radius + circle2.radius >= distance
}