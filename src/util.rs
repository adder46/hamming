use crate::Bit;

#[allow(dead_code)]
pub fn bin2dec(bits: Vec<Bit>) -> u8 {
    let mut result = 0;
    bits.into_iter().for_each(|bit| {
        result <<= 1;
        result |= bit.0;
    });
    result
}

#[allow(dead_code)]
pub fn dec2bin(n: u8) -> Vec<Bit> {
    (0..length(n)).rev().map(|i| Bit((n >> i) & 1)).collect()
}

pub fn length(n: u8) -> u8 {
    (n as f32).log2() as u8 + 1
}

#[cfg(test)]
mod tests {
    use crate::Bit;
    use rstest::*;

    #[rstest(
        input, expected,
        case(0, vec![Bit(0)]),
        case(1, vec![Bit(1)]),
        case(3, vec![Bit(1), Bit(1)]),
        case(8, vec![Bit(1), Bit(0), Bit(0), Bit(0)]),
    )]
    fn dec2bin(input: u8, expected: Vec<Bit>) {
        assert_eq!(super::dec2bin(input), expected);
    }

    #[rstest(
        input, expected,
        case(vec![Bit(0)], 0),
        case(vec![Bit(1)], 1),
        case(vec![Bit(1), Bit(1)], 3),
        case(vec![Bit(1), Bit(0), Bit(0), Bit(0)], 8),
    )]
    fn bin2dec(input: Vec<Bit>, expected: u8) {
        assert_eq!(super::bin2dec(input), expected);
    }

    #[rstest(
        input,
        expected,
        case(0b1, 1),
        case(0b10, 2),
        case(0b101, 3),
        case(0b1010, 4),
        case(0b10101, 5),
        case(0b101010, 6),
        case(0b1010101, 7),
        case(0b10101010, 8)
    )]
    fn length(input: u8, expected: u8) {
        assert_eq!(super::length(input), expected);
    }
}
