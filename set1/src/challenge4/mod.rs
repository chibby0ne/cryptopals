/*
Detect single-character XOR

One of the 60-character strings in this file has been encrypted by single-character XOR.

Find it.

(Your code from #3 should help.)
*/

use crate::challenge3::{find_message_and_key, MessageBundle};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn challenge4(filename: &str) -> String {
    let mut possible_lines: Vec<MessageBundle> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(lin) = line {
                possible_lines.push(find_message_and_key(&lin));
            }
        }
    }
    possible_lines.sort_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap());
    possible_lines.iter().last().unwrap().message.clone()
}

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge4() {
        assert_eq!(
            challenge4("src/challenge4/input_challenge4.txt"),
            String::from("Now that the party is jumping\n")
        );
    }
}
