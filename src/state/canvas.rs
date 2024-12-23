use std::cell::{Ref, RefCell};
use std::rc::Weak;

use iced::{Color, mouse, Point, Rectangle, Renderer, Size, Theme, Vector};
use iced::event::Status;
use iced::mouse::{Button, Cursor, ScrollDelta};
use iced::widget::canvas;
use iced::widget::canvas::{Event, Frame, Geometry, Path, Stroke, Style};
use tap::TapOptional;
use crate::util::geometry::point::translate_point;
use crate::util::objects::{MovingObject, Object};

use crate::Message;
use crate::objects::stars::Star;
use crate::state::State;
use crate::state::system_position::CursorPinch;

/// Перенос центра координатной системы на позицию Солнца
fn translate_frame_to_new_center(frame: &mut Frame, center_position: Point) {
    let frame_center = frame.center();
    frame.translate(
        Vector::new(frame_center.x + center_position.x, frame_center.y + center_position.y)
    );
}

/// Отрисовка фоновых звёзд
fn draw_stars(frame: &mut Frame, stars: &[Star]) {
    frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);

    let stars = Path::new(|path| {
        let half_width = frame.width() / 2.;
        let half_height = frame.height() / 2.;
        stars.iter().for_each(|Star { relative_point, size }| path
            .circle(
                Point::new(relative_point.x * half_width, relative_point.y * half_height),
                *size,
            )
        );
    });

    frame.translate(frame.center() - Point::ORIGIN);
    frame.fill(&stars, Color::WHITE);
}

/// Отрисовка Солнечной системы
fn draw_system(
    frame: &mut Frame,
    center_position: Point,
    bounds: Rectangle,
    scale: u32,
    step: u32,
    all_objects: &[Weak<RefCell<dyn Object>>],
    moving_objects: &[Weak<RefCell<dyn MovingObject>>],
) {
    translate_frame_to_new_center(frame, center_position);

    // Отрисовка орбит объектов
    moving_objects.iter().for_each(|object| {
        object.upgrade().tap_some(|object_rc| {
            draw_object_orbit(frame, scale, center_position, bounds, object_rc.borrow(), step)
        });
    });

    // Отрисовка объектов
    all_objects.iter().for_each(|object| {
        object.upgrade()
            .tap_some(|object_rc| draw_object(frame, scale, object_rc.borrow()));
    });
}

/// Отрисовка орбит (траекторий) объектов
fn draw_object_orbit(
    frame: &mut Frame,
    scale: u32,
    center_position: Point,
    bounds: Rectangle,
    object: Ref<dyn MovingObject>,
    step: u32,
) {
    let path = Path::new(|builder| {
        let frame_center = frame.center();
        let system_center_position = Point::new(
            frame_center.x + center_position.x,
            frame_center.y + center_position.y,
        );

        // Масшабированные позиции объекта с шагом, зависимым от масштаба
        let mut object_positions = object.trajectory(step, scale as f32);

        // Проверяем и передвигаем начальную позицию орбиты
        let first_position = object_positions.next().unwrap();

        builder.move_to(first_position);

        let mut is_last_position_was_inside = bounds
            .contains(translate_point(first_position, system_center_position, Point::ORIGIN));

        for position in object_positions {
            // Позиция внутри окна
            if bounds.contains(translate_point(position, system_center_position, Point::ORIGIN)) {
                // Отрисовываем линию к позиции
                builder.line_to(position);
                is_last_position_was_inside = true;
                continue;
            }

            // Позиция не внутри, но предыдущая была внутри
            if is_last_position_was_inside {
                // Отрисовываем линию к позиции
                builder.line_to(position);
                is_last_position_was_inside = false;
                continue;
            }

            // Передвигаем курсор, не отрисовывая ничего
            builder.move_to(position);
        }
    });

    frame.stroke(
        &path,
        Stroke {
            style: Style::Solid(object.trajectory_color()),
            width: 2.,
            ..Stroke::default()
        },
    )
}

/// Отрисовка объекта
fn draw_object(frame: &mut Frame, scale: u32, object: Ref<dyn Object>) {
    let radius = object.scaled_radius(scale);
    let position = object.scaled_position(scale);

    let bounds = Rectangle::new(
        Point::new(position.x - radius, position.y - radius),
        Size::new(radius * 2., radius * 2.),
    );

    frame.draw_image(bounds, object.image());
}

impl canvas::Program<Message> for State {
    type State = ();

    fn update(&self,
        _: &mut Self::State,
        event: Event,
        _: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                // Изменение масштаба
                mouse::Event::WheelScrolled { delta: ScrollDelta::Lines { y, .. } } => (
                    Status::Captured,
                    Some(Message::ScaleChange(y as i16))
                ),

                // Перемещение Солнечной системы
                mouse::Event::CursorMoved { position }
                if matches!(self.system_position.pinch(), CursorPinch::Clamped) => (
                    Status::Captured,
                    Some(Message::PositionChange(position))
                ),

                // Начало перемещения Солнечной системы
                mouse::Event::ButtonPressed(Button::Left) => (
                    Status::Ignored,
                    Some(Message::LeftButtonPressed(cursor.position().unwrap()))
                ),

                // Конец перемещения Солнечной системы
                mouse::Event::ButtonReleased(Button::Left) => (
                    Status::Ignored,
                    Some(Message::LeftButtonReleased)
                ),

                _ => (Status::Ignored, None),
            },
            _ => (Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let stars = self.cache.stars().draw(
            renderer,
            bounds.size(),
            |frame| draw_stars(frame, self.space.stars()),
        );

        let system = self.cache.system().draw(
            renderer,
            bounds.size(),
            |frame| draw_system(
                frame,
                self.system_position.center_position(),
                bounds,
                self.settings.scale().value(),
                self.step(),
                self.space.all_objects(),
                self.space.moving_objects(),
            ),
        );

        vec![stars, system]
    }
}