use std::cell::RefCell;
use std::rc::Rc;
use crate::objects::comet::{Comet, CometPossibleValues};
use getset::Getters;

#[derive(Getters)]
pub struct CometsState {
    #[getset(get = "pub")]
    possible_values: CometPossibleValues,
    comets: Vec<Rc<RefCell<Comet>>>,
    new_comet_number: u16,
}

impl CometsState {
    pub fn new(possible_values: CometPossibleValues) -> Self {
        Self {
            possible_values,
            comets: Vec::with_capacity(10),
            new_comet_number: 1,
        }
    }
}

impl CometsState {
    pub fn count(&self) -> u8 {
        self.comets.len() as u8
    }
}

impl CometsState {
    pub fn as_slice(&self) -> &[Rc<RefCell<Comet>>] {
        self.comets.as_slice()
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [Rc<RefCell<Comet>>]{
        self.comets.as_mut_slice()
    }
}

impl CometsState {
    pub fn add_new_comet(&mut self) {
        if self.comets.capacity() == self.comets.len() {
            return;
        }

        let comet = Comet::new(&self.possible_values, self.new_comet_number);
        
        self.comets.push(Rc::new(RefCell::new(comet)));
        self.new_comet_number += 1;
    }

    pub fn delete_comet(&mut self, comet_number: u8) {
        self.comets.remove(comet_number as usize);
    }
}