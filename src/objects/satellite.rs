use iced::{Color, Point};

use util::file_data::SatelliteData;
use util::objects::{Object, ObjectPositionUpdate, ObjectTrajectory, SolarSystemObjectConsts};
use util::objects::movement::ObjectMovement;
use util::physics::formulas::orbital_velocity;
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{Kilograms, Kilometers, KilometersPerSecond, Seconds};
use util::physics::vector::VectorValue;

pub struct Satellite {
    name: String,
    consts: SolarSystemObjectConsts,
    color: Color,
    movement: ObjectMovement,
}

impl Satellite {
    pub fn new(
        satellite_data: SatelliteData,
        sun_radius: f32,
        planet_mass: f32,
        planet_orbit: f32,
        planet_radius: f32,
        planet_velocity: Quantity<KilometersPerSecond>,
    ) -> Self {
        let SatelliteData {
            name,
            consts: satellite_consts,
            color,
        } = satellite_data;

        let velocity = orbital_velocity(
            Quantity::new(Kilograms::new(planet_mass as f64)),
            Quantity::new(Kilometers::new(satellite_consts.orbit)),
        );

        let consts = SolarSystemObjectConsts::new(
            satellite_consts.mass as f64,
            satellite_consts.orbit,
            satellite_consts.radius,
        );

        let movement = ObjectMovement::new_solar_system_object_movement(
            (velocity + planet_velocity).parse(),
            sun_radius + planet_orbit + planet_radius * 2. + satellite_consts.orbit,
        );

        Self {
            name,
            consts,
            color: Color::from_rgb8(color.red, color.green, color.blue),
            movement,
        }
    }
}

impl Object for Satellite {
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

impl ObjectPositionUpdate for Satellite {
    fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    ) {
        self.movement.update_position(velocity_change, time_interval);
    }
}

impl ObjectTrajectory for Satellite {
    fn trajectory<'a>(&'a self, step: u16, scale: f32) -> Box<dyn Iterator<Item=Point> + 'a> {
        Box::new(self.movement.stepped_scaled_trajectory(step, scale))
    }
}