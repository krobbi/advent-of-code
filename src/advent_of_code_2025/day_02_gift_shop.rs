//! [Day 2: Gift Shop][link]
//!
//! [link]: https://adventofcode.com/2025/day/2

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The gift shop Elves want to find invalid product IDs inside product ID
    // ranges. An invalid product ID is a sequence of repeated digits
    // (e.g. "123123", "44"). They want the sum of every invalid product ID in
    // the database.
    let Some(ranges) = parse_product_id_ranges(input) else {
        return Solution::ParseError;
    };

    let mut sum = 0;

    for (lower, upper) in ranges {
        let mut invalid_id = InvalidId::next_from_id(lower);

        loop {
            let id = invalid_id.as_id();

            if id > upper {
                break;
            }

            sum += id;
            invalid_id.half += 1;
        }
    }

    sum.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// An invalid product ID for part one.
#[derive(Clone, Copy)]
struct InvalidId {
    /// The digits of the product ID, repeated over two halves.
    half: u64,
}

impl InvalidId {
    /// Returns the current or next invalid product ID from a product ID.
    fn next_from_id(id: u64) -> InvalidId {
        let digit_count = id.ilog10() + 1;
        let magnitude = 10u64.pow(digit_count / 2);

        let half = if digit_count.is_multiple_of(2) {
            id / magnitude
        } else {
            // If digit count is odd, jump up to the next even set of digits.
            magnitude
        };

        let mut invalid_id = InvalidId { half };

        if invalid_id.as_id() < id {
            invalid_id.half += 1;
        }

        invalid_id
    }

    /// Converts the invalid product ID to a full product ID.
    fn as_id(self) -> u64 {
        let magnitude = 10u64.pow(self.half.ilog10() + 1);
        self.half * magnitude + self.half
    }
}

/// Parses a boxed slice of product ID ranges. This function returns [`None`] if
/// the product ID ranges could not be parsed.
fn parse_product_id_ranges(ranges: &str) -> Option<Box<[(u64, u64)]>> {
    let mut parsed_ranges = Vec::new();

    for range in ranges.split(',') {
        let range = parse_product_id_range(range)?;
        parsed_ranges.push(range);
    }

    Some(parsed_ranges.into())
}

/// Parses a product ID range. This function returns [`None`] if the product ID
/// range could not be parsed.
fn parse_product_id_range(range: &str) -> Option<(u64, u64)> {
    let mut range = range.split('-');
    let lower = range.next()?.parse().ok()?;
    let upper = range.next()?.trim_end().parse().ok()?;
    debug_assert!(lower < upper);
    Some((lower, upper))
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
