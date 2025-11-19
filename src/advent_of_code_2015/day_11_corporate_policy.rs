//! [Day 11: Corporate Policy][link]
//!
//! [link]: https://adventofcode.com/2015/day/11

use std::fmt::{self, Display, Formatter, Write as _};

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    let Some(password) = parse_password(input) else {
        return Solution::ParseError;
    };

    println!("{password:?}");
    println!("{password}");
    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// An eight-letter password.
#[derive(Clone, Copy, Debug)]
struct Password {
    /// The letters of the `Password` where "a" is 0 and "z" is 25.
    letters: [u8; 8],
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut string = String::with_capacity(8);

        for letter in self.letters {
            let _ = string.write_char((letter + b'a').into());
        }

        f.write_str(&string)
    }
}

/// Parses a [`Password`] from a string. This function returns [`None`] if a
/// [`Password`] could not be parsed.
fn parse_password(input: &str) -> Option<Password> {
    let input = input.as_bytes();

    if input.len() < 8 {
        return None;
    }

    let mut letters = [0; 8];

    for index in 0..8 {
        let letter = input[index];

        if !letter.is_ascii_lowercase() {
            return None;
        }

        letters[index] = letter - b'a';
    }

    Some(Password { letters })
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {}

    /// Tests part two.
    #[test]
    fn part_two_works() {}
}
*/
