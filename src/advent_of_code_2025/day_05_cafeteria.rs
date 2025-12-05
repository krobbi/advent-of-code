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

    ids.iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
        .into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now the Elves want to know how many possible ingredient IDs are fresh.
    // Unfortunately, some of the ranges of fresh ingredient IDs are
    // overlapping.
    let Some((ranges, _)) = parse_database(input) else {
        return Solution::ParseError;
    };

    let mut ranges: Vec<_> = ranges.iter().map(IdRange::new).collect();
    let mut merged_ranges: Vec<IdRange> = Vec::new();

    'merge_range: while let Some(range) = ranges.pop() {
        let mut checked_ranges = Vec::with_capacity(merged_ranges.len());

        while let Some(mut merged_range) = merged_ranges.pop() {
            if merged_range.merge(&range) {
                ranges.push(merged_range);
                merged_ranges.append(&mut checked_ranges);
                continue 'merge_range;
            }

            checked_ranges.push(merged_range);
        }

        merged_ranges.append(&mut checked_ranges);
        merged_ranges.push(range);
    }

    merged_ranges.iter().map(IdRange::size).sum::<u64>().into()
}

/// A non-overlapping range of ingredient IDs for part two.
struct IdRange {
    /// The start ingredient ID.
    start: u64,

    /// The end ingredient ID.
    end: u64,
}

impl IdRange {
    /// Creates a new `IdRange` from a range of ingredient IDs.
    fn new(range: &RangeInclusive<u64>) -> Self {
        Self {
            start: *range.start(),
            end: *range.end(),
        }
    }

    /// Returns the size of the `IdRange` in ingredient IDs.
    fn size(&self) -> u64 {
        self.end - self.start + 1
    }

    /// Attempts to merge another `IdRange` into the `IdRange`. This function
    /// returns `true` if a merge occurred.
    fn merge(&mut self, other: &IdRange) -> bool {
        if other.start <= self.end + 1 && other.end >= self.start - 1 {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
            true
        } else {
            false
        }
    }
}

/// Parses a database from input. This function returns [`None`] if a database
/// could not be parsed.
#[expect(clippy::type_complexity)]
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 3.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 14.into());
    }
}
