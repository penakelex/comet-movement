use std::{
    cell::{Ref, RefCell},
    rc::Weak,
};

use iced::{
    Color, Point, Rectangle, Renderer, Size, Theme, Vector,
    mouse,
    mouse::{Button, Cursor, ScrollDelta},
    widget::canvas::{
        self, Action, Event, Frame, Geometry, Path, Stroke,
        Style,
    },
};
use tap::TapOptional;

use crate::{
    Message,
    objects::stars::Star,
    state::{State, system_position::CursorPinch},
    util::{
        geometry::point::translate_point,
        objects::{MovingObject, Object},
    },
};

/// Перенос центра координатной системы на позицию Солнца
fn translate_frame_to_new_center(
    frame: &mut Frame,
    center_position: Point,
) {
    let frame_center = frame.center();
    frame.translate(Vector::new(
        frame_center.x + center_position.x,
        frame_center.y + center_position.y,
    ));
}

/// Отрисовка фоновых звёзд
fn draw_stars(frame: &mut Frame, stars: &[Star]) {
    // Задний фон "пустота"
    frame.fill_rectangle(
        Point::ORIGIN,
        frame.size(),
        Color::BLACK,
    );

    let stars = Path::new(|path| {
        // Для преобразований относительных позиций звёзд в абсолютные
        let half_width = frame.width() / 2.;
        let half_height = frame.height() / 2.;
        stars.iter().for_each(|star| {
            path.circle(
                Point::new(
                    star.relative_point().x * half_width,
                    star.relative_point().y * half_height,
                ),
                star.size(),
            )
        });
    });

    // Перенос центра координатной системы
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
            draw_object_orbit(
                frame,
                scale,
                center_position,
                bounds,
                object_rc.borrow(),
                step,
            )
        });
    });

    // Отрисовка объектов
    all_objects.iter().for_each(|object| {
        object.upgrade().tap_some(|object_rc| {
            draw_object(frame, scale, object_rc.borrow())
        });
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
        let mut object_positions =
            object.trajectory(step, scale as f32);

        // Проверяем и передвигаем начальную позицию орбиты
        let first_position =
            object_positions.next().unwrap();

        builder.move_to(first_position);

        let mut is_last_position_was_inside = bounds
            .contains(translate_point(
                first_position,
                system_center_position,
                Point::ORIGIN,
            ));

        for position in object_positions {
            // Позиция внутри окна
            if bounds.contains(translate_point(
                position,
                system_center_position,
                Point::ORIGIN,
            )) {
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
fn draw_object(
    frame: &mut Frame,
    scale: u32,
    object: Ref<dyn Object>,
) {
    let radius = object.scaled_radius(scale);
    let position = object.scaled_position(scale);

    // Определение границ изображения
    let bounds = Rectangle::new(
        Point::new(
            position.x - radius,
            position.y - radius,
        ),
        Size::new(radius * 2., radius * 2.),
    );

    // Отрисовка изображения
    frame.draw_image(bounds, object.image());
}

impl canvas::Program<Message> for State {
    type State = ();

    fn update(
        &self,
        _: &mut Self::State,
        event: &Event,
        _: Rectangle,
        cursor: Cursor,
    ) -> Option<Action<Message>> {
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    // Изменение масштаба
                    mouse::Event::WheelScrolled {
                        delta: ScrollDelta::Lines { y, .. },
                    } => Some(Action::publish(
                        Message::ScaleChange(*y as i16),
                    )),

                    // Перемещение Солнечной системы
                    mouse::Event::CursorMoved {
                        position,
                    } if matches!(
                        self.system_position.pinch(),
                        CursorPinch::Clamped
                    ) =>
                    {
                        Some(Action::publish(
                            Message::PositionChange(
                                *position,
                            ),
                        ))
                    }

                    // Начало перемещения Солнечной системы
                    mouse::Event::ButtonPressed(
                        Button::Left,
                    ) => Some(Action::publish(
                        Message::LeftButtonPressed(
                            cursor.position().unwrap(),
                        ),
                    )),

                    // Конец перемещения Солнечной системы
                    mouse::Event::ButtonReleased(
                        Button::Left,
                    ) => Some(Action::publish(
                        Message::LeftButtonReleased,
                    )),

                    _ => None,
                }
            }
            _ => None,
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
            |frame| {
                draw_system(
                    frame,
                    self.system_position.center_position(),
                    bounds,
                    self.settings.scale().value(),
                    self.step(),
                    self.space.all_objects(),
                    self.space.moving_objects(),
                )
            },
        );

        vec![stars, system]
    }
}
