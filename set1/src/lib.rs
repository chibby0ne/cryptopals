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

// Convert hex string to 8-bit binary
fn hex_string_to_binary(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut binary = String::new();
    for b in input.chars() {
        let res = match b {
            '0'..='9' | 'a'..='f' | 'A'..='F' => Ok(format!("{:08b}", b as u32)),
            _ => Err(InvalidHexCharFoundError { ch: char::from(b) }),
        };
        binary.push_str(&res?);
    }
    Ok(binary)
}

fn add_base64_char(s: &mut String, slice: &str) {
    if let Ok(value) = u32::from_str_radix(&format!("{:0<6}", slice), 2) {
        match value {
            0..=25 => {
                eprintln!("It's a upper case letter");
                s.push(char::from_u32(value + b'A' as u32).unwrap())
            },
            26..=51 => {
                eprintln!("It's a lower case letter");
                s.push(char::from_u32(value - 26 + b'a' as u32).unwrap())
            },
            52..=61 => {
                eprintln!("It's a number");
                s.push(char::from_u32(value - 4 as u32).unwrap())
            },
            62 => s.push('+'),
            63 => s.push('/'),
            _ => panic!("this is way bigger than 2**6"),
        }
    }
}


// Convert hex to base64
fn base64_from_string(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut result = String::new();
    let binary_input = hex_string_to_binary(input)?;
    dbg!(&binary_input);
    let mut chars_in_block = 0;
    for i in (0..binary_input.len()).step_by(6) {
        match binary_input.get(i..i + 6) {
            Some(slice) => {
                dbg!(slice);
                add_base64_char(&mut result, slice);
                chars_in_block += 1;
            },
            None =>  {
                add_base64_char(&mut result, binary_input.get(i..).unwrap());
                chars_in_block += 1;
                while chars_in_block != 4 {
                    result.push('=');
                    chars_in_block += 1;
                }
            },
        };
        if chars_in_block == 4 {
            chars_in_block = 0;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_octal() {
        let res = hex_string_to_binary("2");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), String::from("00110010"));
    }

    #[test]
    fn test_base64_from_string_4() {
        let input = "4";
        let expected_output = "NA==";
        let res = base64_from_string(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }

    #[test]
    fn test_base64_from_string_42() {
        let input = "42";
        let expected_output = "NDI=";
        let res = base64_from_string(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }

    #[test]
    fn test_base64_from_string_423() {
        let input = "423";
        let expected_output = "NDIz";
        let res = base64_from_string(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }

    #[test]
    fn test_base64_from_string() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "NDkyNzZkMjA2YjY5NmM2YzY5NmU2NzIwNzk2Zjc1NzIyMDYyNzI2MTY5NmUyMDZjNjk2YjY1MjA2MTIwNzA2ZjY5NzM2ZjZlNmY3NTczMjA2ZDc1NzM2ODcyNmY2ZjZk";
        let res = base64_from_string(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }

    #[test]
    fn test_challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    }
}
