use std::{cell::RefCell, rc::Rc};

use iced::{Color, Point, widget::image};

use crate::{
    objects::satellite::Satellite,
    util::{
        data::solar_system_data::ObjectConsts,
        objects::{
            Object, ObjectMotion,
            consts::SolarSystemObjectConsts,
            movement::ObjectMovement,
        },
        physics::{
            formulas::orbital_velocity,
            quantities::{
                Quantity,
                quantity_units::{
                    Kilograms, Kilometers,
                    KilometersPerSecond,
                },
            },
        },
    },
};

/// Планета
pub struct Planet {
    /// Название
    name: Box<str>,
    /// Константы
    consts: SolarSystemObjectConsts,
    /// Движение
    movement: ObjectMovement,
    /// Изображение
    image: image::Handle,
    /// Спутники
    satellites: Box<[Rc<RefCell<Satellite>>]>,
}

impl Planet {
    pub fn new(
        planet_name: Box<str>,
        initial_position: f32,
        planet_consts: ObjectConsts,
        velocity: Quantity<KilometersPerSecond>,
        trajectory_color: Color,
        path_to_image: Box<str>,
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
        let satellites = satellites
            .into_iter()
            .map(|satellite| {
                Rc::new(RefCell::new(satellite))
            })
            .collect();

        Self {
            name: planet_name,
            consts,
            movement,
            image: image::Handle::from_path(
                path_to_image.as_ref(),
            ),
            satellites,
        }
    }

    /// Расчёт начальной позиции
    pub fn initial_position(
        sun_radius: f32,
        planet_orbit: f32,
        planet_radius: f32,
    ) -> f32 {
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
        self.satellites.as_ref()
    }

    #[inline(always)]
    pub fn satellites_mut(
        &mut self,
    ) -> &mut [Rc<RefCell<Satellite>>] {
        self.satellites.as_mut()
    }
}

impl Object for Planet {
    #[inline(always)]
    fn name(&self) -> &str {
        self.name.as_ref()
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
    pub fn reload(
        &mut self,
        sun_mass: Quantity<Kilograms>,
    ) {
        let velocity = orbital_velocity(
            sun_mass,
            Quantity::new(Kilometers::new(
                self.consts.initial_orbit().value(),
            )),
        );

        // Обновление движения
        self.movement = ObjectMovement::new_solar_system_object_movement(
            velocity,
            self.consts.initial_orbit().value(),
            self.movement.trajectory_color(),
        );
    }
}
