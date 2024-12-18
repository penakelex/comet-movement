use gset::Getset;
use util::data::config_data::ConfigData;
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::Seconds;

/// Данные из файла конфигурации
#[derive(Getset)]
pub struct Config {
    /// Секунд проходит каждый тик
    #[getset(get_copy, vis = "pub")]
    seconds_per_tick: Quantity<Seconds>,
    /// Базовое значение изменения масштаба
    #[getset(get_copy, vis = "pub")]
    base_scale_change_factor: u32,
    /// Путь к файлу с данными Солнечной системы
    #[getset(get, vis = "pub", ty = "&str")]
    path_to_solar_system_values: String,
    /// Значение для изменения количества точек для отрисовки при уменьшении
    #[getset(get_copy, vis = "pub")]
    step_formation: u32,
    /// Промежуток времени в наносекундах между тиками
    #[getset(get_copy, vis = "pub")]
    time_between_ticks_in_nanos: u16,
    /// Количество тиков между отрисовками
    #[getset(get_copy, vis = "pub")]
    ticks_between_redraws: u32,
    /// Начальное значение масштаба
    #[getset(get_copy, vis = "pub")]
    default_scale: u64,
    /// Количество фоновых звёзд
    #[getset(get_copy, vis = "pub")]
    background_stars_count: u16,
    /// Путь к папке с изображениями
    #[getset(get, vis = "pub", ty = "&str")]
    path_to_assets: String,
    /// Максимальное количество комет
    #[getset(get_copy, vis = "pub")]
    maximum_number_of_comets: u8,
}

impl Config {
    pub fn new() -> Self {
        ConfigData::init();
        Self {
            seconds_per_tick: Quantity::new(Seconds::new(ConfigData::get("SECONDS_PER_TICK"))),
            base_scale_change_factor: ConfigData::get("BASE_SCALE_CHANGE_FACTOR"),
            path_to_solar_system_values: ConfigData::get("PATH_TO_SOLAR_SYSTEM_VALUES"),
            step_formation: ConfigData::get("STEP_FORMATION"),
            time_between_ticks_in_nanos: ConfigData::get("BASE_TIME_BETWEEN_TICKS_IN_NANOS"),
            ticks_between_redraws: ConfigData::get("TICKS_BETWEEN_REDRAWS"),
            default_scale: ConfigData::get("DEFAULT_SCALE"),
            background_stars_count: ConfigData::get("BACKGROUND_STARS_COUNT"),
            path_to_assets: ConfigData::get("PATH_TO_ASSETS"),
            maximum_number_of_comets: ConfigData::get("MAXIMUM_NUMBER_OF_COMETS"),
        }
    }
}