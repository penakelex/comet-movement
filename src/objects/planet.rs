use std::cell::RefCell;
use std::rc::Rc;

use iced::{Color, Point};

use util::file_data::{ColorData, PlanetConsts};
use util::objects::{Object, ObjectPositionUpdate, ObjectTrajectory, SolarSystemObjectConsts};
use util::objects::movement::ObjectMovement;
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{
    Kilograms,
    Kilometers,
    KilometersPerSecond,
    Seconds,
};
use util::physics::vector::VectorValue;

use crate::objects::satellite::Satellite;

pub struct Planet {
    name: String,
    consts: SolarSystemObjectConsts,
    color: Color,
    satellites: Vec<Rc<RefCell<Satellite>>>,
    movement: ObjectMovement,
}

impl Planet {
    pub fn new(
        planet_name: String,
        sun_radius: f32,
        planet_consts: PlanetConsts,
        velocity: Quantity<KilometersPerSecond>,
        color: ColorData,
        satellites: Vec<Satellite>,
    ) -> Self {
        let consts = SolarSystemObjectConsts::new(
            planet_consts.mass as f64,
            planet_consts.orbit,
            planet_consts.radius,
        );

        let movement = ObjectMovement::new_solar_system_object_movement(
            velocity,
            sun_radius + planet_consts.orbit + planet_consts.radius,
        );

        let satellites = satellites.into_iter()
            .map(|satellite| Rc::new(RefCell::new(satellite)))
            .collect();

        Self {
            name: planet_name,
            consts,
            color: Color::from_rgb8(color.red, color.green, color.blue),
            satellites,
            movement,
        }
    }
}

impl Planet {
    pub fn satellites(&self) -> &[Rc<RefCell<Satellite>>] {
        self.satellites.as_slice()
    }

    pub fn satellites_mut(&mut self) -> &mut [Rc<RefCell<Satellite>>] {
        self.satellites.as_mut_slice()
    }
}

impl Object for Planet {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn mass(&self) -> Quantity<Kilograms> {
        self.consts.mass()
    }

    fn radius(&self) -> Quantity<Kilometers> {
        self.consts.radius()
    }

    fn position(&self) -> Point<Quantity<Kilometers>> {
        self.movement.position()
    }

    fn color(&self) -> Color {
        self.color
    }
}

impl ObjectPositionUpdate for Planet {
    fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    ) {
        self.movement.update_position(velocity_change, time_interval);
    }
}

impl ObjectTrajectory for Planet {
    fn trajectory<'a>(&'a self, step: u16, scale: f32) -> Box<dyn Iterator<Item=Point> + 'a> {
        Box::new(self.movement.stepped_scaled_trajectory(step, scale))
    }
}