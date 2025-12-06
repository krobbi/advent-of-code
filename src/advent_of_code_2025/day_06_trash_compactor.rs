//! [Day 6: Trash Compactor][link]
//!
//! [link]: https://adventofcode.com/2025/day/6

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // We accidentally jumped into the kitchen's garbage chute while re-enacting
    // a movie scene. Luckily, there is a family of cephalopods who can open the
    // door, but they want to know if we can help with the youngest cephalopod's
    // math homework. The worksheet consists of columns of numbers separated
    // with spaces, with a "+" or "*" operator at the end. We need to apply the
    // operators to the corresponding columns, and find the sum of all the
    // columns.
    let mut lines = input.lines();

    let Some(sums) = parse_row(lines.next().unwrap_or("*")) else {
        return Solution::SolveError;
    };

    println!("{sums:?}");
    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Parses a row of numbers from a line of input. This function returns [`None`]
/// if a row could not be parsed.
fn parse_row(line: &str) -> Option<Vec<u64>> {
    let mut row = Vec::new();

    for number in line.split_whitespace() {
        let number = number.parse().ok()?;
        row.push(number);
    }

    Some(row)
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
