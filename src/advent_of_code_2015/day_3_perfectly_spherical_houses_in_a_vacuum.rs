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
    // The set of visited houses needs to be kept track of again.
    let mut visited_houses = HashSet::new();

    // Santa's position is kept track of again.
    let (mut santa_x, mut santa_y) = (0, 0);

    // He is also joined by Robo-Santa.
    let (mut robo_x, mut robo_y) = (0, 0);

    // They both deliver a present to the starting house.
    visited_houses.insert((santa_x, santa_y));

    // This time, Santa and Robo-Santa take turns following the directions.
    let mut is_santas_turn: bool = true;

    for direction in input.chars() {
        let (vx, vy) = direction_vector(direction);

        if is_santas_turn {
            santa_x += vx;
            santa_y += vy;
            visited_houses.insert((santa_x, santa_y));
        } else {
            robo_x += vx;
            robo_y += vy;
            visited_houses.insert((robo_x, robo_y));
        }

        is_santas_turn = !is_santas_turn;
    }

    visited_houses.len().into()
}

/// Returns the vector of a direction.
fn direction_vector(direction: char) -> (i16, i16) {
    match direction {
        '^' => (0, 1),
        'v' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        _ => (0, 0),
    }
}
