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

// Convert hex to 4-bit binary
fn hex_to_binary(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut binary = String::new();
    for b in input.bytes() {
        let res = match b {
            b'0'..=b'9' => Ok(format!("{:04b}", b - b'0')),
            b'a'..=b'f' => Ok(format!("{:04b}", b - b'a')),
            b'A'..=b'F' => Ok(format!("{:04b}", b - b'A')),
            _ => Err(InvalidHexCharFoundError { ch: char::from(b) }),
        };
        binary.push_str(&res?);
    }
    Ok(binary)
}

// Convert hex to base64
fn challenge1(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut result = String::new();
    let binary_input = hex_to_binary(input)?;
    for i in (0..binary_input.len()).step_by(6) {
        match binary_input.get(i..i + 5) {
            Some(slice) => {
                if let Ok(value) = u32::from_str_radix(&format!("{}", slice), 2) {
                    match value {
                        0..=25 => result.push(char::from_u32(value + b'A' as u32).unwrap()),
                        26..=51 => result.push(char::from_u32(value - 26 + b'a' as u32).unwrap()),
                        52..=61 => result.push(char::from_u32(value - 52).unwrap()),
                        62 => result.push('+'),
                        63 => result.push('/'),
                        _ => panic!("this is way bigger than 2**6"),
                    }
                }
            }
            None => result.push_str(&format!("{:=<6}", binary_input.get(i..).unwrap())),
        };
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_octal() {
        let res = hex_to_binary("2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), String::from("0010"));
    }

    // #[ignore]
    #[test]
    fn test_challenge1() {
        let input = r#"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"#;
        let expected_output = r#"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"#;
        let res = challenge1(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }
}
