use crate::physics::quantities::{NewQuantity, QuantityUnit};

#[derive(Copy, Clone)]
pub struct Newtons(f64);

impl Newtons {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for Newtons {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Newtons {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "N"
    }
}

#[derive(Copy, Clone)]
pub struct Kilometers(f32);

impl Kilometers {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl NewQuantity for Kilometers {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Kilometers {
    type Value = f32;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "km"
    }
}

#[derive(Copy, Clone)]
pub struct Meters(f64);

impl Meters {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for Meters {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Meters {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "m"
    }
}

#[derive(Copy, Clone)]
pub struct KilometersPerSecond(f32);

impl KilometersPerSecond {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl NewQuantity for KilometersPerSecond {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for KilometersPerSecond {
    type Value = f32;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "km/s"
    }
}

#[derive(Copy, Clone)]
pub struct MetersPerSecond(f64);

impl MetersPerSecond {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for MetersPerSecond {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for MetersPerSecond {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "m/s"
    }
}

#[derive(Copy, Clone)]
pub struct Kilograms(f64);

impl Kilograms {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for Kilograms {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Kilograms {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "kg"
    }
}

#[derive(Copy, Clone)]
pub struct NewtonMeterSquaredPerKilogramSquared(f64);

impl NewtonMeterSquaredPerKilogramSquared {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for NewtonMeterSquaredPerKilogramSquared {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for NewtonMeterSquaredPerKilogramSquared {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "N*m^2/kg^2"
    }
}

#[derive(Copy, Clone)]
pub struct KilogramMeterPerSecond(f64);

impl KilogramMeterPerSecond {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for KilogramMeterPerSecond {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for KilogramMeterPerSecond {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "kg*m/s"
    }
}

#[derive(Copy, Clone)]
pub struct Seconds(f32);

impl Seconds {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl NewQuantity for Seconds {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Seconds {
    type Value = f32;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "s"
    }
}

#[derive(Copy, Clone)]
pub struct InterimQuantityUnit(f64);

impl InterimQuantityUnit {
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for InterimQuantityUnit {
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for InterimQuantityUnit {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.0
    }

    fn marking(&self) -> &str {
        "unknown"
    }
}