use crate::util::bin2dec;
use hamming::*;

mod util;

fn main() {
    let mut input = BinaryNumber::new(0b1010);

    for (index, check_bit) in input
        .check_bit_positions()
        .iter()
        .zip(input.compute_check_bits().iter())
    {
        input.insert(*index as usize, *check_bit);
    }

    let mut output = input.clone();
    output.flip_random_bit();

    let mut erroneous_bit = Bit(0);

    output.compute_check_bits().iter().rev().for_each(|bit| {
        erroneous_bit <<= Bit(1);
        erroneous_bit |= *bit;
    });

    println!("input:   {:07b}", bin2dec(input.bits));
    println!("output:  {:07b}", bin2dec(output.bits));
    println!("erroneous bit: {:?}", erroneous_bit);
}
