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
    let Some(junction_boxes) = parse_input(input) else {
        return Solution::ParseError;
    };

    for junction_box in junction_boxes {
        println!(
            "({}, {}, {})",
            junction_box.position.0, junction_box.position.1, junction_box.position.2,
        );
    }

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A junction box.
struct JunctionBox {
    /// The `JunctionBox`'s [`Position`].
    position: Position,
}

impl JunctionBox {
    /// Creates a new `JunctionBox` from its [`Position`].
    fn new(position: Position) -> Self {
        Self { position }
    }
}

/// A 3D position.
struct Position(u32, u32, u32);

/// Parses a boxed slice of [`JunctionBox`]es. from input. This function returns
/// [`None`] if the [`JunctionBox`]es could not be parsed.
fn parse_input(input: &str) -> Option<Box<[JunctionBox]>> {
    let mut junction_boxes = Vec::new();

    for line in input.lines() {
        let position = parse_position(line)?;
        junction_boxes.push(JunctionBox::new(position));
    }

    Some(junction_boxes.into_boxed_slice())
}

/// Parses a [`Position`] from a line of input. This function returns [`None`]
/// if a [`Position`] could not be parsed.
fn parse_position(line: &str) -> Option<Position> {
    let mut numbers = line.trim().split(',');
    let x = numbers.next()?.parse().ok()?;
    let y = numbers.next()?.parse().ok()?;
    let z = numbers.next()?.parse().ok()?;
    Some(Position(x, y, z))
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
