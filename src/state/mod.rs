use iced::widget::canvas::Cache;
use rand::{Rng, thread_rng};

use crate::objects::PLANETS;
use crate::objects::planets::Planet;
use crate::objects::stars::Star;
use crate::objects::sun::Sun;
use crate::time::Time;

mod canvas;

pub struct State {
    pub background_cache: Cache,
    pub sun_cache: Cache,
    pub planets_caches: Vec<Cache>,
    pub time: Time,
    // pub speed: u16, //TODO
    pub scale: u32,
    pub stars: Vec<Star>,
    pub planets: &'static [&'static dyn Planet],
    pub sun: Sun
}

impl State {
    pub fn new() -> State {
        let mut planets_caches = Vec::with_capacity(PLANETS.len());
        
        for _ in 0..planets_caches.capacity() {
            planets_caches.push(Cache::new());
        }
        
        State {
            background_cache: Cache::new(),
            sun_cache: Cache::new(),
            planets_caches,
            time: Time::new(),
            //speed: 1,
            scale: 15_000_000,
            stars: Self::generate_stars(),
            planets: &PLANETS,
            sun: Sun
        }
    }

    pub fn update(&mut self) {
        self.sun_cache.clear();
        self.planets_caches.iter().for_each(|cache| cache.clear());
        for _ in 0..1_000 {
            self.time.add_minute();
        }
    }

    fn generate_stars() -> Vec<Star> {
        let mut rng = thread_rng();

        (0..500_u16)
            .map(|_| {
                let point_x = rng.gen_range(-1.0..=1.0);
                let point_y = rng.gen_range(-1.0..=1.0);
                let size = rng.gen_range(0.5..1.0);
                Star::new(point_x, point_y, size)
            })
            .collect()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}