use crate::objects::PermanentObjectParameters;
use crate::objects::planets::PlanetParameters;

pub struct Venus;

impl PermanentObjectParameters for Venus {
    const ORBIT: f32 = 108_159_261.;
    const RADIUS: f32 = 6052.;
    const CIRCULATION_PERIOD: f32 = 0.615;
    const RED: u8 = 255;
    const GREEN: u8 = 178;
    const BLUE: u8 = 102;
    const NAME: &'static str = "Venus";
}

impl PlanetParameters for Venus {}