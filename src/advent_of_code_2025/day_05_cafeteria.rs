//! [Day 5: Cafeteria][link]
//!
//! [link]: https://adventofcode.com/2025/day/5

use std::ops::RangeInclusive;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The kitchen Elves have a new inventory management system and want to know
    // how many of their ingredients are fresh. Their database consists of
    // inclusive ranges of ingredient IDs that are fresh, a blank line, and then
    // lines with an available ingredient ID. An ingredient is fresh if it is in
    // any of the ranges, and spoiled if it is not.
    let Some((ranges, ids)) = parse_database(input) else {
        return Solution::ParseError;
    };

    for range in ranges {
        println!("{range:?}");
    }

    for id in ids {
        println!("{id}");
    }

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Parses a database from input. This function returns [`None`] if a database
/// could not be parsed.
fn parse_database(input: &str) -> Option<(Box<[RangeInclusive<u64>]>, Box<[u64]>)> {
    let mut lines = input.lines();
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let range = parse_range(line)?;
        ranges.push(range);
    }

    for line in lines {
        let id = line.parse().ok()?;
        ids.push(id);
    }

    Some((ranges.into(), ids.into()))
}

/// Parses a range of ingredient IDs from a line of input. This function returns
/// [`None`] if a range could not be parsed.
fn parse_range(line: &str) -> Option<RangeInclusive<u64>> {
    let mut numbers = line.split('-');
    let start = numbers.next()?.parse().ok()?;
    let end = numbers.next()?.parse().ok()?;
    Some(start..=end)
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
