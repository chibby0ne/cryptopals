/*

Break repeating-key XOR

It is officially on, now.

This challenge isn't conceptually hard, but it involves actual error-prone coding. The other
challenges in this set are there to bring you up to speed. This one is there to qualify you. If you
can do this one, you're probably just fine up to Set 6.

There's a file here. It's been base64'd after being encrypted with repeating-key XOR.

Decrypt it.

Here's how:

    Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.  Write a function
    to compute the edit distance/Hamming distance between two strings. The Hamming distance is just
    the number of differing bits. The distance between:

    this is a test

    and

    wokka wokka!!!

    is 37. Make sure your code agrees before you proceed.  For each KEYSIZE, take the first KEYSIZE
    worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them.
    Normalize this result by dividing by KEYSIZE.  The KEYSIZE with the smallest normalized edit
    distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values.
    Or take 4 KEYSIZE blocks instead of 2 and average the distances.  Now that you probably know
    the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.  Now transpose the blocks:
    make a block that is the first byte of every block, and a block that is the second byte of
    every block, and so on.  Solve each block as if it was single-character XOR. You already have
    code to do this.  For each block, the single-byte XOR key that produces the best looking
    histogram is the repeating-key XOR key byte for that block. Put them together and you have the
    key.

This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR
("Vigenere") statistically is obviously an academic exercise, a "Crypto 101" thing. But more people
"know how" to break it than can actually break it, and a similar technique breaks something much
more important.  No, that's not a mistake.

We get more tech support questions for this challenge than any of the other ones. We promise, there
aren't any blatant errors in this text. In particular: the "wokka wokka!!!" edit distance really is
37.
*/

use super::challenge1::InvalidHexCharFoundError;

fn base64_char_to_binary(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut binary = String::new();
    for byte in input.bytes() {
        let res = match byte {
            b'A'..=b'Z' => Ok(format!("{:06b}", byte - b'A')),
            b'a'..=b'z' => Ok(format!("{:06b}", byte - b'a' + 26)),
            b'0'..=b'9' => Ok(format!("{:06b}", byte - b'0' + 52)),
            b'+' => Ok(format!("{:06b}", 62)),
            b'/' => Ok(format!("{:06b}", 63)),
            b'=' => Ok(format!("")),
            _ => Err(InvalidHexCharFoundError {
                ch: char::from(byte),
            }),
        };
        binary.push_str(&res?);
    }
    Ok(binary)
}

fn add_char(s: &mut String, slice: &str) {
    if let Ok(value) = u32::from_str_radix(&format!("{:0<4}", slice), 2) {
        match value {
            0..=9 => s.push_str(&format!("{}", value)),
            10..=15 => s.push_str(&format!("{:x}", value)),
            _ => panic!(
                "This shouldn't happen: value: {}, string so far: {}",
                value, &s
            ),
        }
    }
}

fn base64_decode(input: &str) -> Result<String, InvalidHexCharFoundError> {
    let mut result = String::new();
    let binary_input = base64_char_to_binary(input)?;
    for i in (0..binary_input.len()).step_by(4) {
        if let Some(slice) = binary_input.get(i..i + 4) {
            add_char(&mut result, slice);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_decode() {
        let input = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let expected_output = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let res = base64_decode(input);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected_output)
    }
}
