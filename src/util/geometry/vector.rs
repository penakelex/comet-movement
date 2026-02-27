use iced::Vector;

/// Структура вектора через единичный вектор
#[derive(Clone)]
pub struct VectorValue<T, V> {
    /// Значение вектора
    pub value: T,
    /// Единичный вектор
    pub unit_vector: Vector<V>,
}

impl<T, V> VectorValue<T, V> {
    #[inline(always)]
    pub const fn new(
        value: T,
        unit_vector: Vector<V>,
    ) -> Self {
        Self { value, unit_vector }
    }
}
