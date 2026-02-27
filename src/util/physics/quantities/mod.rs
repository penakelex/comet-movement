use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

use iced::Point;
use num_traits::{Float, NumCast, ToPrimitive};

use crate::util::physics::quantities::quantity_units::{
    InterimQuantityUnit, Kilometers, KilometersPerSecond,
    Meters, MetersPerSecond,
};

pub mod quantity_units;

/// Типаж физической единицы
pub trait QuantityUnit {
    type Value: Float + ToPrimitive + Display;

    /// Значение физической величины
    fn value(&self) -> Self::Value;
    /// Обозначение физической единицы
    fn marking(&self) -> &str;
}

pub trait NewQuantity: QuantityUnit {
    fn new(value: Self::Value) -> Self;
}

/// Физическая величина
#[derive(Copy, Clone)]
pub struct Quantity<
    T: QuantityUnit + NewQuantity + Copy + Clone,
>(T);

impl<T: QuantityUnit + NewQuantity + Copy + Clone>
    Quantity<T>
{
    #[inline(always)]
    pub const fn new(quantity_unit: T) -> Self {
        Self(quantity_unit)
    }

    #[inline(always)]
    pub fn value(&self) -> T::Value {
        self.0.value()
    }

    #[inline(always)]
    pub(crate) fn value_f64(&self) -> f64 {
        self.value().to_f64().unwrap()
    }
}

impl Quantity<InterimQuantityUnit> {
    /// Перевод физической величины в конкретную
    pub fn parse<
        T: QuantityUnit + NewQuantity + Copy + Clone,
    >(
        &self,
    ) -> Quantity<T> {
        Quantity::new(T::new(
            <T::Value as NumCast>::from(self.value())
                .unwrap(),
        ))
    }
}

impl Quantity<Kilometers> {
    /// Перевод из км в м
    pub fn to_meters(self) -> Quantity<Meters> {
        Quantity::new(Meters::new(
            (self.value() as f64) * 1e3,
        ))
    }
}

impl Quantity<MetersPerSecond> {
    /// Перевод из м/с в км/с
    #[inline(always)]
    pub fn to_kilometers_per_second(
        self,
    ) -> Quantity<KilometersPerSecond> {
        Quantity::new(KilometersPerSecond::new(
            (self.value() / 1e3) as f32,
        ))
    }
}

impl<T: QuantityUnit + NewQuantity + Copy + Clone> Display
    for Quantity<T>
{
    fn fmt(
        &self,
        fmt: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(fmt, "{} {}", self.value(), self.0.marking())
    }
}

impl<T, Q> Add<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn add(self, other: Quantity<Q>) -> Self::Output {
        Quantity::new(InterimQuantityUnit::new(
            self.value_f64() + other.value_f64(),
        ))
    }
}

impl<T, Q> Mul<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn mul(self, other: Quantity<Q>) -> Self::Output {
        Quantity::new(InterimQuantityUnit::new(
            self.value_f64() * other.value_f64(),
        ))
    }
}

impl<T, Q> Sub<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn sub(self, other: Quantity<Q>) -> Self::Output {
        Quantity::new(InterimQuantityUnit::new(
            self.value_f64() - other.value_f64(),
        ))
    }
}

impl<T, Q> Div<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn div(self, other: Quantity<Q>) -> Self::Output {
        Quantity::new(InterimQuantityUnit::new(
            self.value_f64() / other.value_f64(),
        ))
    }
}

/// Представление позиции тела без физических единиц
pub fn point_without_quantity_units<T>(
    Point { x, y }: Point<Quantity<T>>,
) -> Point<T::Value>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
{
    Point::new(x.value(), y.value())
}
