use crate::objects::{Object, PermanentObjectParameters};
use crate::objects::planets::PlanetParameters;

pub struct Neptune;

impl PermanentObjectParameters for Neptune {
    const ORBIT: f32 = 4_498_407_972.;
    const RADIUS: f32 = 24_764.;
    const CIRCULATION_PERIOD: f32 = 164.8;
    const RED: u8 = 0;
    const GREEN: u8 = 128;
    const BLUE: u8 = 255;
    const NAME: &'static str = "Neptune";
}

struct Triton;

impl PermanentObjectParameters for Triton {
    const ORBIT: f32 = 354_759.;
    const RADIUS: f32 = 1353.4;
    const CIRCULATION_PERIOD: f32 = -0.0161;
    const RED: u8 = 224;
    const GREEN: u8 = 224;
    const BLUE: u8 = 224;
    const NAME: &'static str = "Triton";
}

impl PlanetParameters for Neptune {
    const SATELLITES: &'static [&'static dyn Object] = &[&Triton];
}