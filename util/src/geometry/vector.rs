use iced::Vector;

#[derive(Clone, Debug)]
pub struct VectorValue<T, V> {
    pub value: T,
    pub unit_vector: Vector<V>,
}

impl<T, V> VectorValue<T, V> {
    pub const fn new(value: T, unit_vector: Vector<V>) -> Self {
        Self {
            value,
            unit_vector,
        }
    }
}