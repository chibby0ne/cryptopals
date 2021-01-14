/*

Break repeating-key XOR

It is officially on, now.

This challenge isn't conceptually hard, but it involves actual error-prone coding. The other
challenges in this set are there to bring you up to speed. This one is there to qualify you. If you
can do this one, you're probably just fine up to Set 6.

There's a file here. It's been base64'd after being encrypted with repeating-key XOR.

Decrypt it.

Here's how:

    1. Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.

    2. Write a function to compute the edit distance/Hamming distance between two strings. The
       Hamming distance is just the number of differing bits. The distance between:

       this is a test

       and

       wokka wokka!!!

       is 37. Make sure your code agrees before you proceed.

    3. For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of
       bytes, and find the edit distance between them. Normalize this result by dividing by
       KEYSIZE.

    4. The KEYSIZE with the smallest normalized edit distance is probably the key. You could
       proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2
       and average the distances.

    5. Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.

    6. Now transpose the blocks: make a block that is the first byte of every block, and a block
       that is the second byte of every block, and so on.

    7. Solve each block as if it was single-character XOR. You already have code to do this.  For
       each block, the single-byte XOR key that produces the best looking histogram is the
       repeating-key XOR key byte for that block. Put them together and you have the key.

This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR
("Vigenere") statistically is obviously an academic exercise, a "Crypto 101" thing. But more people
"know how" to break it than can actually break it, and a similar technique breaks something much
more important.  No, that's not a mistake.

We get more tech support questions for this challenge than any of the other ones. We promise, there
aren't any blatant errors in this text. In particular: the "wokka wokka!!!" edit distance really is
37.
*/

use super::challenge1::InvalidHexCharFoundError;
use super::challenge4::read_lines;
use super::challenge5::repeating_xor;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn hamming_distance(s: &str, t: &str) -> usize {
    s.bytes().zip(t.bytes()).fold(0, |acc, pair| {
        acc + format!("{:b}", pair.0 ^ pair.1)
            .chars()
            .filter(|&c| c == '1')
            .count()
    })
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

#[derive(Debug)]
struct KeysizeAverageDistance {
    keysize: usize,
    avg_distance: f64,
}

fn read_file<P: AsRef<Path>>(filename: P) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn find_keysize(s: &str) -> usize {
    let mut avg_distances: Vec<KeysizeAverageDistance> = vec![];
    for keysize in 2..=40 {
        let mut distances: Vec<f64> = Vec::new();
        if let Ok(mut reader) = read_file(s) {
            let mut all_lines: Vec<u8> = Vec::new();
            if let Ok(_) = reader.read_until(b'\0', &mut all_lines) {
                let all_lines_res = String::from_utf8(all_lines);
                if let Ok(all_lines_str) = all_lines_res {
                    for i in (0..all_lines_str.len()).step_by(keysize * 2) {
                        if let Some(s) = all_lines_str.get(i..i + keysize) {
                            if let Some(t) = all_lines_str.get(i + keysize..i + keysize * 2) {
                                distances.push(hamming_distance(s, t) as f64 / keysize as f64);
                            }
                        }
                    }
                }
            }
            let sum_distances = distances.iter().sum::<f64>();
            let avg_distance = sum_distances / distances.len() as f64;
            avg_distances.push(KeysizeAverageDistance {
                keysize,
                avg_distance,
            });
        }
    }
    dbg!(&avg_distances);
    avg_distances.sort_by(|a, b| a.avg_distance.partial_cmp(&b.avg_distance).unwrap());
    avg_distances.iter().nth(0).unwrap().keysize
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

    #[test]
    fn test_hamming_distance() {
        let s = "this is a test";
        let t = "wokka wokka!!!";
        let expected_output: usize = 37;
        assert_eq!(hamming_distance(s, t), expected_output);
    }

    #[test]
    fn test_find_keysize() {
        let input = "src/challenge6/6.txt";
        assert_eq!(find_keysize(input), 20);
    }
}
