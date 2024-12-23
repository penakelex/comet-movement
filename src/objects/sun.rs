use iced::Point;
use iced::widget::image;
use crate::util::data::solar_system_data::SunData;
use crate::util::objects::consts::SolarSystemObjectConsts;
use crate::util::objects::{Object, ObjectView};
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{Kilograms, Kilometers, KilometersPerSecond};

pub struct Sun {
    consts: SolarSystemObjectConsts,
    image: image::Handle,
}

impl Sun {
    pub fn new(data: SunData, path_to_images: &str) -> Self {
        let SunData { consts, image_filename} = data;
        
        Self {
            consts: SolarSystemObjectConsts::new(consts.mass, 0., consts.radius),
            image: image::Handle::from_path(format!("{path_to_images}/{image_filename}")),
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

    fn image(&self) -> &image::Handle {
        &self.image
    }
}

impl ObjectView for Sun {
    fn image_view(&self) -> &image::Handle {
        &self.image
    }

    fn name_view(&self) -> String {
        self.name().to_string()
    }

    fn velocity_view(&self) -> String {
        Quantity::new(KilometersPerSecond::new(0.)).to_string()
    }
}