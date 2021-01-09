/* Fixed XOR

Write a function that takes two equal-length buffers and produces their XOR combination.

If your function works properly, then when you feed it the string:

1c0111001f010100061a024b53535009181c

... after hex decoding, and when XOR'd against:

686974207468652062756c6c277320657965

... should produce:

746865206b696420646f6e277420706c6179

*/

use crate::challenge1::InvalidHexCharFoundError;
use std::fmt;

#[derive(Debug)]
pub struct DifferenceSize {
    a: usize,
    b: usize,
}

#[derive(Debug)]
pub enum XorError {
    DifferenceSizedError(DifferenceSize),
    InvalidHexCharFound(InvalidHexCharFoundError),
}

impl fmt::Display for XorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XorError::DifferenceSizedError(inner) => write!(
                f,
                "strings are of different size: {} and {}",
                inner.a, inner.b
            ),
            XorError::InvalidHexCharFound(inner) => write!(f, "invalid char: {}", inner.ch),
        }
    }
}

fn convert_ascii_byte_to_decimal(num: u8) -> Result<u8, XorError> {
    match num {
        b'0'..=b'9' => Ok(num - b'0'),
        b'a'..=b'f' => Ok(num - b'a' + 10),
        b'A'..=b'F' => Ok(num - b'A' + 10),
        _ => Err(XorError::InvalidHexCharFound(InvalidHexCharFoundError {
            ch: char::from(num),
        })),
    }
}

pub fn fixed_xor(a: &str, b: &str) -> Result<String, XorError> {
    if a.len() != b.len() {
        return Err(XorError::DifferenceSizedError(DifferenceSize {
            a: a.len(),
            b: b.len(),
        }));
    }
    let mut res = String::new();
    for (a_ch, b_ch) in a.bytes().zip(b.bytes()) {
        let char_xor = convert_ascii_byte_to_decimal(a_ch)? ^ convert_ascii_byte_to_decimal(b_ch)?;
        res.push_str(&format!("{:x}", char_xor));
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";
        let res = fixed_xor(a, b);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected.to_string());
    }
}
