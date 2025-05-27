use primitive_types::U256;
use std::ops::{
    Add, AddAssign, Sub, SubAssign,
    Mul, MulAssign, Div, DivAssign,
    Neg, Shl, ShlAssign,
    Rem,
    BitAnd
};
    
pub trait FiniteField {
    const ZERO: Self;
    const ONE: Self;

    fn ffadd(&self, other: Self) -> Self;
    fn ffsub(&self, other: Self) -> Self;
    fn ffmul(&self, other: Self) -> Self;
    fn ffdiv(&self, other: Self) -> Self;
    fn ffscale(&self, scalar: U256) -> Self;
    fn ffexp(&self, scalar: U256) -> Self;
}
    
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct U256FFE<const P: U256>(pub U256);
    
impl<const P: U256> FiniteField for U256FFE<P> {
    const ZERO: Self = Self(U256([0; 4]));
    const ONE: Self = Self(U256([1, 0, 0, 0]));

    fn ffadd(&self, other: Self) -> Self { U256FFE((self.0 + other.0) % U256::from(P)) }
    fn ffsub(&self, other: Self) -> Self { U256FFE((self.0 + U256::from(P) - other.0) % U256::from(P)) }

    fn ffmul(&self, other: Self) -> Self {
        let mut result = U256::zero();

        let (mut a, mut b) = (self.0, other.0);

        while b != U256::zero() {
            if b.bit(0) { result = (result + a) % P; }

            a = (a << 1) % P;
            b >>= 1;
        }

        Self(result)
    }
    
    fn ffdiv(&self, other: Self) -> Self {
        assert!(P > U256::from(2));
        assert!(other != Self::ZERO);

        *self * other.exp(U256::from(P - 2))
    }
    
    fn ffscale(&self, scalar: U256) -> Self {
        let mut result = U256::zero();

        let (mut a, mut b) = (self.0, scalar);

        while b != U256::zero() {
            if b.bit(0) { result = (result + a) % P }

            a = (a << 1) % P;
            b >>= 1;
        }

        Self(result)
    }
    
    fn ffexp(&self, scalar: U256) -> Self {
        let mut accumulator = Self::ONE;
        let scalar_bits = 256 - scalar.leading_zeros();


        for i in (0..scalar_bits).rev() {
            let full_mul = (accumulator.0.full_mul(accumulator.0) % P).0;
            accumulator.0 = U256([full_mul[0], full_mul[1], full_mul[2], full_mul[3]]);

            let mask = if i <= 128 {
                U256::from(1_u128 << i)
            } else if i <= 192 {
                U256([0, 0, 1_u64 << (i - 128), 0])
            } else {
                U256([0, 0, 0, 1_u64 << (i - 192)])
            };

            if (scalar & mask) != U256::zero() {
                let fm = (accumulator.0.full_mul(self.0) % P).0;
                accumulator.0 = U256([fm[0], fm[1], fm[2], fm[3]]);
            }
            accumulator = accumulator % Self(P.into())
        }

        accumulator
    }
}
    
impl<const P: U256> U256FFE<P> {
    pub const fn new(n: [u64; 4]) -> Self { Self(U256(n)) }
    #[inline] pub fn exp(&self, n: U256) -> Self { self.ffexp(n) }
}
    
impl<const P: U256> Add<U256FFE<P>> for U256FFE<P> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output { self.ffadd(other) }
}
    
impl<const P: U256> Sub<U256FFE<P>> for U256FFE<P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output { self.ffsub(other) }
}

impl<const P: U256> Neg for U256FFE<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (Self(P) - self) % Self(P)
    }
}
    
impl<const P: U256> AddAssign<U256FFE<P>> for U256FFE<P> {
    fn add_assign(&mut self, other: Self) { *self = self.ffadd(other) }
}
    
impl<const P: U256> SubAssign<U256FFE<P>> for U256FFE<P> {
    fn sub_assign(&mut self, other: Self) { *self = self.ffsub(other) }
}
    
impl<const P: U256> Mul<U256> for U256FFE<P> {
    type Output = U256FFE<P>;

    fn mul(self, other: U256) -> Self::Output { self.ffscale(other) }
}
    
impl<const P: U256> Mul<U256FFE<P>> for U256 {
    type Output = U256FFE<P>;

    fn mul(self, other: U256FFE<P>) -> Self::Output { other.ffscale(self) }
}
    
impl<const P: U256> Mul<i32> for U256FFE<P> {
    type Output = U256FFE<P>;

    fn mul(self, other: i32) -> Self::Output { self.ffscale(other.try_into().expect("scaling by U256")) }
}
    
impl<const P: U256> Mul<U256FFE<P>> for i32 {
    type Output = U256FFE<P>;

    fn mul(self, other: U256FFE<P>) -> Self::Output { other.ffscale(self.try_into().expect("scaling by U256")) }
}
    
impl<const P: U256> MulAssign<U256> for U256FFE<P> {
    fn mul_assign(&mut self, other: U256) { *self = self.ffscale(other.try_into().expect("scaling by U256")) }
}
    
impl<const P: U256> Mul<U256FFE<P>> for U256FFE<P> {
    type Output = Self;

    fn mul(self, other: U256FFE<P>) -> Self::Output { self.ffmul(other) }
}
    
impl<const P: U256> MulAssign<U256FFE<P>> for U256FFE<P> {
    fn mul_assign(&mut self, other: U256FFE<P>) { *self = self.ffmul(other); }
}
    
impl<const P: U256> Div<U256FFE<P>> for U256FFE<P> {
    type Output = Self;

    fn div(self, other: U256FFE<P>) -> Self::Output { self.ffdiv(other) }
}
    
impl<const P: U256> DivAssign<U256FFE<P>> for U256FFE<P> {
    fn div_assign(&mut self, other: U256FFE<P>) { *self = self.ffdiv(other); }
}
    
impl<const P: U256> Shl<i32> for U256FFE<P> {
    type Output = U256FFE<P>;

    fn shl(self, other: i32) -> Self::Output {
        Self(self.0 << other)
    }
}
    
impl<const P: U256> BitAnd<U256FFE<P>> for U256FFE<P> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self(self.0 & other.0)
    }
}
    
impl<const P: U256> BitAnd<u32> for U256FFE<P> {
    type Output = U256FFE<P>;

    fn bitand(self, other: u32) -> Self::Output {
        Self(self.0 & U256::from(other))
    }
}
    
impl<const P: U256> ShlAssign<i32> for U256FFE<P> {
    fn shl_assign(&mut self, other: i32) {
        self.0 <<= other;
    }
}
    
impl<const P: U256> Rem<U256FFE<P>> for U256FFE<P> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        Self(self.0 % other.0)
    }
}
    
impl<const P: U256> Into<U256FFE<P>> for i32 {
    fn into(self) -> U256FFE<P> {
        U256FFE(U256::from(self))
    }
}
