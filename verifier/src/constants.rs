pub(crate) const GAMMA: &str = "gamma";
pub(crate) const BETA: &str = "beta";
pub(crate) const ALPHA: &str = "alpha";
pub(crate) const ZETA: &str = "zeta";

pub const MASK: u8 = 0b11 << 6;
pub const COMPRESSED_POSTIVE: u8 = 0b10 << 6;
pub const COMPRESSED_NEGATIVE: u8 = 0b11 << 6;
pub const COMPRESSED_INFINITY: u8 = 0b01 << 6;

#[derive(Debug, PartialEq, Eq)]
pub enum CompressedPointFlag {
    Positive = COMPRESSED_POSTIVE as isize,
    Negative = COMPRESSED_NEGATIVE as isize,
    Infinity = COMPRESSED_INFINITY as isize,
}

impl From<u8> for CompressedPointFlag {
    fn from(val: u8) -> Self {
        match val {
            COMPRESSED_POSTIVE => CompressedPointFlag::Positive,
            COMPRESSED_NEGATIVE => CompressedPointFlag::Negative,
            COMPRESSED_INFINITY => CompressedPointFlag::Infinity,
            _ => panic!("Invalid compressed point flag"),
        }
    }
}

impl From<CompressedPointFlag> for u8 {
    fn from(value: CompressedPointFlag) -> Self {
        value as u8
    }
}
