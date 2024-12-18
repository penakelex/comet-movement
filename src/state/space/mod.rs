use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::rc::{Rc, Weak};

use gset::Getset;
use iced::Color;
use rand::{Rng, thread_rng};
use serde_json::from_reader;
use tap::{Tap, TapOptional};
use util::data::solar_system_data::{Data, PlanetData};
use util::geometry::circle::{Circle, is_circles_have_common_points};
use util::objects::{MovingObject, Object, ObjectMotion};
use util::objects::values::FormValues;
use util::physics::formulas::{orbital_velocity, vector_of_velocity_change};
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{Kilograms, Kilometers, KilometersPerSecond, Seconds};
use util::physics::vector::VectorValue;

use crate::objects::comet::CometPossibleValues;
use crate::objects::planet::Planet;
use crate::objects::satellite::Satellite;
use crate::objects::stars::Star;
use crate::objects::sun::Sun;
use crate::state::space::comets::CometsState;

pub mod comets;

/// Состояние космических объектов
#[derive(Getset)]
pub struct SpaceState {
    /// Фоновые звёзды
    stars: Vec<Star>,
    /// Планеты
    planets: Vec<Rc<RefCell<Planet>>>,
    /// Солнце
    #[getset(get, vis = "pub")]
    sun: Rc<RefCell<Sun>>,
    /// Все объекты
    all_objects: Vec<Weak<RefCell<dyn Object>>>,
    /// Двигающиеся объекты
    moving_objects: Vec<Weak<RefCell<dyn MovingObject>>>,
    /// Состояние комет
    comets: CometsState,
}

impl SpaceState {
    pub fn new(
        path_to_solar_system_values: &str,
        path_to_images: &str,
        background_stars_count: u16,
        maximum_number_of_comets: u8,
    ) -> Self {
        let (
            sun,
            planets,
            comet_possible_values
        ) = Self::get_solar_system_data(
            path_to_solar_system_values,
            path_to_images,
            maximum_number_of_comets,
        );

        let sun = Rc::new(RefCell::new(sun));

        let planets = planets.into_iter()
            .map(|planet| Rc::new(RefCell::new(planet)))
            .collect::<Vec<_>>();

        let all_objects = Self::get_all_objects(&sun, planets.as_slice());

        let moving_objects = Self::get_moving_objects(planets.as_slice());

        Self {
            stars: Self::generate_stars(background_stars_count),
            planets,
            sun,
            all_objects,
            moving_objects,
            comets: CometsState::new(comet_possible_values, maximum_number_of_comets),
        }
    }

    /// Генерация фоновых звёзд
    fn generate_stars(stars_count: u16) -> Vec<Star> {
        let mut rng = thread_rng();
        (0..stars_count).map(|_| Star::generate(&mut rng)).collect()
    }

    /// Составление массива всех объектов
    fn get_all_objects(
        sun: &Rc<RefCell<Sun>>,
        planets: &[Rc<RefCell<Planet>>],
    ) -> Vec<Weak<RefCell<dyn Object>>> {
        Vec::<Weak<RefCell<dyn Object>>>::new().tap_mut(|all_objects| {
            let sun: Rc<RefCell<dyn Object>> = sun.clone();
            all_objects.push(Rc::downgrade(&sun));
            
            planets.iter().for_each(|planet| {
                let planet_as_object: Rc<RefCell<dyn Object>> = planet.clone();
                all_objects.push(Rc::downgrade(&planet_as_object));
                
                planet.borrow().satellites().iter().for_each(|satellite| {
                    let satellite: Rc<RefCell<dyn Object>> = satellite.clone();
                    all_objects.push(Rc::downgrade(&satellite));
                });
            });
        })
    }
    
    /// Составление всех двигающихся объектов
    fn get_moving_objects(planets: &[Rc<RefCell<Planet>>]) -> Vec<Weak<RefCell<dyn MovingObject>>> {
        Vec::<Weak<RefCell<dyn MovingObject>>>::new().tap_mut(|moving_objects| {
            planets.iter().for_each(|planet| {
                let planet_as_object: Rc<RefCell<dyn MovingObject>> = planet.clone();
                moving_objects.push(Rc::downgrade(&planet_as_object));
                
                planet.borrow().satellites().iter().for_each(|satellite| {
                    let satellite: Rc<RefCell<dyn MovingObject>> = satellite.clone();
                    moving_objects.push(Rc::downgrade(&satellite));
                });
            });
        })
    }

