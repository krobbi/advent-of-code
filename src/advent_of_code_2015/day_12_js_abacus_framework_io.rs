//! [Day 12: JSAbacusFramework.io][link]
//!
//! [link]: https://adventofcode.com/2015/day/12

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Accounting-Elves need help balancing the books and want to find the
    // sum of all the numbers in their JSON document. Luckily no numbers appear
    // in strings, so the actual structure can be ignored for now.
    let mut sum = 0;
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        if !is_char_number(char) {
            continue;
        }

        let mut number = char.to_string();

        while let Some(char) = chars.next_if(|c| is_char_number(*c)) {
            number.push(char);
        }

        sum += number.parse::<i32>().expect("number should be valid");
    }

    sum.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Returns `true` if a [`char`] is part of a number.
fn is_char_number(char: char) -> bool {
    char.is_ascii_digit() || char == '-'
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
