use std::{env::var, fmt::Debug, str::FromStr};

/// Стуктура для получения данных из конфига
pub struct ConfigData;

impl ConfigData {
    pub fn init() {
        dotenv::from_path("./.env").ok();
    }
}

impl ConfigData {
    /// Получение данных из конфига по ключу
    #[inline(always)]
    pub fn get<T: FromStr>(key: &str) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        var(key).unwrap().parse::<T>().unwrap()
    }
}
