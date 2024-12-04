use std::fmt::Display;

use num_traits::FromPrimitive;

use util::physics::quantities::Quantity;
use util::physics::quantities::quantity_units::Seconds;

pub struct Settings {
    pub speed: Speed,
    pub scale: Scale,
    pub running: bool,
}

impl Settings {
    pub const SECONDS_PER_TICK: Quantity<Seconds> = Quantity::new(Seconds::new(7201.));
}

impl Settings {
    pub fn new() -> Self {
        Self {
            speed: Speed::default(),
            scale: Scale::default(),
            running: false,
        }
    }
}

pub enum Speed {
    X1(u16),
    X2(u8),
    X4(u8),
    X8(u8),
    X16(u8),
}

impl Speed {
    pub const fn x1() -> Self {
        Self::X1(320)
    }

    pub const fn x2() -> Self {
        Self::X2(160)
    }

    pub const fn x4() -> Self {
        Self::X4(80)
    }

    pub const fn x8() -> Self {
        Self::X8(40)
    }

    pub const fn x16() -> Self {
        Self::X16(10)
    }
}

impl Speed {
    pub fn next(&self) -> Self {
        match self {
            Speed::X1(_) => Self::x2(),
            Speed::X2(_) => Self::x4(),
            Speed::X4(_) => Self::x8(),
            Speed::X8(_) => Self::x16(),
            Speed::X16(_) => Self::x1(),
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Speed::X1(_) => Self::x16(),
            Speed::X2(_) => Self::x1(),
            Speed::X4(_) => Self::x2(),
            Speed::X8(_) => Self::x4(),
            Speed::X16(_) => Self::x8(),
        }
    }
}

impl Speed {
    pub fn value<T: FromPrimitive>(&self) -> T {
        match self {
            Speed::X1(value) => T::from_u16(*value).unwrap(),
            Speed::X2(value) => T::from_u8(*value).unwrap(),
            Speed::X4(value) => T::from_u8(*value).unwrap(),
            Speed::X8(value) => T::from_u8(*value).unwrap(),
            Speed::X16(value) => T::from_u8(*value).unwrap(),
        }
    }
}

impl Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Speed::X1(_) => "1",
            Speed::X2(_) => "2",
            Speed::X4(_) => "4",
            Speed::X8(_) => "8",
            Speed::X16(_) => "16",
        };
        write!(f, "x{value}")
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self::x16()
    }
}

pub struct Scale {
    pub value: u64,
    pub value_string: String,
}

impl Scale {
    pub const SCALE_CHANGE_FACTOR: u16 = 10_000;
}

impl Default for Scale {
    fn default() -> Self {
        let default_value = 150_000;
        Self {
            value: default_value,
            value_string: default_value.to_string(),
        }
    }
}