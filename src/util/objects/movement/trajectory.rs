use std::collections::VecDeque;
use gset::Getset;
use iced::{Color, Point, Vector};
use num_traits::Float;
use tap::Tap;

use crate::util::physics::quantities::{point_without_quantity_units, Quantity};
use crate::util::physics::quantities::quantity_units::Kilometers;

/// Траектория двигающегося объекта
#[derive(Getset)]
pub struct Trajectory {
    /// Замкнута ли траектория объекта
    is_closed: bool,
    closing: Option<TrajectoryClosing>,
    /// Цвет траектории
    #[getset(get_copy, vis = "pub")]
    color: Color,
    /// Позиции тела
    positions: VecDeque<Point>,
}

impl Trajectory {
    pub fn new<F: Float>(
        initial_velocity_vector: Vector<F>,
        starting_position: Point<Quantity<Kilometers>>,
        color: Color,
    ) -> Self {
        let trajectory = VecDeque::with_capacity(10_000)
            .tap_mut(|trajectory| trajectory
                .push_back(point_without_quantity_units(starting_position))
            );

        Self {
            is_closed: false,
            closing: Some(TrajectoryClosing::new(initial_velocity_vector)),
            color,
            positions: trajectory,
        }
    }
}

impl Trajectory {
    /// Позиции тела с пропуском некоторых точек и масштабированием
    pub fn positions(&self, step: u32, scale: f32) -> impl Iterator<Item=Point> + '_ {
        self.positions.iter().step_by(step as usize)
            .map(move |position| Point::new(position.x / scale, position.y / scale))
    }
}

impl Trajectory {
    /// Добавление позиции объекта после движения
    pub fn add_position<F: Float>(
        &mut self,
        position: Point<Quantity<Kilometers>>,
        velocity_vector: Vector<F>,
    ) {
        self.positions.push_back(point_without_quantity_units(position));

        // Траектория замкнута
        if self.is_closed {
            self.positions.pop_front();
            return;
        }

        let direction = Direction::from(velocity_vector);

        let closing = self.closing.as_mut().unwrap();

        // Изменение направления
        if closing.direction == direction {
            return;
        }
        
        match direction {
            Direction::LeftDown => closing.is_left_down_used = true,
            Direction::LeftUp => closing.is_left_up_used = true,
            Direction::RightDown => closing.is_right_down_used = true,
            Direction::RightUp => closing.is_right_up_used = true,
        }

        closing.direction = direction;

        // Каждое направление уже было
        if closing.is_left_down_used && closing.is_left_up_used
            && closing.is_right_down_used && closing.is_right_up_used
        {
            if !closing.is_all_directions_used {
                closing.is_all_directions_used = true;
                return;
            }

            // Направление сменилось нужное количество раз, при этом в разные стороны
            self.is_closed = true;
            self.closing.take();
        }
    }
}


/// Значения для проверки замкнутости траектории
struct TrajectoryClosing {
    pub direction: Direction,
    /// Двигался ли уже влево вниз
    pub is_left_down_used: bool,
    /// Двигался ли уже влево вверх
    pub is_left_up_used: bool,
    /// Двигался ли уже вправо вниз
    pub is_right_down_used: bool,
    /// Двигался ли уже вправо вверх
    pub is_right_up_used: bool,
    /// Двигался ли уже во все направления
    pub is_all_directions_used: bool,
}

impl TrajectoryClosing {
    pub fn new<F: Float>(initial_velocity_vector: Vector<F>) -> Self {
        Self {
            direction: Direction::from(initial_velocity_vector),
            is_left_down_used: false,
            is_left_up_used: false,
            is_right_down_used: false,
            is_right_up_used: false,
            is_all_directions_used: false,
        }
    }
}

/// Направление движения объекта
#[derive(PartialEq)]
enum Direction {
    /// Влево вниз
    LeftDown,
    /// Влево вверх
    LeftUp,
    /// Вправо вниз
    RightDown,
    /// Вправо вверх
    RightUp,
}


impl<F: Float> From<Vector<F>> for Direction {
    fn from(vector: Vector<F>) -> Self {
        match (vector.x, vector.y) {
            (x, y) if x.is_sign_positive() && y.is_sign_positive() => Self::RightDown,
            (x, y) if x.is_sign_positive() && y.is_sign_negative() => Self::RightUp,
            (x, y) if x.is_sign_negative() && y.is_sign_positive() => Self::LeftDown,
            _ => Self::LeftUp,
        }
    }
}
