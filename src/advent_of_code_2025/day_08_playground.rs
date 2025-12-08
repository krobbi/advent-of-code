//! [Day 8: Playground][link]
//!
//! [link]: https://adventofcode.com/2025/day/8

use std::{cmp::Reverse, collections::HashSet};

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are decorating the playground with junction boxes
    // (3D co-ordinates) connected by strings of lights. They have a limited
    // number of string lights, so they decide to connect the pair of junction
    // boxes that is closest together, and repeat this process. Any pair of
    // junction boxes can be connected, as long as they are not already
    // *directly* connected to each other. When they run out of lights, they
    // want to find the product of the sizes of the three largest circuits. An
    // unconnected junction box is a circuit with a size of 1. Two junction
    // boxes connected in a line is a circuit with a size of 2. A triangle or
    // line of three junction boxes is a circuit with a size of 3, etc.
    let Some(mut junction_boxes) = parse_input(input) else {
        return Solution::ParseError;
    };

    // The elves have 1000 string lights.
    let mut string_light_count = 1000;

    while string_light_count > 0 {
        let Some((first_index, second_index)) = find_shortest_unconnected_pair(&junction_boxes)
        else {
            break;
        };

        junction_boxes[first_index].connect(second_index);
        junction_boxes[second_index].connect(first_index);
        string_light_count -= 1;
    }

    let mut connected_junction_boxes = HashSet::new();
    let mut circuit_sizes = Vec::new();

    for index in 0..junction_boxes.len() {
        if connected_junction_boxes.contains(&index) {
            continue;
        }

        let mut indices = vec![index];
        let mut circuit_size = 0;

        while let Some(index) = indices.pop() {
            if connected_junction_boxes.contains(&index) {
                continue;
            }

            connected_junction_boxes.insert(index);
            indices.extend_from_slice(&junction_boxes[index].connections);
            circuit_size += 1;
        }

        circuit_sizes.push(circuit_size);
    }

    circuit_sizes.sort_by_key(|k| Reverse(*k));
    (circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]).into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // The elves definitely don't have enough extension cables. They need to
    // continue connecting junction boxes until they are all in one large
    // circuit. The Elves want to know the product of the X co-ordinates of the
    // last two connected junction boxes.
    let Some(mut junction_boxes) = parse_input(input) else {
        return Solution::ParseError;
    };

    while let Some((first_index, second_index)) = find_shortest_unconnected_pair(&junction_boxes) {
        junction_boxes[first_index].connect(second_index);
        junction_boxes[second_index].connect(first_index);

        if is_one_circuit(&junction_boxes) {
            let first_x = junction_boxes[first_index].position.0;
            let second_x = junction_boxes[second_index].position.0;
            return (first_x * second_x).into();
        }
    }

    Solution::SolveError
}

/// Finds the shortest unconnected pair of indices in a slice of
/// [`JunctionBox`]es. This function returns [`None`] if there are no
/// unconnected pairs.
fn find_shortest_unconnected_pair(junction_boxes: &[JunctionBox]) -> Option<(usize, usize)> {
    let mut best_pair = None;
    let mut best_distance = u64::MAX;

    for first_index in 0..junction_boxes.len() - 1 {
        for second_index in (first_index + 1)..junction_boxes.len() {
            let first_junction_box = &junction_boxes[first_index];
            let second_junction_box = &junction_boxes[second_index];
            let distance = first_junction_box
                .position
                .distance_squared_to(&second_junction_box.position);

            if distance >= best_distance {
                continue;
            }

            if first_junction_box.is_connected_to(second_index) {
                continue;
            }

            best_distance = distance;
            best_pair = Some((first_index, second_index));
        }
    }

    best_pair
}

/// Returns `true` if a slice of [`JunctionBox`]es forms a single circuit.
fn is_one_circuit(junction_boxes: &[JunctionBox]) -> bool {
    let mut visited_indices = HashSet::new();
    let mut indices = vec![0];

    while let Some(index) = indices.pop() {
        if visited_indices.contains(&index) {
            continue;
        }

        let junction_box = &junction_boxes[index];

        if junction_box.connections.is_empty() {
            return false;
        }

        visited_indices.insert(index);
        indices.extend_from_slice(&junction_box.connections);
    }

    visited_indices.len() == junction_boxes.len()
}

/// A junction box.
struct JunctionBox {
    /// The `JunctionBox`'s [`Position`].
    position: Position,

    /// The indices of `JunctionBox`es connected to this `JunctionBox`.
    connections: Vec<usize>,
}

impl JunctionBox {
    /// Creates a new `JunctionBox` from its [`Position`].
    fn new(position: Position) -> Self {
        Self {
            position,
            connections: Vec::new(),
        }
    }

    /// Connects the `JunctionBox` to a `JunctionBox` index.
    fn connect(&mut self, index: usize) {
        self.connections.push(index);
    }

    /// Returns true if the `JunctionBox` is connected to a `JunctionBox` index.
    fn is_connected_to(&self, index: usize) -> bool {
        self.connections.contains(&index)
    }
}

/// A 3D position.
struct Position(u32, u32, u32);

impl Position {
    /// Returns the distance squared to another `Position`.
    fn distance_squared_to(&self, other: &Position) -> u64 {
        let x = u64::from(self.0.abs_diff(other.0));
        let y = u64::from(self.1.abs_diff(other.1));
        let z = u64::from(self.2.abs_diff(other.2));
        x * x + y * y + z * z
    }
}

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
