use serde::Deserialize;

/// Данные для всех обектом Солнечной системы
#[derive(Deserialize)]
pub struct Data {
    pub sun: SunData,
    pub planets: Box<[PlanetData]>,
    pub comet: CometData,
}

/// Данные Солнца
#[derive(Deserialize)]
pub struct SunData {
    pub consts: SunConsts,
    /// Название изображения Солнца
    pub image_filename: Box<str>,
}

/// Данные констант Солнца
#[derive(Deserialize)]
pub struct SunConsts {
    /// Масса
    pub mass: f64,
    /// Радиус
    pub radius: f32,
}

/// Данные планеты
#[derive(Deserialize)]
pub struct PlanetData {
    /// Название
    pub name: Box<str>,
    pub consts: ObjectConsts,
    /// Название изображения планеты
    pub image_filename: Box<str>,
    /// Спутники планеты
    pub satellites: Box<[SatelliteData]>,
}

/// Данные спутника
#[derive(Deserialize)]
pub struct SatelliteData {
    /// Название
    pub name: Box<str>,
    pub consts: ObjectConsts,
    /// Название изображения спутника
    pub image_filename: Box<str>,
}

/// Данные констант объекта (планеты или спутника)
#[derive(Deserialize)]
pub struct ObjectConsts {
    /// Масса
    pub mass: f64,
    /// Орбита
    pub orbit: f32,
    /// Радиус
    pub radius: f32,
}

/// Данные возможных значений кометы
#[derive(Deserialize)]
pub struct CometData {
    /// Возможные скорости (минимальная и максимальная)
    pub possible_velocities: [f32; 2],
    /// Возможные массы (минимальная и максимальная)
    pub possible_masses: [f32; 2],
    /// Возможные радиусы (минимальный и максимальный)
    pub possible_radii: [f32; 2],
    /// Название изображений комет
    pub images_filenames: Box<[Box<str>]>,
}
