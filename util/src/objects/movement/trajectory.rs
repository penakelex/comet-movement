use std::collections::VecDeque;

use iced::Point;

use crate::physics::quantities::{point_without_quantity_units, Quantity};
use crate::physics::quantities::quantity_units::Kilometers;

pub struct Trajectory {
    is_closed: bool,
    is_going_right: bool,
    last_position: Point,
    trajectory: VecDeque<Point>,
}

impl Trajectory {
    pub fn new(starting_position: Point<Quantity<Kilometers>>) -> Self {
        let mut trajectory = VecDeque::with_capacity(10_000);

        let starting_position = point_without_quantity_units(starting_position);
        trajectory.push_back(starting_position);

        Self {
            is_closed: false,
            is_going_right: false,
            last_position: starting_position,
            trajectory,
        }
    }
}

impl Trajectory {
    pub fn positions(&self) -> impl Iterator<Item=&Point> {
        self.trajectory.iter()
    }

    pub fn stepped_scaled_positions<'a>(
        &'a self,
        step: u16,
        scale: f32,
    ) -> impl Iterator<Item=Point> + 'a {
        self.positions()
            .step_by(step as usize)
            .map(move |position| Point::new(position.x / scale, position.y / scale))
    }
}

impl Trajectory {
    pub fn add_position(&mut self, position: Point<Quantity<Kilometers>>) {
        let position = point_without_quantity_units(position);

        if self.is_going_right {
            let first_position = self.trajectory.front().unwrap();
            let last_position = self.trajectory.back().unwrap();

            if first_position.x > last_position.x
                || self.last_position.y > position.y
            {
                if self.is_closed {
                    self.trajectory.pop_front();
                } else {
                    self.is_closed = true;
                }
            }

            if self.last_position.x > position.x {
                self.is_going_right = false;
            }
        } else {
            let first_position = self.trajectory.front().unwrap();
            let last_position = self.trajectory.back().unwrap();

            if first_position.x < last_position.x
                || self.last_position.y < position.y
            {
                if self.is_closed {
                    self.trajectory.pop_front();
                } else {
                    self.is_closed = true;
                }
            }

            if self.last_position.x < position.x {
                self.is_going_right = false;
            }
        }

        self.trajectory.push_back(position);
    }
}
