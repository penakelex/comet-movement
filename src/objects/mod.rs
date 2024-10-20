use std::f32::consts::PI;

use iced::Color;

use crate::objects::planets::{
    earth::Earth,
    jupiter::Jupiter,
    mars::Mars,
    mercury::Mercury,
    neptune::Neptune,
    Planet,
    saturn::Saturn,
    uranus::Uranus,
    venus::Venus,
};
use crate::time::Time;

pub mod stars;
pub mod sun;
pub mod planets;

pub const PLANETS: &[&'static dyn Planet] = &[
    &Mercury,
    &Venus,
    &Earth,
    &Mars,
    &Jupiter,
    &Saturn,
    &Uranus,
    &Neptune
];

pub trait Object {
    fn orbit_radius_scaled(&self, scale: u32) -> f32;
    fn object_radius_scaled(&self, scale: u32) -> f32;
    fn circulation_period(&self) -> f32;
    fn angle_of_rotation(&self, time: &Time) -> f32;
    fn color(&self) -> Color;
    fn name(&self) -> & str;
}

pub trait PermanentObjectParameters {
    const ORBIT: f32;
    const RADIUS: f32;
    /**The period of rotation around a larger body 
        (if a planet, then around the Sun, if a satellite, then around its own planet)*/
    const CIRCULATION_PERIOD: f32;
    const RED: u8;
    const GREEN: u8;
    const BLUE: u8;
    const NAME: &'static str;
}

impl<T: PermanentObjectParameters> Object for T {
    fn orbit_radius_scaled(&self, scale: u32) -> f32 {
        Self::ORBIT / (scale as f32)
    }

    fn object_radius_scaled(&self, scale: u32) -> f32 {
        Self::RADIUS / (scale as f32)
    }

    fn circulation_period(&self) -> f32 {
        Self::CIRCULATION_PERIOD
    }

    fn angle_of_rotation(&self, time: &Time) -> f32 {
        (2. * PI / (Self::CIRCULATION_PERIOD * 365. * 24. * 60.)) 
            * (time.minutes() as f32) % (2. * PI)
    }

    fn color(&self) -> Color {
        Color::from_rgb8(Self::RED, Self::GREEN, Self::BLUE)
    }

    fn name(&self) -> &str {
        Self::NAME
    }
}