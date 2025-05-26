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
        assert_eq!(
            a * (FFE::ONE / a),
            FFE::ONE
        );

        // x * x
        assert_eq!(a * a, FFE::new([196, 0, 0, 0]));

        let e = U256FFE::<{ U256([2, 0, 0, 0]) }>::new([234, 0, 0, 0]);
        assert_eq!(e * e, U256FFE::<{ U256([2, 0, 0, 0]) }>::new([0; 4]));
    }

    #[test]
    fn ed25519() {
        use crate::ed25519::*;
    }
}
