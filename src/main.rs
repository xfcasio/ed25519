use primitive_types::U256;
use crypto::finite_field::U256FFE;

fn main() {
    const MODULUS: u128 = 71;
    type FFE = U256FFE<MODULUS>;
    let fe = FFE::new([0xfffaf103, 0x94000fb0, 0x39c3d09f, 0xe4a0a3fe]);

    let a = U256([0x1ff498ea, 0x9541c5d1, 0xd6f9a6c0, 0xe6f89d1f]);

    let prod = a * fe;
    println!("{}", 1);
}
