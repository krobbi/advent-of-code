//! [Day 8: Matchsticks][link]
//!
//! [link]: https://adventofcode.com/2015/day/8

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Find the length of the strings in Santa's list, minus the length of the
    // unescaped strings.
    let mut len_difference = 0;

    for string in input.lines() {
        let len = string.len();

        let Some(unescaped_len) = unescaped_len(string) else {
            return Solution::SolveError;
        };

        len_difference += len - unescaped_len;
    }

    len_difference.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now we need to find the difference if we escaped the strings instead.
    let mut len_difference = 0;

    for string in input.lines() {
        let len = string.len();

        // The escaped string is at least 2 characters longer because of the
        // surrounding quotes.
        let mut escaped_len = 2;

        for char in string.chars() {
            // Quotes and backslashes need to be escaped, everything else is
            // fine.
            escaped_len += match char {
                '"' | '\\' => 2,
                _ => 1,
            };
        }

        len_difference += escaped_len - len;
    }

    len_difference.into()
}

/// Returns the unescaped length of a string. This function returns [`None`] if
/// the string is unterminated.
fn unescaped_len(string: &str) -> Option<usize> {
    // Keep track of the length.
    let mut length = 0;

    // Iterate over the characters.
    let mut chars = string.chars();
    chars.next()?; // Skip the opening quote.

    loop {
        let char = chars.next()?;

        // Stop on a closing quote, and skip the next character or hexadecimal
        // digits after a backslash.
        match char {
            '"' => return Some(length),
            '\\' => {
                if chars.next()? == 'x' {
                    chars.next()?;
                    chars.next()?;
                }
            }
            _ => (),
        }

        // One character has been added to the string.
        length += 1;
    }
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
