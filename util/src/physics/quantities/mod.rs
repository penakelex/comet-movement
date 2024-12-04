use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use iced::Point;
use num_traits::{Float, NumCast, ToPrimitive};

use crate::physics::quantities::quantity_units::{
    InterimQuantityUnit,
    Kilometers,
    KilometersPerSecond,
    Meters,
    MetersPerSecond,
};

pub mod quantity_units;

pub trait QuantityUnit {
    type Value: Float + ToPrimitive + Display;

    fn value(&self) -> Self::Value;
    fn marking(&self) -> &str;
}

pub trait NewQuantity: QuantityUnit {
    fn new(value: Self::Value) -> Self;
}

#[derive(Copy, Clone, Debug)]
pub struct Quantity<T>(T)
where
    T: QuantityUnit + NewQuantity + Copy + Clone;

impl<T> Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
{
    pub const fn new(quantity_unit: T) -> Self {
        Self(quantity_unit)
    }
}

impl<T> Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
{
    pub fn value(&self) -> T::Value {
        self.0.value()
    }
}

impl Quantity<InterimQuantityUnit> {
    pub fn parse<T>(&self) -> Quantity<T>
    where
        T: QuantityUnit + NewQuantity + Copy + Clone,
    {
        Quantity::new(T::new(<T::Value as NumCast>::from(self.value()).unwrap()))
    }
}

impl Quantity<Kilometers> {
    pub fn to_meters(self) -> Quantity<Meters> {
        Quantity::new(Meters::new((self.value() as f64) * 1e3))
    }
}

impl Quantity<Meters> {
    pub fn to_kilometers(self) -> Quantity<Kilometers> {
        Quantity::new(Kilometers::new((self.value() / 1e3) as f32))
    }
}

impl Quantity<KilometersPerSecond> {
    pub fn to_meters_per_second(self) -> Quantity<MetersPerSecond> {
        Quantity::new(MetersPerSecond::new(self.value() as f64 * 1e3))
    }
}

impl Quantity<MetersPerSecond> {
    pub fn to_kilometers_per_second(self) -> Quantity<KilometersPerSecond> {
        Quantity::new(KilometersPerSecond::new((self.value() / 1e3) as f32))
    }
}

impl<T> Display for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
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
        let self_value = self.value().to_f64().unwrap();
        let other_value = other.value().to_f64().unwrap();

        Quantity::new(InterimQuantityUnit::new(self_value + other_value))
    }
}

impl<T, Q> Mul<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn mul(self, other: Quantity<Q>) -> Self::Output {
        let self_value = self.value().to_f64().unwrap();
        let other_value = other.value().to_f64().unwrap();

        Quantity::new(InterimQuantityUnit::new(self_value * other_value))
    }
}

impl<T, Q> Sub<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn sub(self, other: Quantity<Q>) -> Self::Output {
        let self_value = self.value().to_f64().unwrap();
        let other_value = other.value().to_f64().unwrap();

        Quantity::new(InterimQuantityUnit::new(self_value - other_value))
    }
}

impl<T, Q> Div<Quantity<Q>> for Quantity<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = Quantity<InterimQuantityUnit>;

    fn div(self, other: Quantity<Q>) -> Self::Output {
        let self_value = self.value().to_f64().unwrap();
        let other_value = other.value().to_f64().unwrap();

        Quantity::new(InterimQuantityUnit::new(self_value / other_value))
    }
}

pub fn point_without_quantity_units<T>(Point { x, y }: Point<Quantity<T>>) -> Point<T::Value>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
{
    Point::new(x.value(), y.value())
}