    /// Получение данных для объектов Солнечной системы 
    fn get_solar_system_data(
        path_to_values: &str,
        path_to_images: &str,
        maximum_number_of_comets: u8,
    ) -> (Sun, Vec<Planet>, CometPossibleValues) {
        let file = File::open(path_to_values)
            .expect("Can not read file with data.");

        let Data {
            sun: sun_data,
            planets: planets_data,
            comet: comet_data
        } = from_reader(BufReader::new(file)).expect("Can not read data");

        let sun = Sun::new(sun_data, path_to_images);

        let mut trajectory_colors_values = HashSet::new();

        let planets = Self::get_planets_data(
            planets_data,
            &mut trajectory_colors_values,
            sun.mass(),
            sun.radius().value(),
            path_to_images,
        );

        let comets_colors = (0..maximum_number_of_comets)
            .map(|_| Self::generate_object_trajectory_color(&mut trajectory_colors_values))
            .collect::<Vec<_>>();

        let comet_values =
            CometPossibleValues::new(comet_data, path_to_images, comets_colors);

        (sun, planets, comet_values)
    }

    /// Получение данных для планет и их спутников
    fn get_planets_data(
        planets_data: Vec<PlanetData>,
        trajectory_colors_values: &mut HashSet<(u8, u8, u8)>,
        sun_mass: Quantity<Kilograms>,
        sun_radius: f32,
        path_to_images: &str,
    ) -> Vec<Planet> {
        planets_data.into_iter().map(|planet_data| {
            let PlanetData {
                name,
                consts: planet_consts,
                image_filename: planet_image_filename,
                satellites
            } = planet_data;

            let planet_trajectory_color =
                Self::generate_object_trajectory_color(trajectory_colors_values);

            let planet_velocity = orbital_velocity(
                sun_mass,
                Quantity::new(Kilometers::new(planet_consts.orbit)),
            );

            let planet_initial_position = Planet::initial_position(
                sun_radius,
                planet_consts.orbit,
                planet_consts.radius,
            );

            let satellites = satellites.into_iter()
                .map(|satellite_data| {
                    let satellite_trajectory_color =
                        Self::generate_object_trajectory_color(trajectory_colors_values);

                    Satellite::new(
                        satellite_data,
                        planet_velocity,
                        planet_consts.mass,
                        planet_initial_position,
                        planet_consts.radius,
                        satellite_trajectory_color,
                        path_to_images,
                    )
                })
                .collect::<Vec<Satellite>>();

            Planet::new(
                name,
                planet_initial_position,
                planet_consts,
                planet_velocity,
                planet_trajectory_color,
                format!("{path_to_images}/{planet_image_filename}"),
                satellites,
            )
        })
            .collect::<Vec<Planet>>()
    }

    /// Генерация цвета траектории объекта
    fn generate_object_trajectory_color(
        trajectory_colors_values: &mut HashSet<(u8, u8, u8)>
    ) -> Color {
        let mut rng = thread_rng();
        loop {
            let color_values = (
                rng.gen_range(0..255_u8),
                rng.gen_range(0..255_u8),
                rng.gen_range(0..255_u8),
            );

            if !trajectory_colors_values.contains(&color_values) {
                trajectory_colors_values.insert(color_values);
                return Color::from_rgba8(color_values.0, color_values.1, color_values.2, 0.8);
            }
        }
    }
}

impl SpaceState {
    pub fn stars(&self) -> &[Star] {
        self.stars.as_slice()
    }

    pub fn comets_count(&self) -> u8 {
        self.comets.count()
    }

    pub fn all_objects(&self) -> &[Weak<RefCell<dyn Object>>] {
        self.all_objects.as_slice()
    }

    pub fn moving_objects(&self) -> &[Weak<RefCell<dyn MovingObject>>] {
        self.moving_objects.as_slice()
    }
}

