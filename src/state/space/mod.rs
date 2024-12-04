use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use getset::Getters;
use rand::{Rng, thread_rng};
use serde_json::from_reader;

use util::file_data::{Data, PlanetData};
use util::geometry::circle::{Circle, is_circles_have_common_points};
use util::objects::{FormValues, Object, ObjectPositionUpdate, ObjectTrajectory};
use util::physics::formulas::{orbital_velocity, velocity_change};
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{Kilometers, KilometersPerSecond};
use util::physics::vector::VectorValue;

use crate::objects::comet::{Comet, CometPossibleValues};
use crate::objects::planet::Planet;
use crate::objects::satellite::Satellite;
use crate::objects::stars::Star;
use crate::objects::sun::Sun;
use crate::state::settings::Settings;
use crate::state::space::comets::CometsState;

pub mod comets;

#[derive(Getters)]
pub struct SpaceState {
    stars: Vec<Star>,
    planets: Vec<Rc<RefCell<Planet>>>,
    #[getset(get = "pub")]
    sun: Rc<RefCell<Sun>>,
    all_objects: Vec<Rc<RefCell<dyn Object>>>,
    objects_with_trajectory: Vec<Rc<RefCell<dyn ObjectTrajectory>>>,
    comets: CometsState,
}

impl SpaceState {
    pub fn new() -> Self {
        let (
            sun,
            planets,
            comet_possible_values
        ) = Self::get_solar_system_data();

        let sun = Rc::new(RefCell::new(sun));

        let planets = planets.into_iter()
            .map(|planet| Rc::new(RefCell::new(planet)))
            .collect::<Vec<_>>();

        let all_objects = Self::get_all_objects(&sun, planets.as_slice());

        let objects_with_trajectory = Self::get_objects_with_trajectory(planets.as_slice());

        Self {
            stars: Self::generate_stars(),
            planets,
            sun,
            all_objects,
            objects_with_trajectory,
            comets: CometsState::new(comet_possible_values),
        }
    }

    fn generate_stars() -> Vec<Star> {
        let mut rng = thread_rng();

        (0..500_u16)
            .map(|_| {
                Star::new(
                    rng.gen_range(-1.0..=1.0),
                    rng.gen_range(-1.0..=1.0),
                    rng.gen_range(0.5..1.0),
                )
            })
            .collect()
    }

    fn get_all_objects(
        sun: &Rc<RefCell<Sun>>,
        planets: &[Rc<RefCell<Planet>>],
    ) -> Vec<Rc<RefCell<dyn Object>>> {
        let mut all_objects = Vec::<Rc<RefCell<dyn Object>>>::new();

        all_objects.push(sun.clone());

        planets.iter().for_each(|planet| {
            all_objects.push(planet.clone());

            planet.borrow().satellites().iter()
                .for_each(|satellite| all_objects.push(satellite.clone()));
        });

        all_objects
    }

    fn get_objects_with_trajectory(planets: &[Rc<RefCell<Planet>>]) -> Vec<Rc<RefCell<dyn ObjectTrajectory>>> {
        let mut objects_with_trajectory = Vec::<Rc<RefCell<dyn ObjectTrajectory>>>::new();

        planets.iter().for_each(|planet| {
            objects_with_trajectory.push(planet.clone());

            planet.borrow().satellites().iter().for_each(|satellite| {
                objects_with_trajectory.push(satellite.clone())
            });
        });

        objects_with_trajectory
    }

    fn get_solar_system_data() -> (Sun, Vec<Planet>, CometPossibleValues) {
        let file = File::open("solar_system_data.json")
            .expect("Can not read file with data.");

        let Data {
            sun: sun_data,
            planets: planets_data,
            comet: comet_data
        } = from_reader(BufReader::new(file)).expect("Can not read data");

        let sun = Sun::new(sun_data);

        let planets = planets_data.into_iter().map(|planet_data| {
            let PlanetData {
                name,
                consts,
                color,
                satellites
            } = planet_data;

            let velocity = orbital_velocity(
                sun.mass(),
                Quantity::new(Kilometers::new(consts.orbit)),
            );

            let satellites = satellites.into_iter()
                .map(|satellite_data| {
                    Satellite::new(
                        satellite_data,
                        sun.radius().value(),
                        consts.mass,
                        consts.orbit,
                        consts.radius,
                        velocity,
                    )
                })
                .collect::<Vec<Satellite>>();

            Planet::new(
                name,
                sun.radius().value(),
                consts,
                velocity,
                color,
                satellites,
            )
        })
            .collect::<Vec<Planet>>();

        let comet_values = CometPossibleValues::new(comet_data);

        (sun, planets, comet_values)
    }
}

