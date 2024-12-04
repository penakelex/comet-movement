use iced::{Point, Vector};

use crate::objects::ObjectGravitationalForceValues;
use crate::physics::quantities::Quantity;
use crate::physics::quantities::quantity_units::{
    KilogramMeterPerSecond, 
    Kilograms, 
    Kilometers, 
    KilometersPerSecond, 
    Meters, 
    MetersPerSecond, 
    NewtonMeterSquaredPerKilogramSquared, 
    Newtons, 
    Seconds
};
use crate::physics::vector::VectorValue;

const ZERO_FORCE_VECTOR: VectorValue<Newtons> = VectorValue::new(
    Quantity::new(Newtons::new(0.)), Vector::new(0., 0.),
);

const G: Quantity<NewtonMeterSquaredPerKilogramSquared> = Quantity::new(
    NewtonMeterSquaredPerKilogramSquared::new(6.6743e-11)
);

#[allow(non_snake_case)]
pub fn vector_of_gravitational_force(
    object1: &ObjectGravitationalForceValues,
    object2: &ObjectGravitationalForceValues,
) -> VectorValue<Newtons> {
    let Point { x: x1, y: y1 } = object1.position();
    let Point { x: x2, y: y2 } = object2.position();

    let x = (x2 - x1).parse::<Kilometers>().to_meters();
    let y = (y2 - y1).parse::<Kilometers>().to_meters();

    if x.value().abs() < 1e-11 && y.value().abs() < 1e-11 {
        return ZERO_FORCE_VECTOR;
    }

    let r = Quantity::new(Meters::new(x.value().hypot(y.value())));

    let F = (G * object1.mass() * object2.mass() / (r * r)).parse();
    let unit_vector = Vector::new((x / r).value(), (y / r).value());

    VectorValue::new(F, unit_vector)
}

pub fn resulting_gravitational_force(
    object_values: &ObjectGravitationalForceValues,
    other_objects_values: &[ObjectGravitationalForceValues],
) -> VectorValue<Newtons> {
    let mut vectors_of_forces = other_objects_values.iter()
        .map(|other_object| {
            vector_of_gravitational_force(&object_values, other_object)
        });
    
    let mut sum_of_force_vectors = vectors_of_forces.next()
        .unwrap_or(ZERO_FORCE_VECTOR);
    
    for force_vector in vectors_of_forces {
        sum_of_force_vectors = (sum_of_force_vectors + force_vector).parse();
    }

    sum_of_force_vectors
}

pub fn change_of_momentum(
    object_values: &ObjectGravitationalForceValues,
    other_objects_values: &[ObjectGravitationalForceValues],
    time_interval: Quantity<Seconds>,
) -> VectorValue<KilogramMeterPerSecond> {
    let resulting_force = resulting_gravitational_force(
        object_values,
        other_objects_values,
    );

    (resulting_force * time_interval).parse()
}

pub fn velocity_change(
    object_values: ObjectGravitationalForceValues,
    other_objects_values: &[ObjectGravitationalForceValues],
    time_interval: Quantity<Seconds>,
) -> VectorValue<KilometersPerSecond> {
    let change_of_momentum = change_of_momentum(
        &object_values,
        other_objects_values,
        time_interval,
    );

    let velocity_change = (change_of_momentum / object_values.mass())
        .parse::<MetersPerSecond>();

    velocity_change.to_kilometers_per_second()
}

pub fn end_position_after_moving(
    initial_velocity: VectorValue<KilometersPerSecond>,
    velocity_change: VectorValue<KilometersPerSecond>,
    time_interval: Quantity<Seconds>,
    starting_position: Point<Quantity<Kilometers>>,
) -> Point<Quantity<Kilometers>> {
    let average_velocity = (initial_velocity + velocity_change / 2.)
        .parse::<KilometersPerSecond>();

    let moving = (average_velocity.clone() * time_interval)
        .parse::<Kilometers>()
        .to_quantity_vector();

    Point {
        x: (starting_position.x + moving.x).parse(),
        y: (starting_position.y + moving.y).parse(),
    }
}

pub fn orbital_velocity(
    bigger_object_mass: Quantity<Kilograms>,
    orbit: Quantity<Kilometers>,
) -> Quantity<KilometersPerSecond> {
    let velocity_value_in_meters_per_second =
        (G * bigger_object_mass / orbit.to_meters()).value().sqrt();

    Quantity::new(MetersPerSecond::new(velocity_value_in_meters_per_second))
        .to_kilometers_per_second()
}