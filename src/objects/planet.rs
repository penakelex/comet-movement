use std::cell::RefCell;
use std::rc::Rc;

use iced::{Color, Point};
use iced::widget::image;
use crate::util::data::solar_system_data::ObjectConsts;
use crate::util::objects::{Object, ObjectMotion};
use crate::util::objects::consts::SolarSystemObjectConsts;
use crate::util::objects::movement::ObjectMovement;
use crate::util::physics::formulas::orbital_velocity;
use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::{
    Kilograms,
    Kilometers,
    KilometersPerSecond,
};

use crate::objects::satellite::Satellite;

/// Планета
pub struct Planet {
    /// Название
    name: String,
    /// Константы
    consts: SolarSystemObjectConsts,
    /// Движение
    movement: ObjectMovement,
    /// Изображение
    image: image::Handle,
    /// Спутники
    satellites: Vec<Rc<RefCell<Satellite>>>,
}

impl Planet {
    pub fn new(
        planet_name: String,
        initial_position: f32,
        planet_consts: ObjectConsts,
        velocity: Quantity<KilometersPerSecond>,
        trajectory_color: Color,
        path_to_image: String,
        satellites: Vec<Satellite>,
    ) -> Self {
        // Запись констант
        let consts = SolarSystemObjectConsts::new(
            planet_consts.mass,
            planet_consts.orbit,
            planet_consts.radius,
        );

        // Создание движения планеты
        let movement = ObjectMovement::new_solar_system_object_movement(
            velocity,
            initial_position,
            trajectory_color,
        );

        // Запись спутников
        let satellites = satellites.into_iter()
            .map(|satellite| Rc::new(RefCell::new(satellite)))
            .collect();
        
        Self {
            name: planet_name,
            consts,
            movement,
            image: image::Handle::from_path(path_to_image),
            satellites,
        }
    }
    
    /// Расчёт начальной позиции
    pub fn initial_position(sun_radius: f32, planet_orbit: f32, planet_radius: f32) -> f32 {
        sun_radius + planet_orbit + planet_radius
    }
}

impl Planet {
    #[inline(always)]
    pub fn initial_orbit(&self) -> f32 {
        self.consts.initial_orbit().value()
    }

    #[inline(always)]
    pub fn satellites(&self) -> &[Rc<RefCell<Satellite>>] {
        self.satellites.as_slice()
    }

    #[inline(always)]
    pub fn satellites_mut(&mut self) -> &mut [Rc<RefCell<Satellite>>] {
        self.satellites.as_mut_slice()
    }
}

impl Object for Planet {
    #[inline(always)]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline(always)]
    fn mass(&self) -> Quantity<Kilograms> {
        self.consts.mass()
    }

    #[inline(always)]
    fn radius(&self) -> Quantity<Kilometers> {
        self.consts.radius()
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

impl ObjectMotion for Planet {
    #[inline(always)]
    fn movement(&self) -> &ObjectMovement {
        &self.movement
    }

    #[inline(always)]
    fn movement_mut(&mut self) -> &mut ObjectMovement {
        &mut self.movement
    }
}

impl Planet {
    // При перезагрузке симуляции
    pub fn reload(&mut self, sun_mass: Quantity<Kilograms>) {
        let velocity = orbital_velocity(
            sun_mass,
            Quantity::new(Kilometers::new(self.consts.initial_orbit().value())),
        );

        // Обновление движения
        self.movement = ObjectMovement::new_solar_system_object_movement(
            velocity,
            self.consts.initial_orbit().value(),
            self.movement.trajectory_color(),
        );
    }
}