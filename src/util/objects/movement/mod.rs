use gset::Getset;
use iced::{Color, Point, Vector};

use crate::util::objects::movement::trajectory::Trajectory;
use crate::util::physics::formulas::end_position_after_moving;
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{
    Kilometers, KilometersPerSecond, Seconds,
};
use crate::util::physics::vector::VectorValue;

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
            x: Quantity::new(Kilometers::new(
                starting_x_position,
            )),
            y: Quantity::new(Kilometers::new(0.)),
        };

        let velocity_vector = Vector::new(-1e-10, 1.);

        Self {
            velocity: VectorValue::new(
                velocity,
                velocity_vector,
            ),
            trajectory: Trajectory::new(
                velocity_vector,
                starting_position,
                trajectory_color,
            ),
            position: starting_position,
        }
    }

    /// Новый экземпляр для комет
    pub fn new_comet_movement(
        velocity: VectorValue<KilometersPerSecond>,
        starting_position: Point<Quantity<Kilometers>>,
        trajectory_color: Color,
    ) -> Self {
        let velocity_vector = velocity.unit_vector;

        Self {
            velocity,
            position: starting_position,
            trajectory: Trajectory::new(
                velocity_vector,
                starting_position,
                trajectory_color,
            ),
        }
    }
}

impl ObjectMovement {
    pub fn velocity(
        &self,
    ) -> VectorValue<KilometersPerSecond> {
        self.velocity.clone()
    }

    pub fn trajectory_color(&self) -> Color {
        self.trajectory.color()
    }

    /// Масштабированая траектория с пропуском некоторых позиций
    pub fn trajectory(
        &self,
        step: u32,
        scale: f32,
    ) -> impl Iterator<Item = Point> + '_ {
        self.trajectory.positions(step, scale)
    }
}

impl ObjectMovement {
    /// Обновление позиции после движения
    pub fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    ) {
        self.position = end_position_after_moving(
            self.velocity.clone(),
            velocity_change.clone(),
            time_interval,
            self.position,
        );

        self.velocity = (self.velocity.clone()
            + velocity_change)
            .parse();

        self.trajectory.add_position(
            self.position,
            self.velocity.unit_vector,
        );
    }
}
