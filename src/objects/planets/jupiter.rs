use crate::objects::{Object, PermanentObjectParameters};
use crate::objects::planets::PlanetParameters;

pub struct Jupiter;

impl PermanentObjectParameters for Jupiter {
    const ORBIT: f32 = 778_357_721.;
    const RADIUS: f32 = 71_492.;
    const CIRCULATION_PERIOD: f32 = 11.86;
    const RED: u8 = 255;
    const GREEN: u8 = 217;
    const BLUE: u8 = 198;
    const NAME: &'static str = "Jupiter";
}

impl PlanetParameters for Jupiter {
    const SATELLITES: &'static [&'static dyn Object] = &[
        &Europa, 
        &Io, 
        &Callisto, 
        &Ganymede
    ];
}

struct Europa;

impl PermanentObjectParameters for Europa {
    const ORBIT: f32 = 671_100.0;
    const RADIUS: f32 = 1560.8;
    const CIRCULATION_PERIOD: f32 = 0.0097;
    const RED: u8 = 245;
    const GREEN: u8 = 227;
    const BLUE: u8 = 217;
    const NAME: &'static str = "Europa";
}

struct Io;

impl PermanentObjectParameters for Io {
    const ORBIT: f32 = 421_800.0;
    const RADIUS: f32 = 1_821.3;
    const CIRCULATION_PERIOD: f32 = 0.005;
    const RED: u8 = 204;
    const GREEN: u8 = 204;
    const BLUE: u8 = 0;
    const NAME: &'static str = "Io";
}

struct Callisto;

impl PermanentObjectParameters for Callisto {
    const ORBIT: f32 = 1_882_700.;
    const RADIUS: f32 = 2_410.3;
    const CIRCULATION_PERIOD: f32 = 0.0457;
    const RED: u8 = 96;
    const GREEN: u8 = 96;
    const BLUE: u8 = 96;
    const NAME: &'static str = "Callisto";
}

struct Ganymede;

impl PermanentObjectParameters for Ganymede {
    const ORBIT: f32 = 1_070_400.;
    const RADIUS: f32 = 2_634.1;
    const CIRCULATION_PERIOD: f32 = 0.0196;
    const RED: u8 = 160;
    const GREEN: u8 = 160;
    const BLUE: u8 = 160;
    const NAME: &'static str = "Ganymede";
}