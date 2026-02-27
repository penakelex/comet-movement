use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::util::objects::{MovingObject, Object};
use gset::Getset;
use rand::prelude::IteratorRandom;

use crate::objects::comet::{Comet, CometPossibleValues};

/// Состояние комет
#[derive(Getset)]
pub struct CometsState {
    /// Возможные значения
    #[getset(get, vis = "pub")]
    possible_values: CometPossibleValues,
    /// Кометы
    comets: Vec<Rc<RefCell<Comet>>>,
    /// Занятые индексы тел
    taken_colors_indices: HashSet<u8>,
    /// Новый номер кометы
    new_comet_number: u16,
    /// Максимальное количество комет
    maximum_number_of_comets: u8,
}

impl CometsState {
    #[inline(always)]
    pub fn new(possible_values: CometPossibleValues, maximum_number_of_comets: u8) -> Self {
        Self {
            possible_values,
            comets: Vec::with_capacity(maximum_number_of_comets as usize),
            taken_colors_indices: HashSet::with_capacity(maximum_number_of_comets as usize),
            new_comet_number: 1,
            maximum_number_of_comets,
        }
    }
}

impl CometsState {
    /// Подсчёт комет
    #[inline(always)]
    pub fn count(&self) -> u8 {
        self.comets.len() as u8
    }
}

impl CometsState {
    #[inline(always)]
    pub fn as_slice(&self) -> &[Rc<RefCell<Comet>>] {
        self.comets.as_slice()
    }
}

impl CometsState {
    /// Добавление новой кометы
    pub fn add_new_comet(&mut self) -> Option<Rc<RefCell<Comet>>> {
        if self.comets.capacity() == self.comets.len() {
            return None;
        }

        // Получение индекса свободного цвета
        let free_color_index = self.free_comet_color_index();
        self.taken_colors_indices.insert(free_color_index);

        let comet = Rc::new(RefCell::new(Comet::new(
            &self.possible_values,
            self.new_comet_number,
            self.free_comet_image_index(), // Индекс свободной картинки
            free_color_index,
        )));

        self.comets.push(comet.clone());
        self.new_comet_number += 1;

        Some(comet)
    }

    /// Поиск свободного индекса изображения
    fn free_comet_image_index(&self) -> u8 {
        let taken_indices = self
            .comets
            .iter()
            .map(|comet| comet.borrow().image().id())
            .collect::<Vec<_>>();

        // Выборка случайного индекса свободного изображения
        self.possible_values
            .images()
            .iter()
            .enumerate()
            .filter_map(|(index, image)| {
                if !taken_indices.contains(&image.id()) {
                    Some(index as u8)
                } else {
                    None
                }
            })
            .choose(&mut rand::rng())
            .unwrap()
    }

    /// Поиск свободного индекса кометы
    fn free_comet_color_index(&self) -> u8 {
        (0..self.maximum_number_of_comets)
            .filter(|index| !self.taken_colors_indices.contains(index))
            .choose(&mut rand::rng())
            .unwrap()
    }
}

impl CometsState {
    /// Удаление кометы
    pub fn delete_comet(&mut self, comet_number: u8) {
        // Если индекс существует
        if (0..self.count()).contains(&comet_number) {
            let comet_rc = self.comets.remove(comet_number as usize);
            let comet = comet_rc.borrow();

            // Ищем и удаляем из занятых индекс цвета кометы
            let trajectory_color = comet.trajectory_color();
            let trajectory_color_rgb = &trajectory_color.into_rgba8()[0..3];

            for (index, color) in self.possible_values.colors().iter().enumerate() {
                let color_rgb = &color.into_rgba8()[0..3];
                if trajectory_color_rgb == color_rgb {
                    self.taken_colors_indices.remove(&(index as u8));
                }
            }
        }
    }
}

impl CometsState {
    pub fn reload(&mut self) {
        self.taken_colors_indices.clear();
        self.new_comet_number = 1;
        self.comets.clear();
    }
}
