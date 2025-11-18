//! [Day 9: All in a Single Night][link]
//!
//! [link]: https://adventofcode.com/2015/day/9

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa needs to find the shortest distance to visit each location in a set
    // of locations (travelling salesman problem).
    if let Some((shortest_distance, _)) = solve_parts(input) {
        shortest_distance.into()
    } else {
        Solution::ParseError
    }
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now Santa wants to show off and take the longest distance. This is
    // basically the same problem.
    if let Some((_, longest_distance)) = solve_parts(input) {
        longest_distance.into()
    } else {
        Solution::ParseError
    }
}

/// Solves a part and returns the shortest and longest distances. This function
/// returns [`None`] if the list of locations could not be parsed.
fn solve_parts(input: &str) -> Option<(u16, u16)> {
    let graph = parse_graph(input)?;
    let mut visitor = Visitor::new();
    visitor.visit_graph(&graph);
    Some((visitor.shortest_distance, visitor.longest_distance))
}

/// A structure which visits a [`Graph`] and finds the shortest and longest
/// distances through its nodes.
struct Visitor {
    /// The stack of currently visited nodes.
    visited_nodes: Vec<usize>,

    /// The shortest distance through the `Graph`.
    shortest_distance: u16,

    /// The longest distance through the `Graph`.
    longest_distance: u16,
}

impl Visitor {
    /// Creates a new `Visitor`.
    fn new() -> Self {
        Self {
            visited_nodes: Vec::new(),
            shortest_distance: u16::MAX,
            longest_distance: 0,
        }
    }

    /// Visits a [`Graph`].
    fn visit_graph(&mut self, graph: &Graph) {
        for node in 0..graph.node_count {
            self.visit_node(graph, node, 0);
        }
    }

    /// Visits a node in a [`Graph`] with a total distance.
    fn visit_node(&mut self, graph: &Graph, node: usize, distance: u16) {
        self.visited_nodes.push(node);

        if self.visited_nodes.len() == graph.node_count {
            self.shortest_distance = self.shortest_distance.min(distance);
            self.longest_distance = self.longest_distance.max(distance);
        } else {
            for (next_node, link_distance) in graph.distances_from(node).iter().copied().enumerate()
            {
                if next_node == node || self.visited_nodes.contains(&next_node) {
                    continue;
                }

                self.visit_node(graph, next_node, distance + link_distance);
            }
        }

        self.visited_nodes.pop();
    }
}

/// A graph of distances between locations.
struct Graph {
    /// The number of nodes in the `Graph`.
    node_count: usize,

    /// The matrix of distances between nodes.
    links: Vec<u16>,
}

impl Graph {
    /// Creates a new `Graph` with a node count.
    fn new(node_count: usize) -> Self {
        Self {
            node_count,
            links: vec![0; node_count * node_count],
        }
    }

    /// Returns a slice of distances from a node.
    fn distances_from(&self, node: usize) -> &[u16] {
        let index = node * self.node_count;
        &self.links[index..index + self.node_count]
    }
}

/// A map of location names to nodes.
#[derive(Default)]
struct LocationTable {
    /// The location names.
    location_names: Vec<String>,
}

impl LocationTable {
    /// Returns a node from a location name.
    fn node(&mut self, name: &str) -> usize {
        if let Some(node) = self.location_names.iter().position(|n| n == name) {
            node
        } else {
            let node = self.location_names.len();
            self.location_names.push(name.to_owned());
            node
        }
    }
}

/// Parses a [`Graph`] from a list of distances. This function returns [`None`]
/// if a [`Graph`] could not be parsed.
fn parse_graph(input: &str) -> Option<Graph> {
    let mut location_table = LocationTable::default();
    let mut links = Vec::new();

    for line in input.lines() {
        let mut words = line.split(' ');
        let node_a = location_table.node(words.next()?);
        words.next()?; // Skip "to".
        let node_b = location_table.node(words.next()?);
        words.next()?; // Skip "=".
        let distance = words.next()?.parse().ok()?;
        links.push((node_a, node_b, distance));
    }

    let node_count = location_table.location_names.len();
    let mut graph = Graph::new(node_count);

    for (node_a, node_b, distance) in links {
        graph.links[node_a * node_count + node_b] = distance;
        graph.links[node_b * node_count + node_a] = distance;
    }

    Some(graph)
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
