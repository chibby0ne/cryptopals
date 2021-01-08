#![feature(assoc_char_funcs)]
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct InvalidHexCharFoundError {
    ch: char,
}

impl fmt::Display for InvalidHexCharFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Hex Char: {}", self.ch)
    }
}
impl Error for InvalidHexCharFoundError {}

const CHARS_PER_BASE64_BLOCK: usize = 4;
const BASE64_DIGIT_BITWIDTH: usize = 6;

// Convert hex value to 4-bit binary value in string representation
fn hex_to_binary(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut binary = String::new();
    for b in input.bytes() {
        let res = match b {
            b'0'..=b'9' => Ok(format!("{:04b}", b - b'0')),
            b'a'..=b'f' => Ok(format!("{:04b}", b - b'a' + 10)),
            b'A'..=b'F' => Ok(format!("{:04b}", b - b'A' + 10)),
            _ => Err(InvalidHexCharFoundError { ch: char::from(b) }),
        };
        binary.push_str(&res?);
    }
    Ok(binary)
}

fn add_base64_char(s: &mut String, slice: &str) {
    if let Ok(value) = u32::from_str_radix(&format!("{:0<6}", slice), 2) {
        match value {
            0..=25 => s.push(char::from_u32(value + b'A' as u32).unwrap()),
            26..=51 => s.push(char::from_u32(value - 26 + b'a' as u32).unwrap()),
            52..=61 => s.push(char::from_u32(value - 52 + b'0' as u32).unwrap()),
            62 => s.push('+'),
            63 => s.push('/'),
            _ => panic!("This char value is bigger than 2^6"),
        }
    }
}

// Encode hex value to base64
fn base64_encode(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut result = String::new();
    let binary_input = hex_to_binary(input)?;
    let mut chars_in_block: usize = 0;
    for i in (0..binary_input.len()).step_by(BASE64_DIGIT_BITWIDTH) {
        match binary_input.get(i..i + BASE64_DIGIT_BITWIDTH) {
            Some(slice) => {
                add_base64_char(&mut result, slice);
                chars_in_block += 1;
            }
            None => {
                add_base64_char(&mut result, binary_input.get(i..).unwrap());
                chars_in_block += 1;
                while chars_in_block != CHARS_PER_BASE64_BLOCK {
                    result.push('=');
                    chars_in_block += 1;
                }
            }
        };
        if chars_in_block == CHARS_PER_BASE64_BLOCK {
            chars_in_block = 0;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binary_4() {
        let res = hex_to_binary("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), String::from("0100"));
    }

    #[test]
    fn test_hex_to_binary_49() {
        let res = hex_to_binary("49");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), String::from("01001001"));
    }

    #[test]
    fn test_hex_to_binary_492() {
        let res = hex_to_binary("492");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), String::from("010010010010"));
    }

    #[test]
    fn test_challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let res = base64_encode(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }
}
