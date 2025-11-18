//! [Day 10: Elves Look, Elves Say][link]
//!
//! [link]: https://adventofcode.com/2015/day/10

use std::fmt::Write as _;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Play look-and-say on the input string 40 times.
    let mut input = input.to_owned();

    for _ in 0..40 {
        input = look_and_say(&input);
    }

    input.len().into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Returns a new look-and-say string from an input string.
fn look_and_say(input: &str) -> String {
    let mut chars = input.chars();
    let mut current_char = chars
        .next()
        .expect("input string should be at least one character long");

    let mut count = 1;
    let mut output = String::new();

    for char in chars {
        if char == current_char {
            count += 1;
        } else {
            let _ = write!(output, "{count}{current_char}");
            current_char = char;
            count = 1;
        }
    }

    format!("{output}{count}{current_char}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    /*
    /// Tests part two.
    #[test]
    fn part_two_works() {}
    */
}
