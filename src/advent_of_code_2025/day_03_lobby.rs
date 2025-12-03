//! [Day 3: Lobby][link]
//!
//! [link]: https://adventofcode.com/2025/day/3

use std::ops::Range;

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
    let mut sum = 0;

    for bank in banks {
        let bank_len = bank.len();
        let (index, tens) = find_higest_rated_battery(0..bank_len - 1, &bank);
        let (_, units) = find_higest_rated_battery(index + 1..bank_len, &bank);
        sum += u16::from(tens * 10 + units);
    }

    sum.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Finds the highest rated battery in a battery bank between a range of indices
/// and returns its index and joltage rating.
fn find_higest_rated_battery(range: Range<usize>, bank: &[u8]) -> (usize, u8) {
    let start_index = range.start;
    let mut best_offset = 0;
    let mut highest_joltage = 0;

    for (offset, joltage) in bank[range].iter().copied().enumerate() {
        if joltage > highest_joltage {
            highest_joltage = joltage;
            best_offset = offset;

            if joltage == 9 {
                break;
            }
        }
    }

    (start_index + best_offset, highest_joltage)
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 357.into());
    }

    /*
    /// Tests part two.
    #[test]
    fn part_two_works() {}
    */
}
