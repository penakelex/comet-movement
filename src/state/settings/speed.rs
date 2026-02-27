use std::fmt::Display;

use crate::util::physics::quantities::Quantity;
use crate::util::physics::quantities::quantity_units::Seconds;

/// Скорость симуляции
pub enum Speed {
    /// Начальный вариант
    X1(u16),
    X2(u16),
    X4(u16),
    X8(u16),
    X16(u16),
}

impl Speed {
    /// [base_time_between_ticks] -
    /// Базовый промежуток времени между двуями тиками ([`Time`](crate::util::time::Time))
    #[inline(always)]
    pub fn new(base_time_between_ticks: u16) -> Self {
        Self::X1(base_time_between_ticks)
    }
}

impl Speed {
    pub fn set_next(&mut self) {
        let next = match self {
            Speed::X1(time) => Self::X2(*time * 2),
            Speed::X2(time) => Self::X4(*time * 2),
            Speed::X4(time) => Self::X8(*time * 2),
            Speed::X8(time) => Self::X16(*time * 2),
            Speed::X16(time) => Self::X16(*time),
        };

        *self = next;
    }

    pub fn set_previous(&mut self) {
        let previous = match self {
            Speed::X1(time) => Self::X1(*time),
            Speed::X2(time) => Self::X1(*time / 2),
            Speed::X4(time) => Self::X2(*time / 2),
            Speed::X8(time) => Self::X4(*time / 2),
            Speed::X16(time) => Self::X8(*time / 2),
        };

        *self = previous;
    }
}

impl Speed {
    pub fn value(&self) -> Quantity<Seconds> {
        let value = match self {
            Speed::X1(value) => *value,
            Speed::X2(value) => *value,
            Speed::X4(value) => *value,
            Speed::X8(value) => *value,
            Speed::X16(value) => *value,
        };

        Quantity::new(Seconds::new(value as f32))
    }
}

impl Speed {
    pub fn reload(&mut self, base_time_between_ticks: u16) {
        *self = Self::new(base_time_between_ticks)
    }
}

impl Display for Speed {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            fmt,
            "x{value}",
            value = match self {
                Speed::X1(_) => 1,
                Speed::X2(_) => 2,
                Speed::X4(_) => 4,
                Speed::X8(_) => 8,
                Speed::X16(_) => 16,
            }
        )
    }
}
