use gset::Getset;
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{Kilograms, Kilometers};

/// Константы объекта Солнечной системы
#[derive(Getset)]
pub struct SolarSystemObjectConsts {
    /// Масса
    #[getset(get_copy, vis = "pub")]
    mass: Quantity<Kilograms>,
    /// Начальная орбита
    #[getset(get_copy, vis = "pub")]
    initial_orbit: Quantity<Kilometers>,
    /// Радиус
    #[getset(get_copy, vis = "pub")]
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