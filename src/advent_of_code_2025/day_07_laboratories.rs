//! [Day 7: Laboratories][link]
//!
//! [link]: https://adventofcode.com/2025/day/7

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // We tried one of the teleporters from the teleporter hub, but now we're
    // stuck in another room with a broken teleporter. The diagnostics say there
    // is an issue with the tachyon manifold. The tachyon manifold is a grid of
    // characters with empty space ('.') and splitters ('^'). A tachyon beam
    // ('|') starts at the start point ('S') and always moves downard towards
    // the bottom of the manifold. When the beam encounters a splitter, it stops
    // travelling and two new beams appear on the sides of the splitter. To fix
    // the teleporter, we need to find how many times a split occurs.
    let Some(_manifold) = parse_manifold(input) else {
        return Solution::ParseError;
    };

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A tachyon manifold.
struct Manifold {
    /// The width of the `Manifold` in cells.
    width: usize,

    /// The height of the `Manifold` in cells.
    height: usize,

    /// The start X position of the tachyon beam.
    start_x: usize,

    /// The cells of the `Manifold`, which may contain a splitter.
    cells: Box<[bool]>,
}

impl Manifold {
    /// Creates a new `Manifold` from its width, height, and start position.
    fn new(width: usize, height: usize, start_x: usize) -> Self {
        Self {
            width,
            height,
            start_x,
            cells: vec![false; width * height].into_boxed_slice(),
        }
    }

    /// Inserts a splitter into the `Manifold` at a position.
    fn insert_splitter(&mut self, x: usize, y: usize) {
        self.cells[x + y * self.width] = true;
    }
}

/// Parses a [`Manifold`] from input. This function returns [`None`] if a
/// [`Manifold`] could not be parsed.
fn parse_manifold(input: &str) -> Option<Manifold> {
    let grid = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect::<Box<_>>();

    let height = grid.len();
    let width = grid.first()?.len();
    let start_x = grid[0].iter().position(|c| *c == b'S')?;
    let mut manifold = Manifold::new(width, height, start_x);

    for (y, row) in grid.iter().copied().enumerate() {
        for (x, c) in row.iter().copied().enumerate() {
            if c == b'^' {
                manifold.insert_splitter(x, y);
            }
        }
    }

    Some(manifold)
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
