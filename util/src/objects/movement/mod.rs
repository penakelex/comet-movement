use gset::Getset;
use iced::{Color, Point, Vector};

use crate::objects::movement::trajectory::Trajectory;
use crate::physics::formulas::end_position_after_moving;
use crate::physics::quantities::Quantity;
use crate::physics::quantities::quantity_units::{Kilometers, KilometersPerSecond, Seconds};
use crate::physics::vector::VectorValue;

pub mod trajectory;

/// Структура для значений двигающегося вокруг Солнца объекта
#[derive(Getset)]
pub struct ObjectMovement {
    /// Скорость объекта
    velocity: VectorValue<KilometersPerSecond>,
    /// Позиция объекта
    #[getset(get_copy, vis = "pub")]
    position: Point<Quantity<Kilometers>>,
    /// Траектория объекта
    trajectory: Trajectory,
}

impl ObjectMovement {
    /// Новый экземпляр для объектов Солнечной системы
    pub fn new_solar_system_object_movement(
        velocity: Quantity<KilometersPerSecond>,
        starting_x_position: f32,
        trajectory_color: Color,
    ) -> Self {
        let starting_position = Point {
            x: Quantity::new(Kilometers::new(starting_x_position)),
            y: Quantity::new(Kilometers::new(0.)),
        };

        Self {
            velocity: VectorValue::new(velocity, Vector::new(0., 1.)),
            trajectory: Trajectory::new(starting_position, trajectory_color),
            position: starting_position,
        }
    }

    /// Новый экземпляр для комет
    pub fn new_comet_movement(
        velocity: VectorValue<KilometersPerSecond>,
        starting_position: Point<Quantity<Kilometers>>,
        trajectory_color: Color,
    ) -> Self {
        Self {
            velocity,
            position: starting_position,
            trajectory: Trajectory::new(starting_position, trajectory_color),
        }
    }
}

impl ObjectMovement {
    pub fn velocity(&self) -> VectorValue<KilometersPerSecond> {
        self.velocity.clone()
    }

    pub fn trajectory_color(&self) -> Color {
        self.trajectory.color()
    }

    /// Масштабированая траектория с пропуском некоторых позиций
    pub fn stepped_scaled_trajectory<'a>(
        &'a self,
        step: u32,
        scale: f32,
    ) -> impl Iterator<Item=Point> + 'a {
        self.trajectory.stepped_scaled_positions(step, scale)
    }
}

impl ObjectMovement {
    /// Обновление позиции после движения
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