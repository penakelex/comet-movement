use iced::{Color, Point};

use util::file_data::SunData;
use util::objects::{Object, SolarSystemObjectConsts};
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{Kilograms, Kilometers};

pub struct Sun {
    consts: SolarSystemObjectConsts,
    color: Color,
}

impl Sun {
    pub fn new(data: SunData) -> Self {
        let SunData { consts, color} = data;
        
        let consts = SolarSystemObjectConsts::new(
            consts.mass as f64,
            0.,
            consts.radius,
        );

        Self {
            consts,
            color: Color::from_rgb8(color.red, color.green, color.blue),
        }
    }
}

impl Object for Sun {
    fn name(&self) -> &str {
        "Sun"
    }

    fn mass(&self) -> Quantity<Kilograms> {
        self.consts.mass()
    }

    fn radius(&self) -> Quantity<Kilometers> {
        self.consts.radius()
    }

    fn position(&self) -> Point<Quantity<Kilometers>> {
        Point {
            x: Quantity::new(Kilometers::new(0.)),
            y: Quantity::new(Kilometers::new(0.)),
        }
    }

    fn color(&self) -> Color {
        self.color
    }
}