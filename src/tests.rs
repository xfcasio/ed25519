#[cfg(test)]
mod tests {
    use primitive_types::U256;

    #[test]
    fn finite_field() {
        use crate::finite_field::*;
        type FFE = U256FFE<MODULUS>;

        const MODULUS: U256 = U256([
            0xffffffffffffffed, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff
        ]);


        let c = FFE::new([0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff]);
        let a = FFE::new([14, 0, 0, 0]);

        // 1 / x
        assert_eq!(
            FFE::ONE / a,
            FFE::new([0x924924924924923d, 0x4924924924924924, 0x2492492492492492, 0x5249249249249249]),
        );

        // x * (1 / x)
        assert_eq!(a * (FFE::ONE / a), FFE::ONE);

        // x * x
        assert_eq!(a * a, FFE::new([196, 0, 0, 0]));

        let e = U256FFE::<{ U256([2, 0, 0, 0]) }>::new([234, 0, 0, 0]);
        assert_eq!(e * e, U256FFE::<{ U256([2, 0, 0, 0]) }>::new([0; 4]));
    }

    #[test]
    fn ed25519() {
        use crate::ed25519::*;
        use crate::finite_field::*;

        let sum = GENERATOR + GENERATOR;

        assert_eq!(
            GENERATOR + GENERATOR,
            Point::new(
                [0x83c5a14e2843ce0e, 0x080d8e4515d7a45f, 0x3d043b7d1833e7ac, 0x36ab384c9f5a046c], 
                [0x0e5f46ae6af8a3c9, 0x97390f5164385156, 0x1da25ee8c9a21f56, 0x2260cdf3092329c2]
            )
        );

        assert_eq!(
            GENERATOR - GENERATOR,
            Point::identity()
        );
    }
}
