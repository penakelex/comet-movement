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
    #[inline(always)]
    fn name(&self) -> &str {
        "Sun"
    }

    #[inline(always)]
    fn mass(&self) -> Quantity<Kilograms> {
        self.consts.mass()
    }

    #[inline(always)]
    fn radius(&self) -> Quantity<Kilometers> {
        self.consts.radius()
    }

    #[inline(always)]
    fn position(&self) -> Point<Quantity<Kilometers>> {
        Point {
            x: Quantity::new(Kilometers::new(0.)),
            y: Quantity::new(Kilometers::new(0.)),
        }
    }

    #[inline(always)]
    fn image(&self) -> &image::Handle {
        &self.image
    }
}

impl ObjectView for Sun {
    #[inline(always)]
    fn image_view(&self) -> &image::Handle {
        &self.image
    }

    #[inline(always)]
    fn name_view(&self) -> String {
        self.name().to_string()
    }

    #[inline(always)]
    fn velocity_view(&self) -> String {
        Quantity::new(KilometersPerSecond::new(0.)).to_string()
    }
}