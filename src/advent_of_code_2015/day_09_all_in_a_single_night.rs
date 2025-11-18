//! [Day 9: All in a Single Night][link]
//!
//! [link]: https://adventofcode.com/2015/day/9

use std::collections::HashMap;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa needs to find the shortest distance to visit each location in a set
    // of locations (travelling salesman problem).
    let Some(map) = parse_map(input) else {
        return Solution::ParseError;
    };

    println!("{map:#?}");
    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A map of [`Location`]s.
#[derive(Debug, Default)]
struct Map {
    /// The map of names to [`Location`]s.
    locations: HashMap<String, Location>,
}

impl Map {
    /// Creates a new `Map`.
    fn new() -> Self {
        Self::default()
    }

    /// Adds a two-way link between two [`Location`]s.
    fn add_link(&mut self, name_a: &str, name_b: &str, distance: usize) {
        let location_a = self.location_mut(name_a);
        location_a.add_link(name_b, distance);
        let location_b = self.location_mut(name_b);
        location_b.add_link(name_a, distance);
    }

    /// Returns a mutable reference to a [`Location`] from its name.
    fn location_mut(&mut self, name: &str) -> &mut Location {
        self.locations.entry(name.to_owned()).or_default()
    }
}

/// A location.
#[derive(Debug, Default)]
struct Location {
    /// Whether the `Location` is currently visited.
    visited: bool,

    /// The distances to every linked `Location`.
    links: HashMap<String, usize>,
}

impl Location {
    /// Adds a link from the `Location`.
    fn add_link(&mut self, name: &str, distance: usize) {
        self.links.insert(name.to_owned(), distance);
    }
}

/// Parses a [`Map`] from a list of distances. This function returns [`None`] if
/// a [`Map`] could not be parsed.
fn parse_map(list: &str) -> Option<Map> {
    let mut map = Map::new();

    for item in list.lines() {
        let mut words = item.split(' ');
        let name_a = words.next()?;
        words.next()?; // Skip "to".
        let name_b = words.next()?;
        words.next()?; // Skip "=".
        let distance = words.next()?.parse().ok()?;
        map.add_link(name_a, name_b, distance);
    }

    Some(map)
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
