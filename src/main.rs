use std::f32::consts::PI;
use std::time::Instant;

use iced::{application, Color, Element, Fill, Point, Rectangle, Renderer, Size, Subscription, Theme, Vector};
use iced::mouse::Cursor;
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Geometry, LineDash, Path, Stroke, Style};
use iced::window::{frames, Settings};
use rand::{Rng, thread_rng};

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    application(
        SolarSystem::title,
        SolarSystem::update,
        SolarSystem::view,
    )
        .subscription(SolarSystem::subscription)
        .theme(SolarSystem::theme)
        .window_size(Size::new(1920., 1080.))
        .run()
}

#[derive(Default)]
struct SolarSystem {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant)
}

impl SolarSystem {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick(instant) => {
                self.state.update(instant);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        canvas(&self.state)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Moonfly
    }

    fn subscription(&self) -> Subscription<Message> {
        frames().map(Message::Tick)
    }

    fn title(&self) -> String {
        String::from("Comet movement")
    }
}

struct State {
    space_cache: Cache,
    orbits_cache: Cache,
    system_cache: Cache,
    start: Instant,
    now: Instant,
    stars: Vec<Star>,
}

struct Star {
    point: Point,
    size: f32,
}

impl Star {
    pub fn new(point_x: f32, point_y: f32, size: f32) -> Self {
        Star {
            point: Point::new(point_x, point_y),
            size,
        }
    }
}

impl State {
    //Взять потом настоящие радиусы тел и орбит в масштабе
    const SUN_RADIUS: f32 = 70.0;
    const ORBIT_RADIUS: f32 = 150.0;
    const EARTH_RADIUS: f32 = 12.0;
    const MOON_RADIUS: f32 = 4.0;
    const MOON_DISTANCE: f32 = 28.0;

    pub fn new() -> State {
        let now = Instant::now();
        let size = Settings::default().size;
        State {
            space_cache: Cache::new(),
            orbits_cache: Cache::new(),
            system_cache: Cache::new(),
            start: now,
            now,
            stars: Self::generate_stars(size.width, size.height),
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }

    fn generate_stars(width: f32, height: f32) -> Vec<Star> {
        let mut rng = thread_rng();

        (0..500_u16)
            .map(|_| {
                Star::new(
                    rng.gen_range((-width / 2.0)..=(width / 2.0)),
                    rng.gen_range((-height / 2.0)..=(width / 2.0)),
                    rng.gen_range(0.5..1.0),
                )
            })
            .collect()
    }
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
        let background = self.space_cache
            .draw(renderer, bounds.size(), |frame| {
                frame.fill_rectangle(Point::ORIGIN, frame.size(), Color::BLACK);

                let stars = Path::new(|path| {
                    self.stars.iter()
                        .for_each(|Star { point, size }| {
                            path.circle(*point, *size)
                        });
                });

                frame.translate(frame.center() - Point::ORIGIN);
                frame.fill(&stars, Color::WHITE);
            });
        
        let orbits = self.orbits_cache
            .draw(renderer, bounds.size(), |frame| {
                frame.translate(frame.center() - Point::ORIGIN);
                
                frame.stroke(
                    &Path::circle(Point::ORIGIN, Self::ORBIT_RADIUS),
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
            });

        let system = self.system_cache
            .draw(renderer, bounds.size(), |frame| {
                frame.translate(frame.center() - Point::ORIGIN);

                frame.fill(
                    &Path::circle(Point::ORIGIN, Self::SUN_RADIUS),
                    Color::from_rgb8(255, 240, 105),
                );

                let elapsed = self.now - self.start;
                let rotation = (2. * PI / 60.) * elapsed.as_secs() as f32
                    + (2. * PI / 60_000.) * elapsed.subsec_millis() as f32;

                frame.rotate(rotation);
                frame.translate(Vector::new(Self::ORBIT_RADIUS, 0.));

                frame.fill(
                    &Path::circle(Point::ORIGIN, Self::EARTH_RADIUS),
                    Color::from_rgb8(26, 194, 23),
                );

                frame.rotate(rotation * 10.);
                frame.translate(Vector::new(0., Self::MOON_DISTANCE));

                frame.fill(
                    &Path::circle(Point::ORIGIN, Self::MOON_RADIUS),
                    Color::from_rgb8(166, 171, 166),
                );
            });
        vec![background, orbits, system]
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}