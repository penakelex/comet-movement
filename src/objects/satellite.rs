use iced::{Color, Point};
use iced::widget::image;
use crate::util::data::solar_system_data::SatelliteData;
use crate::util::objects::{Object, ObjectMotion};
use crate::util::objects::consts::SolarSystemObjectConsts;
use crate::util::objects::movement::ObjectMovement;
use crate::util::physics::formulas::orbital_velocity;
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{Kilograms, Kilometers, KilometersPerSecond};

/// Спутник
pub struct Satellite {
    /// Название
    name: String,
    /// Константы
    consts: SolarSystemObjectConsts,
    /// Движение
    movement: ObjectMovement,
    /// Изображение
    image: image::Handle,
}

impl Satellite {
    pub fn new(
        satellite_data: SatelliteData,
        planet_velocity: Quantity<KilometersPerSecond>,
        planet_mass: f64,
        planet_initial_orbit: f32,
        planet_radius: f32,
        trajectory_color: Color,
        path_to_images: &str,
    ) -> Self {
        let SatelliteData {
            name,
            consts: satellite_consts,
            image_filename,
        } = satellite_data;

        // Вычисление орбитальной скорости вокруг планеты
        let velocity = orbital_velocity(
            Quantity::new(Kilograms::new(planet_mass)),
            Quantity::new(Kilometers::new(satellite_consts.orbit)),
        );

        // Запись констант
        let consts = SolarSystemObjectConsts::new(
            satellite_consts.mass,
            satellite_consts.orbit,
            satellite_consts.radius,
        );

        // Создание движения спутника
        let movement = ObjectMovement::new_solar_system_object_movement(
            (velocity + planet_velocity).parse(),
            planet_initial_orbit + planet_radius + satellite_consts.orbit,
            trajectory_color,
        );

        Self {
            name,
            consts,
            movement,
            image: image::Handle::from_path(format!("{path_to_images}/{image_filename}")),
        }
    }
}

impl Object for Satellite {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn mass(&self) -> Quantity<Kilograms> {
        self.consts.mass()
    }

    fn radius(&self) -> Quantity<Kilometers> {
        self.consts.radius()
    }

    fn position(&self) -> Point<Quantity<Kilometers>> {
        self.movement.position()
    }

    fn image(&self) -> &image::Handle {
        &self.image
    }
}

impl ObjectMotion for Satellite {
    fn movement(&self) -> &ObjectMovement {
        &self.movement
    }

    fn movement_mut(&mut self) -> &mut ObjectMovement {
        &mut self.movement
    }
}

impl Satellite {
    // При перезагрузке симуляции
    pub fn reload(
        &mut self,
        planet_velocity: Quantity<KilometersPerSecond>,
        planet_mass: f64,
        planet_initial_orbit: f32,
        planet_radius: f32,
    ) {
        let velocity = orbital_velocity(
            Quantity::new(Kilograms::new(planet_mass)),
            Quantity::new(Kilometers::new(self.consts.initial_orbit().value())),
        );
        
        // Обновление движение
        self.movement = ObjectMovement::new_solar_system_object_movement(
            (velocity + planet_velocity).parse(),
            planet_initial_orbit + planet_radius + self.consts.initial_orbit().value(),
            self.movement.trajectory_color(),
        )
    }
}