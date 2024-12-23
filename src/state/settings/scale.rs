use gset::Getset;

/// Масштаб
#[derive(Getset)]
pub struct Scale {
    /// Значение
    #[getset(get_copy, vis = "pub")]
    value: u32,
    /// Строковое знчение
    #[getset(get, vis = "pub", ty = "&str")]
    value_string: String,
}

impl Scale {
    pub fn new(default_scale: u32) -> Self {
        Self {
            value: default_scale,
            value_string: default_scale.to_string(),
        }
    }
}

impl Scale {
    pub fn set(&mut self, scale: u32) {
        self.value = scale;
        self.value_string = scale.to_string();
    }

    pub fn set_value(&mut self, scale: u32) {
        if scale > 0 {
            self.value = scale;
        }
    }

    pub fn set_string_value(&mut self, scale: String) {
        self.value_string = scale;
    }
}

impl Scale {
    pub fn reload(&mut self, default_scale: u32) {
        self.value = default_scale;
        self.value_string = default_scale.to_string();
    }
}