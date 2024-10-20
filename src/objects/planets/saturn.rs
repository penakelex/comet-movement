use crate::objects::{Object, PermanentObjectParameters};
use crate::objects::planets::PlanetParameters;

pub struct Saturn;

impl PermanentObjectParameters for Saturn {
    const ORBIT: f32 = 1_426_714_893.;
    const RADIUS: f32 = 60_268.;
    const CIRCULATION_PERIOD: f32 = 29.46;
    const RED: u8 = 249;
    const GREEN: u8 = 229;
    const BLUE: u8 = 184;
    const NAME: &'static str = "Saturn";
}

impl PlanetParameters for Saturn {
    const SATELLITES: &'static [&'static dyn Object] =  &[&Rhea, &Titan];
}

struct Rhea;

impl PermanentObjectParameters for Rhea {
    const ORBIT: f32 = 527_100.;
    const RADIUS: f32 = 764.;
    const CIRCULATION_PERIOD: f32 = 0.0123;
    const RED: u8 = 220;
    const GREEN: u8 = 220;
    const BLUE: u8 = 220;
    const NAME: &'static str = "Rhea";
}

struct Titan;

impl PermanentObjectParameters for Titan {
    const ORBIT: f32 = 1_221_870.0;
    const RADIUS: f32 = 2576.;
    const CIRCULATION_PERIOD: f32 = 0.0437;
    const RED: u8 = 255;
    const GREEN: u8 = 239;
    const BLUE: u8 = 199;
    const NAME: &'static str = "Titan";
}