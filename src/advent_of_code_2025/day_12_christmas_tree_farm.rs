//! [Day 12: Christmas Tree Farm][link]
//!
//! [link]: https://adventofcode.com/2025/day/12

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are busy decorating the Christmas tree farm, but they are
    // worried that the presents won't fit under all the trees. There is a set
    // of standard shapes for presents, which are made up of connected unit
    // squares (polyominoes). Under each tree is a rectangular region for
    // arranging presents. Each region expects zero or more of each shape to fit
    // in it. Presents may be rotated or flipped to fit better, but cannot be
    // stacked on top of each other. The Elves want to know how many of the
    // regions can actually fit the expected presents.

    // I can't think of a good way to solve this other than trying every
    // possibility until one fits. Flipping or rotating a shape may produce
    // duplicates, which can be removed to possibly speed up the solution.
    let _ = input;
    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "";

    /// Tests part one.
    #[test]
    fn part_one_works() {}

    /// Tests part two.
    #[test]
    fn part_two_works() {}
}
*/
