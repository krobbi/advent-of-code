//! [Day 9: All in a Single Night][link]
//!
//! [link]: https://adventofcode.com/2015/day/9

use std::collections::HashMap;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa needs to find the shortest distance to visit each location in a set
    // of locations (travelling salesman problem).
    let Some(mut map) = parse_map(input) else {
        return Solution::ParseError;
    };

    let mut min_distance = usize::MAX;

    for name in map.location_names() {
        find_best_distance(&mut map, &name, 0, &mut min_distance, false);
    }

    min_distance.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now Santa wants to show off and take the longest distance. This is
    // basically the same problem.
    let Some(mut map) = parse_map(input) else {
        return Solution::ParseError;
    };

    let mut max_distance = 0;

    for name in map.location_names() {
        find_best_distance(&mut map, &name, 0, &mut max_distance, true);
    }

    max_distance.into()
}

/// Recursively visits every [`Location`] in a map and updates a best minimum or
/// maximum distance to cover a [`Map`].
fn find_best_distance(
    map: &mut Map,
    name: &str,
    distance: usize,
    best_distance: &mut usize,
    is_max: bool,
) {
    map.visit(name);

    // If the entire map has been visited, then a new best distance may have
    // been found.
    if map.is_all_visited() {
        *best_distance = if is_max {
            (*best_distance).max(distance)
        } else {
            (*best_distance).min(distance)
        };
    }

    for (name, link_distance) in map.links(name) {
        // Don't revisit locations.
        if map.is_visited(&name) {
            continue;
        }

        find_best_distance(map, &name, distance + link_distance, best_distance, is_max);
    }

    map.unvist(name);
}

/// A map of [`Location`]s.
#[derive(Default)]
struct Map {
    /// The map of names to [`Location`]s.
    locations: HashMap<String, Location>,
}

impl Map {
    /// Creates a new `Map`.
    fn new() -> Self {
        Self::default()
    }

    /// Returns the names of every [`Location`].
    fn location_names(&self) -> Box<[String]> {
        self.locations.keys().cloned().collect()
    }

    /// Returns the links from a [`Location`] name.
    fn links(&self, name: &str) -> HashMap<String, usize> {
        self.location(name).links.clone()
    }

    /// Visits a [`Location`] from its name.
    fn visit(&mut self, name: &str) {
        self.location_mut(name).is_visited = true;
    }

    /// Stops visiting a [`Location`] from its name.
    fn unvist(&mut self, name: &str) {
        self.location_mut(name).is_visited = false;
    }

    /// Returns `true` if a [`Location`] is visited from its name.
    fn is_visited(&self, name: &str) -> bool {
        self.location(name).is_visited
    }

    /// Returns `true` if every [`Location`] is visited.
    fn is_all_visited(&self) -> bool {
        for location in self.locations.values() {
            if !location.is_visited {
                return false;
            }
        }

        true
    }

    /// Adds a two-way link between two [`Location`]s from their names.
    fn add_link(&mut self, name_a: &str, name_b: &str, distance: usize) {
        let location_a = self.location_mut(name_a);
        location_a.add_link(name_b, distance);
        let location_b = self.location_mut(name_b);
        location_b.add_link(name_a, distance);
    }

    /// Returns a reference to a [`Location`] from its name.
    fn location(&self, name: &str) -> &Location {
        self.locations.get(name).expect("location should exist")
    }

    /// Returns a mutable reference to a [`Location`] from its name.
    fn location_mut(&mut self, name: &str) -> &mut Location {
        self.locations.entry(name.to_owned()).or_default()
    }
}

/// A location.
#[derive(Default)]
struct Location {
    /// Whether the `Location` is currently visited.
    is_visited: bool,

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
