//! [Day 3: Lobby][link]
//!
//! [link]: https://adventofcode.com/2025/day/3

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The elevators to the rest of the North Pole Base are offline. They need
    // emergency power from battery banks (lines of digits) where each battery
    // has a joltage rating (single digit). Two batteries from each bank can be
    // turned on and the total joltage from the bank is the number as it is read
    // out. The highest joltage from "12345" would be "45", not "54". We need to
    // find the sum of the highest joltages from each bank.
    let banks = parse_banks(input);

    for bank in banks {
        println!("{bank:?}");
    }

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Parses a boxed slice of battery banks from input.
fn parse_banks(input: &str) -> Box<[Box<[u8]>]> {
    input.lines().map(parse_bank).collect()
}

/// Parses a battery bank from a line of input.
fn parse_bank(line: &str) -> Box<[u8]> {
    line.chars()
        .filter(char::is_ascii_digit)
        .map(|c| u8::try_from(c).expect("character should be ASCII") - b'0')
        .collect()
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
