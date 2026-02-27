use crate::util::data::solar_system_data::CometData;
use crate::util::objects::movement::ObjectMovement;
use crate::util::objects::{Object, ObjectMotion};
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{
    Kilograms, Kilometers, KilometersPerSecond,
};
use crate::util::physics::vector::VectorValue;
use gset::Getset;
use iced::widget::image;
use iced::{Color, Point, Vector};
use rand::RngExt;
use rand::rngs::ThreadRng;

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
        let mut thread_rng = rand::rng();

        // Генерация начальной скорости
        let starting_velocity =
            Quantity::new(KilometersPerSecond::new(
                Self::generate_starting_velocity(
                    comet_possible_values.velocities(),
                    &mut thread_rng,
                ),
            ));

        // Создание движения объекта
        let movement = ObjectMovement::new_comet_movement(
            VectorValue::new(
                starting_velocity,
                Self::generate_starting_velocity_vector(
                    &mut thread_rng,
                ),
            ),
            Self::generate_starting_position(
                &mut thread_rng,
            ),
            comet_possible_values
                .get_color_by_index(trajectory_color_index),
        );

        // Перевод массы в физическую величину
        let mass = Quantity::new(Kilograms::new(
            Self::generate_mass(
                comet_possible_values.masses,
                &mut thread_rng,
            ),
        ));

        // Перевод радиуса в физическую величину
        let radius = Quantity::new(Kilometers::new(
            Self::generate_radius(
                comet_possible_values.radii,
                &mut thread_rng,
            ),
        ));

        // Получение картинки из имеющихся
        let image = comet_possible_values
            .get_image_by_index(image_index)
            .clone();

        Self {
            name: format!("Comet {comet_number}"),
            movement,
            mass,
            radius,
            image,
        }
    }

    /// Генерация начального положения
    fn generate_starting_position(
        rng: &mut ThreadRng,
    ) -> Point<Quantity<Kilometers>> {
        Point {
            x: Quantity::new(Kilometers::new(
                rng.random_range(-1e8..=1e8),
            )),
            y: Quantity::new(Kilometers::new(
                rng.random_range(-1e8..=1e8),
            )),
        }
    }

    /// Генерация начальной скорости
    fn generate_starting_velocity(
        possible_velocities: (f32, f32),
        rng: &mut ThreadRng,
    ) -> f32 {
        rng.random_range(
            possible_velocities.0..=possible_velocities.1,
        )
    }

    /// Генерация единичного вектора начальной скорости
    fn generate_starting_velocity_vector(
        rng: &mut ThreadRng,
    ) -> Vector {
        let velocity_x = rng.random_range(-1.0_f32..=1.);
        let velocity_y = rng.random_range(-1.0_f32..=1.);
        let velocity_vector_length = (velocity_x
            * velocity_x
            + velocity_y * velocity_y)
            .sqrt();

        Vector::new(
            velocity_x / velocity_vector_length,
            velocity_y / velocity_vector_length,
        )
    }

    /// Генерация массы
    fn generate_mass(
        possible_masses: (f32, f32),
        rng: &mut ThreadRng,
    ) -> f64 {
        rng.random_range(
            possible_masses.0..=possible_masses.1,
        ) as f64
    }

    /// Генерация радиуса
    fn generate_radius(
        possible_radii: (f32, f32),
        rng: &mut ThreadRng,
    ) -> f32 {
        rng.random_range(
            possible_radii.0..=possible_radii.1,
        )
    }
}

impl Object for Comet {
    #[inline(always)]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline(always)]
    fn mass(&self) -> Quantity<Kilograms> {
        self.mass
    }

    #[inline(always)]
    fn radius(&self) -> Quantity<Kilometers> {
        self.radius
    }

    #[inline(always)]
    fn position(&self) -> Point<Quantity<Kilometers>> {
        self.movement.position()
    }

    #[inline(always)]
    fn image(&self) -> &image::Handle {
        &self.image
    }
}

impl ObjectMotion for Comet {
    #[inline(always)]
    fn movement(&self) -> &ObjectMovement {
        &self.movement
    }

    #[inline(always)]
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
    #[inline(always)]
    pub fn get_image_by_index(
        &self,
        index: u8,
    ) -> &image::Handle {
        &self.images[index as usize]
    }

    /// Получение цвета по индексу
    #[inline(always)]
    pub fn get_color_by_index(&self, index: u8) -> Color {
        self.colors[index as usize]
    }
}

impl CometPossibleValues {
    #[inline(always)]
    pub fn images(&self) -> &[image::Handle] {
        self.images.as_slice()
    }

    #[inline(always)]
    pub fn colors(&self) -> &[Color] {
        self.colors.as_slice()
    }
}

impl CometPossibleValues {
    pub fn new(
        data: CometData,
        path_to_images: &str,
        colors: Vec<Color>,
    ) -> Self {
        let CometData {
            possible_velocities: velocities,
            possible_masses: masses,
            possible_radii: radii,
            images_filenames,
        } = data;

        // Получение картинок комет из файлов
        let images = images_filenames
            .into_iter()
            .map(|image_filename| {
                image::Handle::from_path(format!(
                    "{path_to_images}/{image_filename}"
                ))
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
