use crate::Bit;

pub fn bin2dec(bits: Vec<Bit>) -> u8 {
    let mut result = 0;
    bits.into_iter().for_each(|bit| {
        result <<= 1;
        result |= bit.0;
    });
    result
}

pub fn dec2bin(n: u8) -> Vec<Bit> {
    (0..((n as f32).log2() as u8 + 1))
        .into_iter()
        .rev()
        .map(|i| Bit((n >> i) & 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

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
