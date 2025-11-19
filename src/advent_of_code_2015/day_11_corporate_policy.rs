//! [Day 11: Corporate Policy][link]
//!
//! [link]: https://adventofcode.com/2015/day/11

use std::fmt::{self, Display, Formatter};

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa needs to change his password. Usually he increments it to get a new
    // one, but the new Security-Elf has some rules about passwords:
    // * Must contain a straight of 3 letters (e.g. "abc")
    // * Must not contain "i", "o", or "l"
    // * Must contain at lest two different pairs of letters
    //   (e.g. "aa" and "bb")
    // The next password that meets these rules must be found.
    let Some(mut password) = parse_password(input) else {
        return Solution::ParseError;
    };

    loop {
        password.increment();
        password.clean();

        if password.is_valid() {
            break;
        }
    }

    password.to_string().into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Returns `true` if a letter is not allowed in a `Password`.
fn is_letter_invalid(letter: u8) -> bool {
    const LETTER_I: u8 = b'i' - b'a';
    const LETTER_L: u8 = b'l' - b'a';
    const LETTER_O: u8 = b'o' - b'a';
    matches!(letter, LETTER_I | LETTER_L | LETTER_O)
}

/// An eight-letter password.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Password {
    /// The letters of the `Password` where `'a'` is `0` and `'z'` is `25`.
    letters: [u8; 8],
}

impl Password {
    /// Increments the `Password`.
    fn increment(&mut self) {
        let mut index = 0;

        loop {
            // Carry to the next letter if it is "z".
            if self.letters[index] == 25 {
                self.letters[index] = 0;
                index = (index + 1) % 8;
            } else {
                self.letters[index] += 1;
                break;
            }
        }
    }

    /// Cleans the `Password` by incrementing past invalid letters.
    /// (e.g. `"hijklmno"` becomes `"hjaaaaaa"`).
    fn clean(&mut self) {
        let mut is_cleaning = false;

        for index in (0..8).rev() {
            if is_cleaning {
                self.letters[index] = 0;
            } else if is_letter_invalid(self.letters[index]) {
                self.letters[index] += 1;
                is_cleaning = true;
            }
        }
    }

    /// Returns `true` if the `Password` is valid.
    /// TODO: Also check for pairs.
    fn is_valid(self) -> bool {
        let mut has_straight = false;

        // First check if the password has a straight. The window is declared in
        // reverse because the password is in reverse order in memory.
        for (c, b, a) in self.letters.windows(3).map(|w| (w[0], w[1], w[2])) {
            if b == a + 1 && c == a + 2 {
                has_straight = true;
                break;
            }
        }

        has_straight
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut chars = self.letters.map(|l| l + b'a');
        chars.reverse();
        f.write_str(str::from_utf8(&chars).expect("password should be valid UTF-8"))
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

    letters.reverse();
    Some(Password { letters })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert!(!is_password_valid("hijklmmn"));
        assert!(!is_password_valid("abbceffg"));
        assert!(!is_password_valid("abbcegjk"));
        assert_eq!(part_one("abcdefgh"), "abcdffaa".into());
        assert_eq!(part_one("ghijklmn"), "ghjaabcc".into());
    }

    /*
    /// Tests part two.
    #[test]
    fn part_two_works() {}
    */

    /// Returns `true` is a [`Password`] is valid from a string.
    fn is_password_valid(input: &str) -> bool {
        let password = parse_password(input).expect("input should be valid");
        let mut cleaned_password = password;
        cleaned_password.clean();

        if password != cleaned_password {
            return false;
        }

        password.is_valid()
    }
}
