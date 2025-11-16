//! [Day 3: Perfectly Spherical Houses in a Vacuum][link]
//!
//! [link]: https://adventofcode.com/2015/day/3

use std::collections::HashSet;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Keep track of the set of houses which have been visited.
    let mut visited_houses = HashSet::new();

    // Also keep track of Santa's position.
    let (mut x, mut y) = (0i16, 0i16);

    // He has a list of directions.
    for direction in input.chars() {
        // And delivers a present before following the direction.
        visited_houses.insert((x, y));

        let (direction_x, direction_y) = match direction {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        };

        x += direction_x;
        y += direction_y;
    }

    // He also visits a house after his last move.
    visited_houses.insert((x, y));

    // Find the number of houses he visited.
    visited_houses.len().into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}
