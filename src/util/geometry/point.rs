use iced::Point;
use num_traits::Num;

/// Масштабирование точки
pub fn scale_point<T: Num + Copy>(Point { x, y }: Point<T>, scale: T) -> Point<T> {
    Point::new(x / scale, y / scale)
}

/// Перенос точки в другую систему координат
pub fn translate_point<T: Num>(
    point_in_previous_system: Point<T>,
    previous_center_point: Point<T>,
    new_center_point: Point<T>,
) -> Point<T> {
    let delta_x = previous_center_point.x - new_center_point.x;
    let delta_y = previous_center_point.y - new_center_point.y;

    Point::new(point_in_previous_system.x + delta_x, point_in_previous_system.y + delta_y)
}