use crate::objects::PermanentObjectParameters;
use crate::objects::planets::PlanetParameters;

pub struct Mars;

impl PermanentObjectParameters for Mars {
    const ORBIT: f32 = 227_987_155.;
    const RADIUS: f32 = 3396.;
    const CIRCULATION_PERIOD: f32 = 1.881;
    const RED: u8 = 212;
    const GREEN: u8 = 91;
    const BLUE: u8 = 31;
    const NAME: &'static str = "Mars";
}

impl PlanetParameters for Mars {}