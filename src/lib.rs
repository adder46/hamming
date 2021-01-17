use std::ops::{BitOrAssign, BitXorAssign, ShlAssign};
use rand::Rng;
use crate::util::{dec2bin, is_power_of_2};

mod util;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bit(pub u8);

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

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryNumber {
    pub bits: Vec<Bit>,
}

impl BinaryNumber {
    pub fn new(n: u8) -> BinaryNumber {
        BinaryNumber { bits: dec2bin(n) }
    }
  
    pub fn parity_bits(&self) -> Vec<u8> {
        (1..self.bits.len() + 1)
            .map(|i| i as u8)
            .filter(|i| is_power_of_2(*i))
            .collect()
    }

    pub fn compute_check_bits(&self) -> Vec<Bit> {
        let mut c = vec![];
        self.covered_bits().into_iter().for_each(|group| {
            let mut c_n = Bit(0);
            group.into_iter().for_each(|(_, b)| {
                c_n ^= b;
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
        self.bits[random_index] = Bit(self.bits[random_index].0 ^ 1);  
    }

    fn covered_positions(&self) -> Vec<Vec<u8>> {
        let mut cp = vec![];
        self.parity_bits().into_iter().for_each(|pb| {
            cp.push((1..self.bits.len() + 1)
                .map(|i| i as u8)
                .filter(|b| pb & *b == pb)
                .collect());
        });
        cp
    }

    fn covered_bits(&self) -> Vec<Vec<(u8, Bit)>> {
        let mut bits = vec![];
        for positions in self.covered_positions().iter() {
            let mut covered_by_pos = vec![];
            for position in positions.iter() {
                covered_by_pos.push((*position, self.bits[*position as usize - 1]));
            }
            bits.push(covered_by_pos);
        }
        bits
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
    fn parity_bits(input: u8, expected: Vec<u8>) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.parity_bits(), expected);
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
            vec![(1, Bit(1)), (3, Bit(1)), (5, Bit(1)), (7, Bit(1))],
            vec![(2, Bit(0)), (3, Bit(1)), (6, Bit(0)), (7, Bit(1))],
            vec![(4, Bit(0)), (5, Bit(1)), (6, Bit(0)), (7, Bit(1))],
            vec![(8, Bit(0))],
        ])
    )]
    fn covered_bits(input: u8, expected: Vec<Vec<(u8, Bit)>>) {
        let binary_number = BinaryNumber::new(input);
        assert_eq!(binary_number.covered_bits(), expected);
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
    
    #[rstest(
        x,
        case(1),
        case(2),
        case(4),
        case(8),
        case(16),
        case(32),
        case(64),
        case(128)
    )]
    fn power_of_2_should_pass(x: u8) {
        assert!(is_power_of_2(x));
    }

    #[rstest(
        x,
        case(3),
        case(5),
        case(13),
        case(23),
        case(56),
        case(79),
        case(100),
        case(127)
    )]
    fn power_of_2_should_not_pass(x: u8) {
        assert!(!is_power_of_2(x));
    }
}
