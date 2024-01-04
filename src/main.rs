use arrayvec::ArrayVec;
use itertools::Itertools;

fn main() {}

pub fn hex_to_base64(input: &str) -> String {
    fn convert(hex: &[u8]) -> [u8; 2] {
        fn left(a: u8, b: u8) -> u8 {
            a << 2 | b >> 2
        }

        fn right(a: u8, b: u8) -> u8 {
            (a & 0b11) << 4 | b
        }

        let len = hex.len();
        match len {
            3 => [left(hex[0], hex[1]), right(hex[1], hex[2])],
            2 => [left(hex[0], hex[1]), right(hex[1], 0)],
            1 => [left(hex[0], 0), 0],
            _ => panic!(),
        }
    }

    fn encode(byte: u8) -> u8 {
        match byte {
            0x00..=0x19 => byte + b'A',
            0x1a..=0x33 => byte - 0x1a + b'a',
            0x34..=0x3d => byte - 0x34 + b'0',
            0x3e => b'+',
            0x3f => b'/',
            _ => panic!(),
        }
    }

    let result_length = input.len() / 3 * 2 + input.len() % 3 + 1;
    let mut result = Vec::with_capacity(result_length);
    for chunk in &input
        .chars()
        .map(|char| char.to_digit(16).unwrap() as u8)
        .chunks(3)
    {
        let hex: ArrayVec<_, 3> = chunk.collect();
        let data = convert(&hex);
        for byte in data {
            result.push(encode(byte));
        }
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let result = hex_to_base64(input);

        assert_eq!(result, expected);
    }
}
