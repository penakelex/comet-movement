use iced::{Color, Point, Rectangle, Renderer, Theme, Vector};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Frame, Geometry, LineDash, Path, Stroke, Style};

use crate::objects::Object;
use crate::objects::planets::Planet;
use crate::objects::stars::Star;
use crate::state::State;

impl State {
    fn draw_background(&self, frame: &mut Frame) {
        let half_width = frame.width() / 2.;
        let half_height = frame.height() / 2.;
        let center_point = frame.center();

        frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);

        let stars = Path::new(|path| {
            for Star { point: relative_point, size } in &self.stars {
                let point = Point {
                    x: relative_point.x * half_width,
                    y: relative_point.y * half_height,
                };
                path.circle(point, *size)
            }
        });

        frame.translate(center_point - Point::ORIGIN);
        frame.fill(&stars, Color::WHITE);

        let is_extreme_point_outside_circle = center_point.x * center_point.x
            + center_point.y * center_point.y;

        let sun_radius = self.sun.object_radius_scaled(self.scale);

        for object in self.planets {
            let scaled_orbit_radius = object.orbit_radius_scaled(self.scale) + sun_radius;

            if is_extreme_point_outside_circle > scaled_orbit_radius * scaled_orbit_radius {
                frame.stroke(
                    &Path::circle(Point::ORIGIN, scaled_orbit_radius),
                    Stroke {
                        style: Style::Solid(Color::WHITE.scale_alpha(0.1)),
                        width: 1.,
                        line_dash: LineDash {
                            offset: 0,
                            segments: &[3., 6.],
                        },
                        ..Stroke::default()
                    },
                );
            }
        }
    }

    fn draw_sun(&self, frame: &mut Frame) {
        frame.translate(frame.center() - Point::ORIGIN);
        frame.rotate(self.sun.angle_of_rotation(&self.time));

        frame.fill(
            &Path::circle(Point::ORIGIN, self.sun.object_radius_scaled(self.scale)),
            self.sun.color(),
        );
    }

    fn draw_planet(&self, frame: &mut Frame, planet: &&dyn Planet) {
        frame.translate(frame.center() - Point::ORIGIN);

        frame.rotate(planet.angle_of_rotation(&self.time));

        frame.translate(
            Vector::new(
                planet.orbit_radius_scaled(self.scale)
                    + self.sun.object_radius_scaled(self.scale),
                0.,
            )
        );

        //TODO: Убрать коэффициент 2000 после добавления увеличения и уменьшения картинки
        frame.fill(
            &Path::circle(Point::ORIGIN, 2000. * planet.object_radius_scaled(self.scale)),
            planet.color(),
        );
    }

    /*fn draw_satellite(&self, frame: &mut Frame, satellite: &&dyn Object) {
        todo!()
    }*/
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut all_geometry = Vec::with_capacity(10);

        all_geometry.push(
            self.background_cache.draw(
                renderer,
                bounds.size(),
                |frame| self.draw_background(frame),
            )
        );

        all_geometry.push(
            self.sun_cache.draw(
                renderer,
                bounds.size(),
                |frame| self.draw_sun(frame),
            )
        );

        for i in 0..self.planets.len() {
            all_geometry.push(
                self.planets_caches[i].draw(
                    renderer,
                    bounds.size(),
                    |frame| self.draw_planet(
                        frame,
                        &self.planets[i],
                    ),
                )
            );
        }

        all_geometry
    }
}