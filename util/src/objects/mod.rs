use iced::{Color, Point};
use iced::widget::image;
use num_traits::ToPrimitive;

use crate::geometry::point::scale_point;
use crate::objects::movement::ObjectMovement;
use crate::objects::values::{FormValues, GravitationalForceValues};
use crate::physics::quantities::Quantity;
use crate::physics::quantities::quantity_units::{
    Kilograms,
    Kilometers,
    KilometersPerSecond,
    Seconds,
};
use crate::physics::vector::VectorValue;

pub mod consts;
pub mod movement;
pub mod values;

/// Типаж объекта
pub trait Object: GravitationalForceValues + FormValues + ObjectScalingSizes
{
    /// Имя
    fn name(&self) -> &str;
    /// Масса
    fn mass(&self) -> Quantity<Kilograms>;
    /// Радиус
    fn radius(&self) -> Quantity<Kilometers>;
    /// Позиция
    fn position(&self) -> Point<Quantity<Kilometers>>;
    /// Изображение
    fn image(&self) -> &image::Handle;
}

/// Типаж масштабирования значений объекта
pub trait ObjectScalingSizes {
    /// Масштабирование радиуса
    fn scaled_radius(&self, scale: u64) -> f32;
    /// Масштабирование позиции
    fn scaled_position(&self, scale: u64) -> Point;
}

impl<T: Object> ObjectScalingSizes for T {
    fn scaled_radius(&self, scale: u64) -> f32 {
        self.radius().value().to_f32().unwrap() / (scale as f32)
    }

    fn scaled_position(&self, scale: u64) -> Point {
        let Point { x, y } = self.position();
        scale_point(
            Point::new(x.value().to_f32().unwrap(), y.value().to_f32().unwrap()),
            scale as f32,
        )
    }
}

/// Типаж для получения [`ObjectMovement`](ObjectMovement) объекта
pub trait ObjectMotion {
    fn movement(&self) -> &ObjectMovement;
    fn movement_mut(&mut self) -> &mut ObjectMovement;
}

/// Типаж двигающихся объектов вокруг Солнца
pub trait MovingObject: Object + ObjectMotion {
    /// Обновление позиции после движения в течение некоторого промежутка времени
    fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    );
    /// Траектория движения
    fn trajectory<'a>(&'a self, step: u32, scale: f32) -> Box<dyn Iterator<Item=Point> + 'a>;
    /// Цвет траектории
    fn trajectory_color(&self) -> Color;
}

impl<T: Object + ObjectMotion> MovingObject for T {
    fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    ) {
        self.movement_mut().update_position(velocity_change, time_interval);
    }

    fn trajectory<'a>(&'a self, step: u32, scale: f32) -> Box<dyn Iterator<Item=Point> + 'a> {
        Box::new(self.movement().stepped_scaled_trajectory(step, scale))
    }

    fn trajectory_color(&self) -> Color {
        self.movement().trajectory_color()
    }
}