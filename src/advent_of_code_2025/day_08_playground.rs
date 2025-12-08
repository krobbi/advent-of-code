//! [Day 8: Playground][link]
//!
//! [link]: https://adventofcode.com/2025/day/8

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are decorating the playground with junction boxes
    // (3D co-ordinates) connected by strings of lights. They have a limited
    // number of string lights, so they decide to connect the pair of junction
    // boxes that is closest together, and repeat this process. Any pair of
    // junction boxes can be connected, as long as they are not already
    // *directly* connected to each other. When they run out of lights, they
    // want to find the product of the sizes of each circuit. An unconnected
    // junction box is a circuit with a size of 1. Two junction boxes connected
    // in a line is a circuit with a size of 2. A triangle or line of three
    // junction boxes is a circuit with a size of 3, etc.
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

    /// Tests part one.
    #[test]
    fn part_one_works() {}

    /// Tests part two.
    #[test]
    fn part_two_works() {}
}
*/
