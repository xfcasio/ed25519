use std::ops::{
    Add, AddAssign, Sub, SubAssign,
    Mul, MulAssign, Div, DivAssign,
    Shl, ShlAssign,
    Rem,
    BitAnd
};

use primitive_types::U256;
use crate::finite_field::*;

type FIELD = U256FFE<ED25519_MODULUS>;

pub const ED25519_MODULUS: U256 = U256([
    0xffffffffffffffed, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff
]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: FIELD,
    y: FIELD
}

pub const GENERATOR: Point = Point::new(
    U256([0xc9562d608f25d51a, 0x692cc7609525a7b2, 0xc0a4e231fdd6dc5c, 0x216936d3cd6e53fe]),
    U256([0x6666666666666658, 0x6666666666666666, 0x6666666666666666, 0x6666666666666666])
);

pub const CURVE_PARAMETER: U256 = U256([
    0x75eb4dca135978a3, 0x00700a4d4141d8ab, 0x8cc740797779e898, 0x52036cee2b6ffe73
]);

impl Point {
    pub const fn new(x: U256, y: U256) -> Self {
        Self {
            x: U256FFE::<ED25519_MODULUS>(x),
            y: U256FFE::<ED25519_MODULUS>(y)
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    // a = -1
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: (self.x * other.y + self.y * other.x) / (FIELD::ONE + CURVE_PARAMETER * self.x * other.x * self.y * other.y),
            y: (self.y * other.y + self.x * other.x) / (FIELD::ONE - CURVE_PARAMETER * self.x * other.x * self.y * other.y),
        }
    }
}
