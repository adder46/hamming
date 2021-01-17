use crate::Bit;

pub fn bin2dec(bits: Vec<Bit>) -> u8 {
    let mut result = 0;
    bits.into_iter().for_each(|b| {
        result <<= 1;
        result |= b.0;
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

pub fn is_power_of_2(n: u8) -> bool {
    n & (n - 1) == 0
}