use crate::objects::PermanentObjectParameters;

pub struct Sun;

impl PermanentObjectParameters for Sun {
    const ORBIT: f32 = 0.;
    const RADIUS: f32 = 696_000_000.;
    const CIRCULATION_PERIOD: f32 = 0.0695;
    const RED: u8 = 255;
    const GREEN: u8 = 153;
    const BLUE: u8 = 51;
    const NAME: &'static str = "Sun";
}