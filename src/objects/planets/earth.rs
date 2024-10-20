use crate::objects::{Object, PermanentObjectParameters};
use crate::objects::planets::PlanetParameters;

pub struct Earth;

impl PermanentObjectParameters for Earth {
    const ORBIT: f32 = 149_597_871.;
    const RADIUS: f32 = 6378.;
    const CIRCULATION_PERIOD: f32 = 1.;
    const RED: u8 = 26;
    const GREEN: u8 = 194;
    const BLUE: u8 = 23;
    const NAME: &'static str = "Earth";
}

struct Moon;

impl PermanentObjectParameters for Moon {
    const ORBIT: f32 = 390.0;
    const RADIUS: f32 = 1737.1;
    const CIRCULATION_PERIOD: f32 = 0.0748;
    const RED: u8 = 166;
    const GREEN: u8 = 171;
    const BLUE: u8 = 166;
    const NAME: &'static str = "Moon";
}

impl PlanetParameters for Earth {
    const SATELLITES: &'static [&'static dyn Object] = &[&Moon];
}