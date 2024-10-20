use crate::objects::PermanentObjectParameters;
use crate::objects::planets::PlanetParameters;

pub struct Mercury;

impl PermanentObjectParameters for Mercury {
    const ORBIT: f32 = 57_894_376.;
    const RADIUS: f32 = 2440.;
    const CIRCULATION_PERIOD: f32 = 0.241;
    const RED: u8 = 102;
    const GREEN: u8 = 102;
    const BLUE: u8 = 255;
    const NAME: &'static str = "Mercury";
}

impl PlanetParameters for Mercury {}