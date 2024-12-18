use iced::Point;
use num_traits::{FromPrimitive, Num, ToPrimitive};

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

/// Дистанция между двумя точками
pub fn distance<T>(Point { x: x1, y: y1 }: Point<T>, Point { x: x2, y: y2 }: Point<T>) -> T
where
    T: Num + FromPrimitive + ToPrimitive + Copy,
{
    T::from_f64(
        (x2 - x1).to_f64().unwrap().hypot((y2 - y1).to_f64().unwrap())
    ).unwrap()
}