use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    pub sun: SunData,
    pub planets: Vec<PlanetData>,
    pub comet: CometData,
}

#[derive(Deserialize)]
pub struct SunData {
    pub consts: SunConsts,
    pub color: ColorData,
}

#[derive(Deserialize)]
pub struct SunConsts {
    pub mass: f32,
    pub radius: f32,
}

#[derive(Deserialize)]
pub struct PlanetData {
    pub name: String,
    pub consts: PlanetConsts,
    pub color: ColorData,
    pub satellites: Vec<SatelliteData>,
}

pub type PlanetConsts = ObjectConsts;

#[derive(Deserialize)]
pub struct SatelliteData {
    pub name: String,
    pub consts: SatelliteConsts,
    pub color: ColorData,
}

pub type SatelliteConsts = ObjectConsts;

#[derive(Deserialize)]
pub struct ObjectConsts {
    pub mass: f32,
    pub orbit: f32,
    pub radius: f32,
}

#[derive(Deserialize)]
pub struct ColorData {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Deserialize)]
pub struct CometData {
    pub possible_velocities: [f32; 2],
    pub possible_masses: [f32; 2],
    pub possible_radii: [f32; 2],
}