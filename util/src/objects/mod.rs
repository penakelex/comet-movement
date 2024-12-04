use getset::CopyGetters;
use iced::{Color, Point};
use num_traits::ToPrimitive;

use crate::geometry::circle::Circle;
use crate::geometry::point::scale_point;
use crate::physics::quantities::Quantity;
use crate::physics::quantities::quantity_units::{
    Kilograms,
    Kilometers,
    KilometersPerSecond,
    Seconds,
};
use crate::physics::vector::VectorValue;

pub mod movement;

pub trait Object
where
    Self: GravitationalForceValues + FormValues + ObjectScalingSizes,
{
    fn name(&self) -> &str;

    fn mass(&self) -> Quantity<Kilograms>;

    fn radius(&self) -> Quantity<Kilometers>;

    fn position(&self) -> Point<Quantity<Kilometers>>;

    fn color(&self) -> Color;
}

pub trait GravitationalForceValues {
    fn gravitational_force_values(&self) -> ObjectGravitationalForceValues;
}

impl<T: Object> GravitationalForceValues for T {
    fn gravitational_force_values(&self) -> ObjectGravitationalForceValues {
        ObjectGravitationalForceValues::new(self.position(), self.mass())
    }
}

pub trait FormValues {
    fn form_values(&self) -> ObjectFormValues;
}

impl<T: Object> FormValues for T {
    fn form_values(&self) -> ObjectFormValues {
        ObjectFormValues::new(self.position(), self.radius())
    }
}

pub trait ObjectScalingSizes {
    fn scaled_radius(&self, scale: u64) -> f32;
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

pub trait ObjectPositionUpdate: Object {
    fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    );
}

pub trait ObjectTrajectory: Object {
    fn trajectory<'a>(&'a self, step: u16, scale: f32) -> Box<dyn Iterator<Item=Point> + 'a>;
}

#[derive(CopyGetters)]
pub struct SolarSystemObjectConsts {
    #[getset(get_copy = "pub")]
    mass: Quantity<Kilograms>,
    #[getset(get_copy = "pub")]
    initial_orbit: Quantity<Kilometers>,
    #[getset(get_copy = "pub")]
    radius: Quantity<Kilometers>,
}

impl SolarSystemObjectConsts {
    pub fn new(mass: f64, initial_orbit: f32, radius: f32) -> Self {
        Self {
            mass: Quantity::new(Kilograms::new(mass)),
            initial_orbit: Quantity::new(Kilometers::new(initial_orbit)),
            radius: Quantity::new(Kilometers::new(radius)),
        }
    }
}


#[derive(CopyGetters)]
pub struct ObjectGravitationalForceValues {
    #[getset(get_copy = "pub")]
    position: Point<Quantity<Kilometers>>,
    #[getset(get_copy = "pub")]
    mass: Quantity<Kilograms>,
}

impl ObjectGravitationalForceValues {
    pub const fn new(position: Point<Quantity<Kilometers>>, mass: Quantity<Kilograms>) -> Self {
        Self {
            position,
            mass,
        }
    }
}

#[derive(CopyGetters)]
pub struct ObjectFormValues {
    #[getset(get_copy = "pub")]
    position: Point<Quantity<Kilometers>>,
    #[getset(get_copy = "pub")]
    radius: Quantity<Kilometers>,
}

impl ObjectFormValues {
    pub const fn new(position: Point<Quantity<Kilometers>>, radius: Quantity<Kilometers>) -> Self {
        Self {
            position,
            radius,
        }
    }
}

impl From<ObjectFormValues> for Circle {
    fn from(value: ObjectFormValues) -> Self {
        let center_position = value.position();

        Circle::new(
            Point::new(center_position.x.value() as f32, center_position.y.value() as f32),
            value.radius().value() as f32,
        )
    }
}

