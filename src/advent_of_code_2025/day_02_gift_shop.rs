//! [Day 2: Gift Shop][link]
//!
//! [link]: https://adventofcode.com/2025/day/2

use std::collections::HashSet;

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
        // Find the number of digits in the lower product ID.
        let digit_count = count_digits(lower);

        // Find the amount to divide the lower product ID by to get half of its
        // digits.
        let half_magnitude = 10u64.pow(digit_count / 2);

        // Find the lowest possible pattern of digits for an invalid product ID
        // that is greater than or equal to the lower product ID.
        let mut pattern = if digit_count.is_multiple_of(2) {
            // If the number of digits is even, then find the most significant
            // and least significant halves of the lower product ID.
            let (big_digits, little_digits) = (lower / half_magnitude, lower % half_magnitude);

            // If the least significant digits are greater than the most
            // significant digits, then the pattern is below the range and
            // should be incremented by one.
            big_digits + u64::from(little_digits > big_digits)
        } else {
            // If the number of digits is odd, then start with the lowest
            // possible pattern for a product ID with one more digit.
            half_magnitude
        };

        // Add up every sequence of repeating digits until the upper product ID
        // is reached.
        let mut sequence = append_pattern(pattern, pattern);

        while sequence <= upper {
            sum += sequence;
            pattern += 1;
            sequence = append_pattern(pattern, pattern);
        }
    }

    sum.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now a product ID is invalid if it contains only a sequence of digits
    // repeated 2 or more times.
    let Some(ranges) = parse_product_id_ranges(input) else {
        return Solution::ParseError;
    };

    // Some sequences of repeated digits may be found more than once. For
    // example, "1" and "11" are both repeated in the range "100-2000". This set
    // collects any duplicates and is created outside of the loop to reduce
    // allocation.
    let mut found_sequences = HashSet::new();
    let mut sum = 0;

    for (lower, upper) in ranges {
        // The repeated pattern can't be greater than the most significant half
        // of the upper product ID.
        let max_pattern = upper / 10u64.pow(count_digits(upper) / 2);

        for pattern in 1..=max_pattern {
            let mut sequence = append_pattern(pattern, pattern);

            while sequence <= upper {
                if sequence >= lower {
                    found_sequences.insert(sequence);
                }

                sequence = append_pattern(sequence, pattern);
            }
        }

        sum += found_sequences.drain().sum::<u64>();
    }

    sum.into()
}

/// Returns the number of digits in a number.
fn count_digits(number: u64) -> u32 {
    number.ilog10() + 1
}

/// Appends a pattern of digits to a sequence of digits.
fn append_pattern(sequence: u64, pattern: u64) -> u64 {
    sequence * 10u64.pow(count_digits(pattern)) + pattern
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The example ranges for testing.
    static RANGES: &str = "11-22,95-115,998-1012,1188511880-1188511890,\
        222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
        565653-565659,824824821-824824827,2121212118-2121212124";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(RANGES), 1_227_775_554.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(RANGES), 4_174_379_265u64.into());
    }
}
