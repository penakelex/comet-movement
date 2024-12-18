use iced::Point;
use iced::widget::image;
use util::data::solar_system_data::SunData;
use util::objects::consts::SolarSystemObjectConsts;
use util::objects::Object;
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{Kilograms, Kilometers};


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