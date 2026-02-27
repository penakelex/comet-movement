use crate::util::physics::quantities::{NewQuantity, QuantityUnit};

/// Физическая единица силы: Ньютоны
#[derive(Copy, Clone)]
pub struct Newtons(f64);

impl Newtons {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for Newtons {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Newtons {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "N"
    }
}

/// Физическая единица расстояния: Километры
#[derive(Copy, Clone)]
pub struct Kilometers(f32);

impl Kilometers {
    #[inline(always)]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl NewQuantity for Kilometers {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Kilometers {
    type Value = f32;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "km"
    }
}

/// Физическая единица расстояния: Метры
#[derive(Copy, Clone)]
pub struct Meters(f64);

impl Meters {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for Meters {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Meters {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "m"
    }
}

/// Физическая единица скорости: Километры в секунду
#[derive(Copy, Clone)]
pub struct KilometersPerSecond(f32);

impl KilometersPerSecond {
    #[inline(always)]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl NewQuantity for KilometersPerSecond {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for KilometersPerSecond {
    type Value = f32;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "km/s"
    }
}

/// Физическая единица скорости: Метры в секунду
#[derive(Copy, Clone)]
pub struct MetersPerSecond(f64);

impl MetersPerSecond {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for MetersPerSecond {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for MetersPerSecond {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "m/s"
    }
}

/// Физическая единица массы: Килограммы
#[derive(Copy, Clone)]
pub struct Kilograms(f64);

impl Kilograms {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for Kilograms {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Kilograms {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "kg"
    }
}

/// Физическая единица гравитационной постоянной
#[derive(Copy, Clone)]
pub struct NewtonMeterSquaredPerKilogramSquared(f64);

impl NewtonMeterSquaredPerKilogramSquared {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for NewtonMeterSquaredPerKilogramSquared {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for NewtonMeterSquaredPerKilogramSquared {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "N*m^2/kg^2"
    }
}

/// Физическая единица импульса: Килограмм на метр в секунду
#[derive(Copy, Clone)]
pub struct KilogramMeterPerSecond(f64);

impl KilogramMeterPerSecond {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for KilogramMeterPerSecond {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for KilogramMeterPerSecond {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "kg*m/s"
    }
}

/// Физическая единица времени: Секунды
#[derive(Copy, Clone)]
pub struct Seconds(f32);

impl Seconds {
    #[inline(always)]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl NewQuantity for Seconds {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for Seconds {
    type Value = f32;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "s"
    }
}

/// Неопределённая (промежуточная) физическая единица
#[derive(Copy, Clone)]
pub struct InterimQuantityUnit(f64);

impl InterimQuantityUnit {
    #[inline(always)]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl NewQuantity for InterimQuantityUnit {
    #[inline(always)]
    fn new(value: Self::Value) -> Self {
        Self::new(value)
    }
}

impl QuantityUnit for InterimQuantityUnit {
    type Value = f64;

    #[inline(always)]
    fn value(&self) -> Self::Value {
        self.0
    }

    #[inline(always)]
    fn marking(&self) -> &str {
        "unknown"
    }
}