impl SpaceState {
    /// Движение объектов
    pub fn move_objects(&mut self, seconds_per_tick: Quantity<Seconds>) {
        let objects_gravitational_values = self.all_objects.iter()
            .filter_map(|object| object.upgrade()
                .map(|object_rc| object_rc.borrow().gravitational_force_values())
            )
            .collect::<Vec<_>>();

        let velocities_changes = self.moving_objects.iter()
            .filter_map(|object| object.upgrade()
                .map(|object_rc| {
                    let object = object_rc.borrow();

                    let comet_velocity_change = vector_of_velocity_change(
                        object.gravitational_force_values(),
                        objects_gravitational_values.as_slice(),
                        seconds_per_tick,
                    );

                    (object.name().to_string(), comet_velocity_change)
                })
            )
            .collect::<HashMap<String, VectorValue<KilometersPerSecond>>>();

        self.moving_objects.iter().for_each(|object| {
            object.upgrade().tap_some(|object_rc| {
                let object_name = object_rc.borrow().name().to_string();
                object_rc.borrow_mut().update_position(
                    velocities_changes[&object_name].clone(),
                    seconds_per_tick,
                );
            });
        });
    }
}

impl SpaceState {
    /// Удаление комет столкнувшихся в другой объект
    pub fn remove_crashed_comets(&mut self) {
        let circles = self.all_objects.iter()
            .filter_map(|object| object.upgrade()
                .map(|object_rc| {
                    (object.clone(), Circle::from(object_rc.borrow().form_values()))
                })
            )
            .collect::<Vec<_>>();

        let mut crashed_indices = Vec::new();

        self.comets.as_slice().iter().enumerate().for_each(|(index, comet)| {
            let comet_as_object: Rc<RefCell<dyn Object>> = comet.clone();
            let comet_weak_reference = Rc::downgrade(&comet_as_object);

            let comet_circle = Circle::from(comet.borrow().form_values());

            for (object, object_circle) in circles.iter() {
                if !object.ptr_eq(&comet_weak_reference)
                    && is_circles_have_common_points(&comet_circle, object_circle)
                {
                    crashed_indices.push(index as u8);
                    break;
                }
            }
        });

        crashed_indices.into_iter().rev()
            .for_each(|index| self.comets.delete_comet(index));
    }
}

impl SpaceState {
    /// Создание и добавление новой кометы
    pub fn add_new_comet(&mut self) {
        if let Some(new_comet) = self.comets.add_new_comet() {
            let new_comet = Rc::downgrade(&new_comet);
            self.all_objects.push(new_comet.clone());
            self.moving_objects.push(new_comet);
        }
    }

    /// Удаление кометы
    pub fn delete_comet(&mut self, index: u8) {
        self.comets.delete_comet(index);
    }
}

impl SpaceState {
    pub fn reload(&mut self) {
        self.comets.reload();
        self.filter_from_cleared_objects();
        self.filter_from_cleared_moving_objects();
        self.reload_planets_and_satellites();
    }

    fn filter_from_cleared_objects(&mut self) {
        let cleared_objects = self.all_objects.iter().enumerate()
            .filter_map(|(index, object)| {
                match object.upgrade() {
                    None => Some(index),
                    _ => None,
                }
            })
            .rev()
            .collect::<Vec<_>>();

        cleared_objects.into_iter().for_each(|index| {
            self.all_objects.remove(index);
        });
    }

    fn filter_from_cleared_moving_objects(&mut self) {
        let cleared_moving_objects = self.moving_objects.iter().enumerate()
            .filter_map(|(index, object)| {
                match object.upgrade() {
                    None => Some(index),
                    _ => None,
                }
            })
            .rev()
            .collect::<Vec<_>>();

        cleared_moving_objects.into_iter().for_each(|index| {
            self.moving_objects.remove(index);
        });
    }

    fn reload_planets_and_satellites(&mut self) {
        let sun_mass = self.sun.borrow().mass();
        self.planets.iter_mut().for_each(|planet| {
            let velocity = planet.borrow().movement().velocity().value;
            let mass = planet.borrow().mass().value();
            let orbit = planet.borrow().initial_orbit();
            let radius = planet.borrow().radius().value();

            let mut planet = planet.borrow_mut();

            planet.reload(sun_mass);

            planet.satellites_mut().iter_mut().for_each(move |satellite| {
                satellite.borrow_mut().reload(velocity, mass, orbit, radius);
            })
        });
    }
}