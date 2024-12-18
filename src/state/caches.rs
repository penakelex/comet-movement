use gset::Getset;
use iced::widget::canvas::Cache;

/// Кеш для отрисовки Солнечной системы
#[derive(Default, Getset)]
pub struct StateCache {
    /// Кеш для фоновых звёзд
    #[getset(get, vis = "pub")]
    stars: Cache,
    /// Кеш для объектов солнечной системы
    #[getset(get, vis = "pub")]
    system: Cache,
}

impl StateCache {
    pub fn clear_system(&self) {
        self.system.clear();
    }
    
    pub fn clear_all(&self) {
        self.stars.clear();
        self.system.clear();
    }
}