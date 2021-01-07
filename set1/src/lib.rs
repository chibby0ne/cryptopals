fn hex_to_octal(input: &str) -> Result<String, String>  {
    let mut v: Vec<u8> = vec![];
    for ch in input.chars().rev() {
        let octal = match ch {
            '0'..='7' => {
                dbg!(ch);
                dbg!(Ok(ch as u8 - b'0'))
            },
            '8'..='9' => Ok((ch as u8 - b'0') % 8),
            'a'..='f' => Ok((ch as u8 - b'a' + 10) % 8),
            'A'..='F' => Ok((ch as u8 - b'A' + 10) % 8),
            _ => Err(format!("Not a valid hex character {}", ch)),
        };
        dbg!(&octal);
        v.insert(0, octal?);
    }
    dbg!(&v);
    match String::from_utf8(v) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

// Convert hex to base64
fn challenge1(input: &str) -> &str {
    let octal_input = hex_to_octal(input);

    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_octal() {
        assert_eq!(hex_to_octal("2"), Ok(format!("2")));
    }

    #[ignore]
    #[test]
    fn test_challenge1() {
        let input =  r#"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"#;
        let expected_output = r#"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"#;
        assert_eq!(challenge1(input), expected_output)
    }
}
