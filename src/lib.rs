use crate::util::dec2bin;
use rand::Rng;
use std::ops::{BitOrAssign, BitXor, BitXorAssign, ShlAssign};

mod util;

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryNumber {
    pub bits: Vec<Bit>,
    payload_length: u8,
}

impl BinaryNumber {
    pub fn new(n: u8) -> BinaryNumber {
        BinaryNumber {
            bits: dec2bin(n),
            payload_length: ((n as f32).log2() + 1.0) as u8,
        }
    }

    pub fn number_of_check_bits(&self) -> u8 {
        (0_u8..)
            .find(|&k| 1_u16 << k >= k as u16 + self.payload_length as u16 + 1)
            .unwrap()
    }

    pub fn check_bit_positions(&self) -> Vec<u8> {
        (0..self.number_of_check_bits()).map(|i| 1 << i).collect()
    }

    pub fn compute_check_bits(&self) -> Vec<Bit> {
        let mut c = vec![];
        self.bits_at_covered_positions()
            .into_iter()
            .for_each(|group| {
                let mut c_n = Bit(0);
                group.into_iter().for_each(|bit| c_n ^= bit);
                c.push(c_n);
            });
        c
    }

    pub fn make_space_for_check_bits(&mut self) {
        self.check_bit_positions().into_iter().for_each(|pos| {
            self.bits.insert(pos as usize - 1, Bit(0));
        });
    }

    pub fn flip_bit(&mut self, index: usize) {
        self.bits[index - 1] ^= Bit(1);
    }

    pub fn flip_random_bit(&mut self) {
        let random_index = rand::thread_rng().gen_range(0..self.bits.len());
        self.bits[random_index] ^= Bit(1);
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
        case(0b1010, vec![1, 2, 4]),
        case(0b10101, vec![1, 2, 4, 8]),
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
        case(0b1010, vec![
            vec![Bit(1), Bit(1)],
            vec![Bit(0), Bit(1)],
            vec![Bit(0)],
        ]),
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
        input,
        expected,
        case(0b10, 3),
        case(0b101, 3),
        case(0b1010, 3),
        case(0b10101, 4),
        case(0b101010, 4),
        case(0b1010101, 4),
        case(0b10101010, 4)
    )]
    fn num_of_check_bits(input: u8, expected: u8) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.number_of_check_bits(), expected);
    }
}
