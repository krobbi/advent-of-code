//! [Day 12: Christmas Tree Farm][link]
//!
//! [link]: https://adventofcode.com/2025/day/12

use std::iter::Peekable;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are busy decorating the Christmas tree farm, but they are
    // worried that the presents won't fit under all the trees. There is a set
    // of standard shapes for presents, which are made up of connected unit
    // squares (polyominoes). Under each tree is a rectangular region for
    // arranging presents. Each region expects zero or more of each shape to fit
    // in it. Presents may be rotated or flipped to fit better, but cannot be
    // stacked on top of each other. The Elves want to know how many of the
    // regions can actually fit the expected presents.

    // I can't think of a good way to solve this other than trying every
    // possibility until one fits. Flipping or rotating a shape may produce
    // duplicates, which can be removed to possibly speed up the solution.
    let Some(shapes) = parse_shapes_and_regions(input) else {
        return Solution::ParseError;
    };

    for (index, shape) in shapes.iter().enumerate() {
        println!("\nShape {index}:");

        for y in 0..shape.height {
            print!(" ");

            for x in 0..shape.width {
                if shape.cells[x + y * shape.width] {
                    print!("[]");
                } else {
                    print!("  ");
                }
            }

            println!();
        }
    }

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A grid of cells which may be occupied by a present.
struct Grid {
    /// The width of the `Grid` in cells.
    width: usize,

    /// The height of the `Grid` in cells.
    height: usize,

    /// The cells.
    cells: Vec<bool>,
}

impl Grid {
    /// Creates a new `Grid` from its width and height.
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height],
        }
    }
}

/// Parses shapes and regions from input. This function returns [`None`] if
/// shapes and regions could not be parsed.
fn parse_shapes_and_regions(input: &str) -> Option<Box<[Grid]>> {
    let mut lines = input.lines().map(str::trim).peekable();
    let mut shapes = Vec::new();

    while let Some(line) = lines.next() {
        if line.ends_with(':') {
            let index = line.trim_end_matches(':').parse::<usize>().ok()?;

            if index != shapes.len() {
                return None;
            }

            let shape = parse_shape(&mut lines)?;
            shapes.push(shape);
        }
    }

    Some(shapes.into())
}

/// Parses a shape [`Grid`] from a peekable line iterator after consuming its
/// index line. This function returns [`None`] if a shape [`Grid`] could not be
/// parsed.
fn parse_shape<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Option<Grid> {
    let mut rows = vec![lines.next()?.to_owned()];
    let width = rows[0].len();

    if width < 1 {
        return None;
    }

    while let Some(row) = lines.next_if(|l| matches!(l.chars().next(), Some('#' | '.'))) {
        if row.len() != width {
            return None;
        }

        rows.push(row.into());
    }

    let mut grid = Grid::new(width, rows.len());

    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid.cells[x + y * width] = c == '#';
        }
    }

    Some(grid)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "";

    /// Tests part one.
    #[test]
    fn part_one_works() {}

    /// Tests part two.
    #[test]
    fn part_two_works() {}
}
*/
