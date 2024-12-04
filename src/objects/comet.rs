use getset::CopyGetters;

use iced::{Color, Point, Vector};
use rand::{Rng, thread_rng};

use util::file_data::CometData;
use util::physics::vector::VectorValue;
use util::objects::{Object, ObjectPositionUpdate, ObjectTrajectory};
use util::objects::movement::ObjectMovement;
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{
    Kilograms, 
    Kilometers, 
    KilometersPerSecond, 
    Seconds
};
pub struct Comet {
    name: String,
    movement: ObjectMovement,
    mass: Quantity<Kilograms>,
    radius: Quantity<Kilometers>,
    color: Color,
}

impl Comet {
    pub fn new(
        comet_possible_values: &CometPossibleValues,
        comet_number: u16,
    ) -> Self {
        let starting_velocity = Quantity::new(
            KilometersPerSecond::new(
                Self::generate_starting_velocity(comet_possible_values.velocities())
            )
        );
        
        let movement = ObjectMovement::new_comet_movement(
            VectorValue::new(
                starting_velocity,
                Self::generate_starting_velocity_vector(),
            ),
            Self::generate_starting_position(),
        );

        let mass = Quantity::new(
            Kilograms::new(Self::generate_mass(comet_possible_values.masses))
        );
        
        let radius = Quantity::new(
            Kilometers::new(Self::generate_radius(comet_possible_values.radii))
        );
        
        Self {
            name: format!("Comet {comet_number}"),
            movement,
            mass,
            radius,
            color: Self::generate_color(),
        }
    }

    fn generate_starting_position() -> Point<Quantity<Kilometers>> {
        let mut rng = thread_rng();

        Point {
            x: Quantity::new(Kilometers::new(rng.gen_range(-1e8..=1e8))),
            y: Quantity::new(Kilometers::new(rng.gen_range(-1e8..=1e8))),
        }
    }

    fn generate_starting_velocity(possible_velocities: (f32, f32)) -> f32 {
        thread_rng().gen_range(possible_velocities.0..=possible_velocities.1)
    }

    fn generate_starting_velocity_vector() -> Vector {
        let mut thread_rng = thread_rng();

        let velocity_x = thread_rng.gen_range(-1.0_f32..=1.);
        let velocity_y = thread_rng.gen_range(-1.0_f32..=1.);
        let velocity_vector_length = (velocity_x * velocity_x + velocity_y * velocity_y).sqrt();

        Vector::new(
            velocity_x / velocity_vector_length,
            velocity_y / velocity_vector_length,
        )
    }

    fn generate_color() -> Color {
        let base_color_value = thread_rng().gen_range(0..=255_u8);
        Color::from_rgb8(base_color_value, base_color_value, base_color_value)
    }

    fn generate_mass(possible_masses: (f32, f32)) -> f64 {
        thread_rng().gen_range(possible_masses.0..=possible_masses.1) as f64
    }

    fn generate_radius(possible_radii: (f32, f32)) -> f32 {
        thread_rng().gen_range(possible_radii.0..=possible_radii.1)
    }
}

impl Object for Comet {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn mass(&self) -> Quantity<Kilograms> {
        self.mass
    }

    fn radius(&self) -> Quantity<Kilometers> {
        self.radius
    }

    fn position(&self) -> Point<Quantity<Kilometers>> {
        self.movement.position()
    }

    fn color(&self) -> Color {
        self.color
    }
}

impl ObjectPositionUpdate for Comet {
    fn update_position(
        &mut self,
        velocity_change: VectorValue<KilometersPerSecond>,
        time_interval: Quantity<Seconds>,
    ) {
        self.movement.update_position(velocity_change, time_interval);
    }
}

impl ObjectTrajectory for Comet {
    fn trajectory<'a>(&'a self, step: u16, scale: f32) -> Box<dyn Iterator<Item=Point> + 'a> {
        Box::new(self.movement.stepped_scaled_trajectory(step, scale))
    }
}

#[derive(CopyGetters)]
pub struct CometPossibleValues {
    #[getset(get_copy = "pub")]
    velocities: (f32, f32),
    #[getset(get_copy = "pub")]
    masses: (f32, f32),
    #[getset(get_copy = "pub")]
    radii: (f32, f32),
}

impl CometPossibleValues {
    pub fn new(data: CometData) -> Self {
        let CometData {
            possible_velocities: velocities,
            possible_masses: masses,
            possible_radii: radii
        } = data;

        Self {
            velocities: (velocities[0], velocities[1]),
            masses: (masses[0], masses[1]),
            radii: (radii[0], radii[1]),
        }
    }
}