use gset::Getset;
use iced::Point;

use crate::util::geometry::circle::Circle;
use crate::util::objects::Object;
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{
    Kilograms, Kilometers,
};

/// Типаж значений объекта для вычисления гравитационной силы
pub trait GravitationalForceValues {
    /// Значения объекта для вычисления гравитационной силы
    fn gravitational_force_values(
        &self,
    ) -> ObjectGravitationalForceValues;
}

impl<T: Object> GravitationalForceValues for T {
    fn gravitational_force_values(
        &self,
    ) -> ObjectGravitationalForceValues {
        ObjectGravitationalForceValues::new(
            self.position(),
            self.mass(),
        )
    }
}

/// Значения объекта для вычисления гравитационной силы
#[derive(Getset)]
pub struct ObjectGravitationalForceValues {
    /// Позиция объекта
    #[getset(get_copy, vis = "pub")]
    position: Point<Quantity<Kilometers>>,
    /// Масса объекта
    #[getset(get_copy, vis = "pub")]
    mass: Quantity<Kilograms>,
}

impl ObjectGravitationalForceValues {
    #[inline(always)]
    pub const fn new(
        position: Point<Quantity<Kilometers>>,
        mass: Quantity<Kilograms>,
    ) -> Self {
        Self { position, mass }
    }
}

/// Типаж значений объекта для вычисления коллизий
pub trait FormValues {
    /// Значения объекта для вычисления коллизий
    fn form_values(&self) -> ObjectFormValues;
}

impl<T: Object> FormValues for T {
    fn form_values(&self) -> ObjectFormValues {
        ObjectFormValues::new(
            self.position(),
            self.radius(),
        )
    }
}

/// Значения объекта для вычисления коллизий
#[derive(Getset)]
pub struct ObjectFormValues {
    /// Позиция объекта
    #[getset(get_copy, vis = "pub")]
    position: Point<Quantity<Kilometers>>,
    /// Радиус объекта
    #[getset(get_copy, vis = "pub")]
    radius: Quantity<Kilometers>,
}

impl ObjectFormValues {
    #[inline(always)]
    pub const fn new(
        position: Point<Quantity<Kilometers>>,
        radius: Quantity<Kilometers>,
    ) -> Self {
        Self { position, radius }
    }
}

impl From<ObjectFormValues> for Circle {
    fn from(value: ObjectFormValues) -> Self {
        let center_position = value.position();
        Circle::new(
            Point::new(
                center_position.x.value(),
                center_position.y.value(),
            ),
            value.radius().value(),
        )
    }
}
