use iced::widget::canvas::Cache;

#[derive(Default)]
pub struct StateCache {
    pub stars: Cache,
    pub orbits: Cache,
    pub system: Cache,
}

impl StateCache {
    pub fn clear_orbits_and_system(&self) {
        self.orbits.clear();
        self.system.clear();
    }
}