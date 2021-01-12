/*
The hex encoded string:

1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good
metric. Evaluate each output and choose the one with the best score.
*/

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LETTER_FREQUENCY: HashMap<char, f64> = {
        let mut map = HashMap::new();
        map.insert('a', 0.082);
        map.insert('b', 0.015);
        map.insert('c', 0.028);
        map.insert('d', 0.043);
        map.insert('e', 0.13);
        map.insert('f', 0.022);
        map.insert('g', 0.02);
        map.insert('h', 0.061);
        map.insert('i', 0.07);
        map.insert('j', 0.0015);
        map.insert('k', 0.0077);
        map.insert('l', 0.04);
        map.insert('m', 0.024);
        map.insert('n', 0.067);
        map.insert('o', 0.075);
        map.insert('p', 0.019);
        map.insert('q', 0.00095);
        map.insert('r', 0.06);
        map.insert('s', 0.063);
        map.insert('t', 0.091);
        map.insert('u', 0.028);
        map.insert('v', 0.0098);
        map.insert('w', 0.024);
        map.insert('x', 0.0015);
        map.insert('y', 0.0015);
        map.insert('z', 0.02);
        map.insert(' ', 0.14);
        map
    };
}

const HEX_ENCODED_STRING: &str =
    "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn one_char_xor(byte: u8, encoded_hex_str: &str) -> Option<String> {
    let mut end_string = String::new();
    for i in (0..encoded_hex_str.len()).step_by(2) {
        if let Some(slice) = encoded_hex_str.get(i..i + 2) {
            let ch = u32::from_str_radix(slice, 16).ok()? ^ byte as u32;
            end_string.push(char::from_u32(ch)?);
        }
    }
    Some(end_string)
}

fn calculate_probability(message: &str) -> f64 {
    message
        .to_lowercase()
        .chars()
        .fold(0.0, |acc, x| acc + LETTER_FREQUENCY.get(&x).unwrap_or(&0.0))
}

fn decode_single_message_with_probability(byte: u8, s: &str) -> Option<MessageBundle> {
    if let Some(message) = one_char_xor(byte, s) {
        let probability = calculate_probability(&message);
        Some(MessageBundle {
            message,
            probability,
            key: byte,
        })
    } else {
        None
    }
}

pub fn find_message_and_key(s: &str) -> MessageBundle {
    let mut possible_messages: Vec<MessageBundle> = vec![];
    for byte in 0..=255 {
        if let Some(message_bundle) = decode_single_message_with_probability(byte, s) {
            possible_messages.push(message_bundle);
        }
    }
    possible_messages.sort_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap());
    possible_messages.iter().last().unwrap().clone()
}

#[derive(Debug, Clone)]
pub struct MessageBundle {
    pub message: String,
    pub key: u8,
    pub probability: f64,
}

fn challenge3() -> MessageBundle {
    find_message_and_key(HEX_ENCODED_STRING)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge3() {
        let res = challenge3();
        assert_eq!(res.key, 88);
        assert_eq!(
            res.message,
            String::from("Cooking MC's like a pound of bacon")
        );
    }
}
