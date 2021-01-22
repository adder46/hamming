use crate::util::bin2dec;
use hamming::{BinaryNumber, Bit};

mod util;

fn main() {
    let mut input = BinaryNumber::new(0b1010);
    input.make_space_for_check_bits();

    for (index, check_bit) in input
        .check_bit_positions()
        .into_iter()
        .zip(input.compute_check_bits().into_iter())
    {
        input.bits[index as usize - 1] = check_bit;
    }

    let mut output = input.clone();
    output.flip_bit(3);

    let erroneous_bit = bin2dec(output.compute_check_bits().into_iter().rev().collect());

    println!("input:   {:07b}", bin2dec(input.bits));
    println!("output:  {:07b}", bin2dec(output.bits));
    println!("erroneous bit: {:?}", erroneous_bit);
}
