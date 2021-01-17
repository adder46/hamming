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

