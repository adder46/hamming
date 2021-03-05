use crate::util::bin2dec;
use hamming::{BinaryNumber, Bit};

mod util;

fn main() {
    /* The code devised by R. W. Hamming is one of the most common error-correcting codes
     * used in RAMs. In the Hamming code, there is an m-bit data word to which r parity bits
     * are added, forming a new codeword of m + r bits. The bit positions are numbered from
     * 1 to m + r. The positions that are powers of two are reserved for parity bits,
     * and the rest are the data bits.
     *
     * For example, if the message we want to send is `1010`, we need to add 3 parity bits.
     * The formula for calculating the number of parity bits needed is: 2^r - m - 1 >= r
     *
     * P₁ covers all bit positions which have the least significant bit set: bit 1 (the parity bit itself), 3, 5, 7, 9, 11.
     * P₂ covers all bit positions which have the second least significant bit set: bit 2 (the parity bit itself), 3, 6, 7, 10, 11, etc.
     * P₄ covers all bit positions which have the third least significant bit set: bits 4–7 and 12.
     * The parity bits are computed by XOR-ing the bits in bit positions they cover.
     *
     * Let's create a message and make space for the parity bits:
     */
    let mut input = BinaryNumber::new(0b1010);
    input.make_space_for_check_bits();

    // Inserting computed parity bits into appropriate positions
    for (index, check_bit) in input
        .check_bit_positions()
        .into_iter()
        .zip(input.compute_check_bits().into_iter())
    {
        input.bits[index as usize - 1] = check_bit;
    }

    // Simulating e.g. a cosmic ray that flipped a single bit during the transmission
    let mut output = input.clone();
    output.flip_bit(3);

    /* When we want to check for the erroneous bit, we need to compute the bitwise XOR
     * of the bits covered by the parity bits. A 0 check bit designates even parity
     * over the checked bits, whereas a 1 designates odd parity. The result C = C₄C₂C₁ = 0000
     * indicates that no errors occurred. However, if C ≠ 0, then the 3-bit number formed
     * by the check bits represents the bit position of the erroneous bit.
     */
    let erroneous_bit = output.compute_check_bits().into_iter().rev().collect();

    println!("input:   {:07b}", bin2dec(input.bits));
    println!("output:  {:07b}", bin2dec(output.bits));
    println!("erroneous bit: {:?}", bin2dec(erroneous_bit));
}
