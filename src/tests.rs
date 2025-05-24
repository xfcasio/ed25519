#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use primitive_types::U256;
        use crate::finite_field::U256FFE;

        const MODULUS: U256 = U256([0xffffffffffffffed, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff]);

        type FFE = U256FFE<MODULUS>;

        let a = FFE::new([0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff]);
        let b = FFE::new([0x1ff498ea, 0x9541c5d1, 0xd6f9a6c0, 0xe6f89d1f]);
        let c = FFE::new([14, 0, 0, 0]);

        //println!("{a:?} * {a:?} = {:?}", a * a);
        println!("{c:?} * {c:?} = {:?}", c * c);
    }
}
