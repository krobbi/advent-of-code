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
    let Some(manifold) = parse_manifold(input) else {
        return Solution::ParseError;
    };

    // Because the beams all move downward at the same speed and direction, only
    // the horizontal position needs to be considered. We use an array of
    // whether a beam exists at a position.
    let mut split_count = 0;
    let mut beams = vec![false; manifold.width].into_boxed_slice();
    beams[manifold.start_x] = true;

    for y in 1..manifold.height {
        for x in 0..manifold.width {
            if beams[x] && manifold.has_splitter(x, y) {
                beams[x - 1] = true;
                beams[x] = false;
                beams[x + 1] = true;
                split_count += 1;
            }
        }
    }

    split_count.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // The broken manifold is actually a quantum manifold. The tachyon beam
    // splits between different timelines at each splitter. We need to find how
    // many timelines (paths through the manifold) there are.
    let Some(manifold) = parse_manifold(input) else {
        return Solution::ParseError;
    };

    // Instead of tracking whether or not a beam exists, we track how many
    // timelines could lead to a beam existing.
    let mut beams = vec![0; manifold.width].into_boxed_slice();
    beams[manifold.start_x] = 1;

    for y in 1..manifold.height {
        for x in 0..manifold.width {
            if manifold.has_splitter(x, y) {
                let split_beam_count = beams[x];
                beams[x - 1] += split_beam_count;
                beams[x] = 0;
                beams[x + 1] += split_beam_count;
            }
        }
    }

    beams.iter().sum::<u64>().into()
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

    /// Returns `true` if the `Manifold` has a splitter at a position.
    fn has_splitter(&self, x: usize, y: usize) -> bool {
        self.cells[x + y * self.width]
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 21.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 40.into());
    }
}
