fn main() {}

//TODO: correctly handle inputs where length % 3 != 0
pub fn hex_to_base64(input: &str) -> String {
    fn decode(chunk: &[u8]) -> [u8; 3] {
        let mut iter = chunk
            .iter()
            .map(|byte| (*byte as char).to_digit(16).unwrap() as u8);
        let mut chunk = [0u8; 3];
        for byte in chunk.iter_mut() {
            *byte = iter.next().unwrap_or(0);
        }
        chunk
    }

    fn convert(hex: [u8; 3]) -> [u8; 2] {
        [
            (hex[0] << 2) | (hex[1] >> 2),
            ((hex[1] & 0b11) << 4) | hex[2],
        ]
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

    let mut result = String::new();
    for chunk in input.as_bytes().chunks(3) {
        let data = decode(chunk);
        convert(data)
            .iter()
            .map(|&byte| char::from_u32(encode(byte) as u32).unwrap())
            .for_each(|char| result.push(char));
    }
    result
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
