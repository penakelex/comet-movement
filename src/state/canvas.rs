use std::cell::{Ref, RefCell};
use std::rc::Rc;

use iced::{Color, mouse, Point, Rectangle, Renderer, Theme, Vector};
use iced::event::Status;
use iced::mouse::{Button, Cursor, ScrollDelta};
use iced::widget::canvas;
use iced::widget::canvas::{Event, Frame, Geometry, LineDash, Path, Stroke, Style};

use util::geometry::point::translate_point;
use util::objects::{Object, ObjectTrajectory};

use crate::Message;
use crate::objects::stars::Star;
use crate::state::State;
use crate::state::system_position::CursorPinch;

fn draw_stars(frame: &mut Frame, stars: &[Star]) {
    frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);

    let stars = Path::new(|path| {
        let half_width = frame.width() / 2.;
        let half_height = frame.height() / 2.;
        stars.iter().for_each(|Star { relative_point, size }| {
            path.circle(
                Point::new(relative_point.x * half_width, relative_point.y * half_height),
                *size,
            )
        });
    });

    frame.translate(frame.center() - Point::ORIGIN);
    frame.fill(&stars, Color::WHITE);
}

fn draw_orbits(
    frame: &mut Frame,
    center_position: Point,
    scale: u64,
    bounds: Rectangle,
    objects_with_trajectory: &[Rc<RefCell<dyn ObjectTrajectory>>],
    step: u16,
) {
    let frame_center = frame.center();

    frame.translate(
        Vector::new(
            frame_center.x + center_position.x,
            frame_center.y + center_position.y,
        )
    );

    objects_with_trajectory.iter().for_each(|object| {
        draw_object_orbit(
            frame,
            scale,
            center_position,
            bounds,
            object.borrow(),
            step,
        );
    });
}

fn draw_object_orbit(
    frame: &mut Frame,
    scale: u64,
    center_position: Point,
    bounds: Rectangle,
    object: Ref<dyn ObjectTrajectory>,
    step: u16,
) {
    let path = Path::new(|builder| {
        let frame_center = frame.center();

        let system_center_position = Point::new(
            frame_center.x + center_position.x,
            frame_center.y + center_position.y,
        );

        let mut object_positions = object.trajectory(step, scale as f32);

        let first_position = object_positions.next().unwrap();

        builder.move_to(first_position);

        let mut is_last_position_was_inside = bounds.contains(first_position);

        for position in object_positions {
            let translated_position = translate_point(
                position,
                system_center_position,
                Point::ORIGIN,
            );

            if bounds.contains(translated_position) {
                builder.line_to(position);
                is_last_position_was_inside = true;
                continue;
            }

            if is_last_position_was_inside {
                builder.move_to(position);
            } else {
                builder.line_to(position);
            }

            is_last_position_was_inside = false;
        }
    });

    frame.stroke(
        &path,
        Stroke {
            style: Style::Solid(Color::WHITE.scale_alpha(0.1)),
            width: 1.,
            line_dash: LineDash {
                offset: 0,
                segments: &[3., 6.],
            },
            ..Stroke::default()
        },
    )
}

fn draw_system(
    frame: &mut Frame,
    center_position: Point,
    scale: u64,
    all_objects: &[Rc<RefCell<dyn Object>>],
) {
    let frame_center = frame.center();

    frame.translate(
        Vector::new(
            frame_center.x + center_position.x,
            frame_center.y + center_position.y,
        )
    );

    all_objects.iter()
        .for_each(|object| draw_object(frame, scale, object.borrow()));
}

fn draw_object(frame: &mut Frame, scale: u64, object: Ref<dyn Object>) {
    frame.fill(
        &Path::circle(object.scaled_position(scale), object.scaled_radius(scale)),
        object.color(),
    )
}

impl canvas::Program<Message> for State {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        _bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        if let Event::Mouse(mouse_event) = event {
            return match mouse_event {
                mouse::Event::WheelScrolled { delta } => {
                    if let ScrollDelta::Lines { y, .. } = delta {
                        (Status::Captured, Some(Message::ScaleChange(y as i16)))
                    } else {
                        (Status::Ignored, None)
                    }
                }

                mouse::Event::CursorMoved {
                    position
                } if matches!(self.system_position.pinch(), CursorPinch::Clamped) => (
                    Status::Captured,
                    Some(Message::PositionChange(position))
                ),

                mouse::Event::ButtonPressed(button) if matches!(button, Button::Left) => (
                    Status::Ignored,
                    Some(Message::LeftButtonPressed(cursor.position().unwrap()))
                ),

                mouse::Event::ButtonReleased(button) if matches!(button, Button::Left) => (
                    Status::Ignored,
                    Some(Message::LeftButtonReleased)
                ),

                _ => (Status::Ignored, None),
            };
        }
        (Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let center_position = self.system_position.system_position();

        let orbits_step_between_points = (self.settings.scale.value / 300_000) as u16 + 1;

        let stars = self.cache.stars.draw(
            renderer,
            bounds.size(),
            |frame| draw_stars(frame, self.space.stars()),
        );

        let orbits = self.cache.orbits.draw(
            renderer,
            bounds.size(),
            |frame| draw_orbits(
                frame,
                center_position,
                self.settings.scale.value,
                bounds,
                self.space.objects_with_trajectory(),
                orbits_step_between_points,
            ),
        );

        let system = self.cache.system.draw(
            renderer,
            bounds.size(),
            |frame| draw_system(
                frame,
                center_position,
                self.settings.scale.value,
                self.space.all_objects(),
            ),
        );

        vec![stars, orbits, system]
    }
}