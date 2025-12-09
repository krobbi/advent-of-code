//! [Day 9: Movie Theater][link]
//!
//! [link]: https://adventofcode.com/2025/day/9

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are decorating the movie theater by switching out some of the
    // floor (2D coordinates) tiles with red tiles. Some of the floor tiles are
    // already red, and they want to find the area of the largest rectangle
    // whose opposite corners are red. In other words, they want to find the
    // distance squared between the furthest tiles.

    // I was considering searching for one corner on an edge of the bounding
    // rectangle of all of the tiles, but this example shows that it may not
    // always be possible:
    // .....#.....
    // .A.........
    // ...........
    // ...........
    // ...........
    // #.........#
    // ...........
    // ...........
    // ...........
    // .........B.
    // .....#.....
    // The largest area is between 'A' and 'B' with 81 tiles, but 'A' and 'B'
    // are not on any of the bounding edges. Maybe the solution would involve a
    // point furthest from the centre of all the points, but I have decided to
    // check every pair.
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
