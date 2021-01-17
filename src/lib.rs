use crate::util::dec2bin;
use rand::Rng;
use std::ops::{BitOrAssign, BitXor, BitXorAssign, ShlAssign};

mod util;

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryNumber {
    pub bits: Vec<Bit>,
}

impl BinaryNumber {
    pub fn new(n: u8) -> BinaryNumber {
        BinaryNumber { bits: dec2bin(n) }
    }

    pub fn check_bit_positions(&self) -> Vec<u8> {
        (1u8..self.bits.len() as u8 + 1)
            .filter(|i| i.is_power_of_two())
            .collect()
    }

    pub fn compute_check_bits(&self) -> Vec<Bit> {
        let mut c = vec![];
        self.bits_at_covered_positions()
            .into_iter()
            .for_each(|group| {
                let mut c_n = Bit(0);
                group.into_iter().for_each(|bit| {
                    c_n ^= bit;
                });
                c.push(c_n);
            });
        c
    }

    pub fn insert(&mut self, index: usize, bit: Bit) {
        self.bits.insert(index - 1, bit);
    }

    pub fn flip_random_bit(&mut self) {
        let random_index = rand::thread_rng().gen_range(0..self.bits.len());
        self.bits[random_index] = self.bits[random_index] ^ Bit(1);
    }

    fn covered_positions(&self) -> Vec<Vec<u8>> {
        let mut cp = vec![];
        self.check_bit_positions().into_iter().for_each(|cbp| {
            cp.push(
                (1u8..self.bits.len() as u8 + 1)
                    .filter(|bit| cbp & *bit == cbp)
                    .collect(),
            );
        });
        cp
    }

    fn bits_at_covered_positions(&self) -> Vec<Vec<Bit>> {
        let mut bits = vec![];
        self.covered_positions().into_iter().for_each(|pos_group| {
            let mut covered = vec![];
            pos_group.into_iter().for_each(|pos| {
                covered.push(self.bits[pos as usize - 1]);
            });
            bits.push(covered);
        });
        bits
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bit(pub u8);

impl BitXor for Bit {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bit(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ShlAssign for Bit {
    fn shl_assign(&mut self, rhs: Self) {
        self.0 <<= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::bin2dec;
    use rstest::*;

    #[rstest(
        input, expected,
        case(0, vec![Bit(0)]),
        case(1, vec![Bit(1)]),
        case(3, vec![Bit(1), Bit(1)]),
        case(8, vec![Bit(1), Bit(0), Bit(0), Bit(0)]),
    )]
    fn binary_number_creation(input: u8, expected: Vec<Bit>) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.bits, expected);
    }

    #[rstest(
        input, expected,
        case(0b10, vec![1, 2]),
        case(0b10101, vec![1, 2, 4]),
        case(0b10101010, vec![1, 2, 4, 8]),
    )]
    fn check_bits(input: u8, expected: Vec<u8>) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.check_bit_positions(), expected);
    }

    #[rstest(
        input, expected,
        case(0b10101010, vec![
            vec![1, 3, 5, 7],
            vec![2, 3, 6, 7],
            vec![4, 5, 6, 7],
            vec![8],
        ])
    )]
    fn covered_positions(input: u8, expected: Vec<Vec<u8>>) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.covered_positions(), expected);
    }

    #[rstest(
        input, expected,
        case(0b10101010, vec![
            vec![Bit(1), Bit(1), Bit(1), Bit(1)],
            vec![Bit(0), Bit(1), Bit(0), Bit(1)],
            vec![Bit(0), Bit(1), Bit(0), Bit(1)],
            vec![Bit(0)],
        ])
    )]
    fn bits_at_covered_positions(input: u8, expected: Vec<Vec<Bit>>) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.bits_at_covered_positions(), expected);
    }

    #[rstest(
        input, expected,
        case(vec![Bit(0)], 0),
        case(vec![Bit(1)], 1),
        case(vec![Bit(1), Bit(1)], 3),
        case(vec![Bit(1), Bit(0), Bit(0), Bit(0)], 8),
    )]
    fn binary2decimal(input: Vec<Bit>, expected: u8) {
        assert_eq!(bin2dec(input), expected);
    }

    #[rstest(
        input, expected,
        case(0, vec![Bit(0)]),
        case(1, vec![Bit(1)]),
        case(3, vec![Bit(1), Bit(1)]),
        case(8, vec![Bit(1), Bit(0), Bit(0), Bit(0)]),
    )]
    fn decimal2binary(input: u8, expected: Vec<Bit>) {
        assert_eq!(dec2bin(input), expected);
    }
}
