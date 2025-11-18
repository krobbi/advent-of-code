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
        len_difference += escaped_len(string) - len;
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

/// Returns the escaped length of a string.
fn escaped_len(string: &str) -> usize {
    // 2 characters are added for the surrounding quotes.
    let mut len = 2;

    for char in string.chars() {
        // Quotes and backslashes need a backslash before them, other characters
        // are kept as-is.
        len += match char {
            '"' | '\\' => 2,
            _ => 1,
        };
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {
        check_unescaped_len("\"\"", 2, 0);
        check_unescaped_len("\"abc\"", 5, 3);
        check_unescaped_len("\"aaa\\\"aaa\"", 10, 7);
        check_unescaped_len("\"\\x27\"", 6, 1);
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        check_escaped_len("\"\"", 2, 6);
        check_escaped_len("\"abc\"", 5, 9);
        check_escaped_len("\"aaa\\\"aaa\"", 10, 16);
        check_escaped_len("\"\\x27\"", 6, 11);
    }

    /// Checks that a string has an expected length after unescaping it.
    fn check_unescaped_len(string: &str, before: usize, after: usize) {
        assert_eq!(string.len(), before);
        let unescaped_len = unescaped_len(string).expect("string should be terminated");
        assert_eq!(unescaped_len, after);
    }

    /// Checks that a string has an expected length after escaping it.
    fn check_escaped_len(string: &str, before: usize, after: usize) {
        assert_eq!(string.len(), before);
        assert_eq!(escaped_len(string), after);
    }
}
