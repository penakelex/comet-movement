use getset::CopyGetters;
use iced::{Point, Vector};

use crate::objects::movement::trajectory::Trajectory;
use crate::physics::formulas::end_position_after_moving;
use crate::physics::quantities::Quantity;
use crate::physics::quantities::quantity_units::{Kilometers, KilometersPerSecond, Seconds};
use crate::physics::vector::VectorValue;

pub mod trajectory;

#[derive(CopyGetters)]
pub struct ObjectMovement {
    velocity: VectorValue<KilometersPerSecond>,
    #[getset(get_copy = "pub")]
    position: Point<Quantity<Kilometers>>,
    trajectory: Trajectory,
}

impl ObjectMovement {
    pub fn new_solar_system_object_movement(
        velocity: Quantity<KilometersPerSecond>,
        starting_x_position: f32,
    ) -> Self {
        let starting_position = Point {
            x: Quantity::new(Kilometers::new(starting_x_position)),
            y: Quantity::new(Kilometers::new(0.)),
        };

        Self {
            velocity: VectorValue::new(velocity, Vector::new(0., 1.)),
            trajectory: Trajectory::new(starting_position),
            position: starting_position,
        }
    }

    pub fn new_comet_movement(
        velocity: VectorValue<KilometersPerSecond>,
        starting_position: Point<Quantity<Kilometers>>,
    ) -> Self {
        Self {
            velocity,
            position: starting_position,
            trajectory: Trajectory::new(starting_position),
        }
    }
}

impl ObjectMovement {
    pub fn velocity(&self) -> VectorValue<KilometersPerSecond> {
        self.velocity.clone()
    }

    pub fn trajectory(&self) -> impl Iterator<Item=&Point> {
        self.trajectory.positions()
    }

    pub fn stepped_scaled_trajectory<'a>(
        &'a self,
        step: u16,
        scale: f32,
    ) -> impl Iterator<Item=Point> + 'a {
        self.trajectory.stepped_scaled_positions(step, scale)
    }
}

impl ObjectMovement {
    pub fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    ) {
        self.trajectory.add_position(self.position);

        self.position = end_position_after_moving(
            self.velocity.clone(),
            velocity_change.clone(),
            time_interval,
            self.position,
        );

        self.velocity = (self.velocity.clone() + velocity_change).parse();
    }
}