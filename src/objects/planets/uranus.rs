use crate::objects::{Object, PermanentObjectParameters};
use crate::objects::planets::PlanetParameters;

pub struct Uranus;

impl PermanentObjectParameters for Uranus {
    const ORBIT: f32 = 2_870_783_139.;
    const RADIUS: f32 = 25_559.;
    const CIRCULATION_PERIOD: f32 = 84.01;
    const RED: u8 = 102;
    const GREEN: u8 = 255;
    const BLUE: u8 = 255;
    const NAME: &'static str = "Uranus";
}

impl PlanetParameters for Uranus {
    const SATELLITES: &'static [&'static dyn Object] = &[&Oberon, &Titania];
}

struct Oberon;

impl PermanentObjectParameters for Oberon {
    const ORBIT: f32 = 583_520.;
    const RADIUS: f32 = 761.4;
    const CIRCULATION_PERIOD: f32 = 0.0368;
    const RED: u8 = 233;
    const GREEN: u8 = 229;
    const BLUE: u8 = 218;
    const NAME: &'static str = "Oberon";
}

struct Titania;

impl PermanentObjectParameters for Titania {
    const ORBIT: f32 = 436_300.;
    const RADIUS: f32 = 788.4;
    const CIRCULATION_PERIOD: f32 = 0.0238;
    const RED: u8 = 128;
    const GREEN: u8 = 128;
    const BLUE: u8 = 128;
    const NAME: &'static str = "Titania";
}

