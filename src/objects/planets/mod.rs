use crate::objects::Object;

pub mod mercury;
pub mod venus;
pub mod earth;
pub mod mars;
pub mod jupiter;
pub mod saturn;
pub mod uranus;
pub mod neptune;

pub trait Planet: Object {
    fn satellites(&self) -> &'static [&'static dyn Object];
}

pub trait PlanetParameters {
    const SATELLITES: &'static [&'static dyn Object] = &[];
}

impl<T: PlanetParameters + Object> Planet for T {
    fn satellites(&self) -> &'static [&'static dyn Object] {
        Self::SATELLITES
    }
}