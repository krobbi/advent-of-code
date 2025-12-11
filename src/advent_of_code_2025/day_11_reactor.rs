//! [Day 11: Reactor][link]
//!
//! [link]: https://adventofcode.com/2025/day/11

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are having some trouble with the reactor. They have installed a
    // new server rack, but it can't communicate with the reactor. The server
    // connects to the reactor through a network of devices connected with data
    // cables. The data cables are one-way, the data cannot flow backwards. One
    // specific path between the device we are looking at (labelled "you") and
    // the main output to the reactor (labelled "out") is causing an issue. The
    // Elves want to know how many paths exist between this device and the main
    // output.

    // This sounds like traversing a directed (hopefully acyclic) graph.
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
