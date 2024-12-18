use gset::Getset;
use iced::{Color, Point, Vector};
use iced::widget::image;
use rand::{Rng, thread_rng};
use util::data::solar_system_data::CometData;
use util::objects::{Object, ObjectMotion};
use util::objects::movement::ObjectMovement;
use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::{
    Kilograms,
    Kilometers,
    KilometersPerSecond,
};
use util::physics::vector::VectorValue;

/// Комета
pub struct Comet {
    /// Название
    name: String,
    /// Движение
    movement: ObjectMovement,
    /// Масса
    mass: Quantity<Kilograms>,
    /// Радиус
    radius: Quantity<Kilometers>,
    /// Изображение
    image: image::Handle,
}

impl Comet {
    pub fn new(
        comet_possible_values: &CometPossibleValues,
        comet_number: u16,
        image_index: u8,
        trajectory_color_index: u8,
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
            comet_possible_values.get_color_by_index(trajectory_color_index),
        );

        let mass = Quantity::new(
            Kilograms::new(Self::generate_mass(comet_possible_values.masses))
        );

        let radius = Quantity::new(
            Kilometers::new(Self::generate_radius(comet_possible_values.radii))
        );

        let image = comet_possible_values.get_image_by_index(image_index).clone();

        Self {
            name: format!("Comet {comet_number}"),
            movement,
            mass,
            radius,
            image,
        }
    }

    /// Генерация начального положения
    fn generate_starting_position() -> Point<Quantity<Kilometers>> {
        let mut rng = thread_rng();

        Point {
            x: Quantity::new(Kilometers::new(rng.gen_range(-1e8..=1e8))),
            y: Quantity::new(Kilometers::new(rng.gen_range(-1e8..=1e8))),
        }
    }

    /// Генерация начальной скорости
    fn generate_starting_velocity(possible_velocities: (f32, f32)) -> f32 {
        thread_rng().gen_range(possible_velocities.0..=possible_velocities.1)
    }

    /// Генерация единичного вектора начальной скорости
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

    /// Генерация массы
    fn generate_mass(possible_masses: (f32, f32)) -> f64 {
        thread_rng().gen_range(possible_masses.0..=possible_masses.1) as f64
    }

    /// Генерация радиуса
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
    
    fn image(&self) -> &image::Handle {
        &self.image
    }
}

impl ObjectMotion for Comet {
    fn movement(&self) -> &ObjectMovement {
        &self.movement
    }

    fn movement_mut(&mut self) -> &mut ObjectMovement {
        &mut self.movement
    }
}

/// Возможные значения кометы
#[derive(Getset)]
pub struct CometPossibleValues {
    /// Скорости (минимум, максимум)
    #[getset(get_copy, vis = "pub")]
    velocities: (f32, f32),
    /// Массы (минимум, максимум)
    #[getset(get_copy, vis = "pub")]
    masses: (f32, f32),
    /// Радиусы (минимум, максимум)
    #[getset(get_copy, vis = "pub")]
    radii: (f32, f32),
    /// Изображения
    images: Vec<image::Handle>,
    /// Цвета
    colors: Vec<Color>,
}

impl CometPossibleValues {
    /// Получения изображения по индексу
    pub fn get_image_by_index(&self, index: u8) -> &image::Handle {
        &self.images[index as usize]
    }
    
    /// Получение цвета по индексу
    pub fn get_color_by_index(&self, index: u8) -> Color {
        self.colors[index as usize]
    }
}

impl CometPossibleValues {
    pub fn images(&self) -> &[image::Handle] {
        self.images.as_slice()
    }
    
    pub fn colors(&self) -> &[Color] {
        self.colors.as_slice()
    }
}

impl CometPossibleValues {
    pub fn new(data: CometData, path_to_images: &str, colors: Vec<Color>) -> Self {
        let CometData {
            possible_velocities: velocities,
            possible_masses: masses,
            possible_radii: radii,
            images_filenames,
        } = data;

        let images = images_filenames.into_iter()
            .map(|image_filename| {
                image::Handle::from_path(format!("{path_to_images}/{image_filename}"))
            })
            .collect::<Vec<_>>();
        
        Self {
            velocities: (velocities[0], velocities[1]),
            masses: (masses[0], masses[1]),
            radii: (radii[0], radii[1]),
            images,
            colors,
        }
    }
}