impl SpaceState {
    pub fn stars(&self) -> &[Star] {
        self.stars.as_slice()
    }

    pub fn planets(&self) -> &[Rc<RefCell<Planet>>] {
        self.planets.as_slice()
    }

    pub fn comets(&self) -> &[Rc<RefCell<Comet>>] {
        self.comets.as_slice()
    }

    pub fn comets_count(&self) -> u8 { self.comets.count() }

    pub fn all_objects(&self) -> &[Rc<RefCell<dyn Object>>] {
        self.all_objects.as_slice()
    }

    pub fn objects_with_trajectory(&self) -> &[Rc<RefCell<dyn ObjectTrajectory>>] {
        self.objects_with_trajectory.as_slice()
    }
}

impl SpaceState {
    pub fn move_objects(&mut self) {
        let objects_gravitational_values = self.all_objects
            .iter()
            .map(|object| object.borrow().gravitational_force_values())
            .collect::<Vec<_>>();

        let velocities_changes = self.objects_with_trajectory.iter()
            .map(|object| {
                let object = object.borrow();

                let comet_velocity_change = velocity_change(
                    object.gravitational_force_values(),
                    objects_gravitational_values.as_slice(),
                    Settings::SECONDS_PER_TICK,
                );
                (object.name().to_string(), comet_velocity_change)
            })
            .collect::<HashMap<String, VectorValue<KilometersPerSecond>>>();

        self.planets.iter_mut().for_each(|planet| {
            let planet_name = planet.borrow().name().to_string();
            planet.borrow_mut().update_position(
                velocities_changes[&planet_name].clone(),
                Settings::SECONDS_PER_TICK,
            );

            planet.borrow_mut().satellites_mut().iter_mut().for_each(|satellite| {
                let satellite_name = satellite.borrow().name().to_string();
                satellite.borrow_mut().update_position(
                    velocities_changes[&satellite_name].clone(),
                    Settings::SECONDS_PER_TICK,
                )
            });
        });

        self.comets.as_mut_slice().iter_mut().for_each(|comet| {
            let comet_name = comet.borrow().name().to_string();
            comet.borrow_mut().update_position(
                velocities_changes[&comet_name].clone(),
                Settings::SECONDS_PER_TICK,
            )
        });
    }
}

impl SpaceState {
    pub fn remove_crashed_comets(&mut self) {
        let objects_form_values = self.all_objects.iter()
            .map(|object| {
                let object = object.borrow();
                (object.name().to_string(), object.form_values())
            });

        let circles = objects_form_values
            .map(|(name, form_values)| (name, Circle::from(form_values)))
            .collect::<HashMap<String, Circle>>();

        let mut crashed_indices = Vec::new();

        self.comets.as_slice().iter().enumerate().rev()
            .for_each(|(index, comet)| {
                let comet = comet.borrow();

                let comet_circle = Circle::from(comet.form_values());

                for (name, object_circle) in circles.iter() {
                    if name == comet.name() {
                        continue;
                    }

                    if is_circles_have_common_points(&comet_circle, object_circle) {
                        crashed_indices.push(index as u8);
                        break;
                    }
                }
            });

        crashed_indices.into_iter().rev().for_each(|index| {
            self.comets.delete_comet(index);
        });
    }
}

impl SpaceState {
    pub fn add_new_comet(&mut self) {
        self.comets.add_new_comet();
    }

    pub fn delete_comet(&mut self, index: u8) {
        self.comets.delete_comet(index);
    }
}