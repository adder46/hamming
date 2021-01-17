use crate::util::bin2dec;
use hamming::*;

mod util;

fn main() {
    let mut input = BinaryNumber::new(0b1010);

    for (index, pb) in input
        .parity_bits()
        .iter()
        .zip(input.compute_check_bits().iter())
    {
        input.insert(*index as usize, *pb);
    }

    let mut output = input.clone();
    output.flip_random_bit();

    let mut eb = Bit(0);

    output.compute_check_bits().iter().rev().for_each(|bit| {
        eb <<= Bit(1);
        eb |= *bit;
    });

    println!("input:   {:07b}", bin2dec(input.bits));
    println!("output:  {:07b}", bin2dec(output.bits.clone()));
    println!("error in bit: {:?}", eb);
}
