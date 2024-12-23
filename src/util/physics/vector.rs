use std::ops::{Add, Div, Mul};

use iced::Vector;
use num_traits::{Float, NumCast, ToPrimitive};

use crate::util::physics::quantities::{NewQuantity, Quantity, QuantityUnit};
use crate::util::physics::quantities::quantity_units::{
    InterimQuantityUnit,
    KilometersPerSecond,
    MetersPerSecond,
};

/// Физический вектор
#[allow(type_alias_bounds)]
pub type VectorValue<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
= crate::util::geometry::vector::VectorValue<Quantity<T>, T::Value>;

impl<T: QuantityUnit + NewQuantity + Copy + Clone> VectorValue<T> {
    pub fn to_vector(&self) -> Vector<T::Value> {
        Vector::new(
            self.value.value() * self.unit_vector.x,
            self.value.value() * self.unit_vector.y,
        )
    }

    pub fn to_quantity_vector(&self) -> Vector<Quantity<T>> {
        Vector::new(
            Quantity::new(T::new(self.value.value() * self.unit_vector.x)),
            Quantity::new(T::new(self.value.value() * self.unit_vector.y)),
        )
    }

    pub fn unit_vector_f64(&self) -> Vector<f64> {
        Vector::new(self.unit_vector.x.to_f64().unwrap(), self.unit_vector.y.to_f64().unwrap())
    }
}

impl VectorValue<InterimQuantityUnit> {
    /// Перевод из неопределённой физической величины в конкретную
    pub fn parse<Q: QuantityUnit + NewQuantity + Copy + Clone>(self) -> VectorValue<Q> {
        let unit_vector = Vector::new(
            <Q::Value as NumCast>::from(self.unit_vector.x).unwrap(),
            <Q::Value as NumCast>::from(self.unit_vector.y).unwrap(),
        );

        VectorValue::new(self.value.parse(), unit_vector)
    }
}

impl VectorValue<MetersPerSecond> {
    /// Перевод скорости из м/с в км/с
    pub fn to_kilometers_per_second(&self) -> VectorValue<KilometersPerSecond> {
        let unit_vector = Vector::new(self.unit_vector.x as f32, self.unit_vector.y as f32);

        VectorValue::new(self.value.to_kilometers_per_second(), unit_vector)
    }
}

impl<T, Q> Mul<Quantity<Q>> for VectorValue<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = VectorValue<InterimQuantityUnit>;

    fn mul(self, scalar: Quantity<Q>) -> Self::Output {
        let value = self.value.value_f64() * scalar.value_f64();
        VectorValue::new(Quantity::new(InterimQuantityUnit::new(value)), self.unit_vector_f64())
    }
}

impl<T, Q> Div<Quantity<Q>> for VectorValue<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = VectorValue<InterimQuantityUnit>;

    fn div(self, scalar: Quantity<Q>) -> Self::Output {
        let value = self.value.value_f64() / scalar.value_f64();
        VectorValue::new(Quantity::new(InterimQuantityUnit::new(value)), self.unit_vector_f64())
    }
}

impl<T, Q> Add<VectorValue<Q>> for VectorValue<T>
where
    T: QuantityUnit + NewQuantity + Copy + Clone,
    Q: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = VectorValue<InterimQuantityUnit>;

    fn add(self, other: VectorValue<Q>) -> Self::Output {
        let self_vector = self.to_vector();
        let other_vector = other.to_vector();

        let self_vector = Vector::new(
            self_vector.x.to_f64().unwrap(),
            self_vector.y.to_f64().unwrap(),
        );
        let other_vector = Vector::new(
            other_vector.x.to_f64().unwrap(),
            other_vector.y.to_f64().unwrap(),
        );

        let Vector { x, y } = self_vector + other_vector;

        let value = x.hypot(y);

        VectorValue::new(
            Quantity::new(InterimQuantityUnit::new(value)),
            Vector::new(x / value, y / value),
        )
    }
}

impl<F, T> Div<F> for VectorValue<T>
where
    F: Float,
    T: QuantityUnit + NewQuantity + Copy + Clone,
{
    type Output = VectorValue<InterimQuantityUnit>;

    fn div(self, scalar: F) -> Self::Output {
        let value = self.value.value_f64() / scalar.to_f64().unwrap();
        VectorValue::new(Quantity::new(InterimQuantityUnit::new(value)), self.unit_vector_f64())
    }
}