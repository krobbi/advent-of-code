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
    let Some((shapes, _regions)) = parse_shapes_and_regions(input) else {
        return Solution::ParseError;
    };

    let shapes = process_shapes(shapes);

    for (index, shape) in shapes.iter().enumerate() {
        println!("\nShape {index}:");

        for variant in shape {
            for y in 0..variant.height {
                print!(" ");

                for x in 0..variant.width {
                    if variant.cells[x + y * variant.width] {
                        print!("[]");
                    } else {
                        print!("  ");
                    }
                }

                println!();
            }

            println!("---");
        }
    }

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Processes a boxed slice of shape [`Grid`]s into a boxed slice of boxed
/// slices of shape variant [`Grid`]s.
fn process_shapes(shapes: Box<[Grid]>) -> Box<[Box<[Grid]>]> {
    shapes
        .into_iter()
        .map(grid_variants)
        .map(deduplicate_grids)
        .collect()
}

/// Consumes a [`Grid`] and returns a vector of its rotational and chiral
/// variant [`Grid`]s.
fn grid_variants(grid: Grid) -> Vec<Grid> {
    let mut flipped_rotations = grid_rotations(grid.flip());
    let mut rotations = grid_rotations(grid);
    rotations.append(&mut flipped_rotations);
    rotations
}

/// Consumes a [`Grid`] and returns a vector of its rotational variant
/// [`Grid`]s.
fn grid_rotations(grid: Grid) -> Vec<Grid> {
    let r1 = grid.rotate();
    let r2 = r1.rotate();
    let r3 = r2.rotate();
    vec![grid, r1, r2, r3]
}

/// Consumes a vector of [`Grid`]s and returns a boxed slice of the [`Grid`]s
/// with any duplicates removed.
fn deduplicate_grids(mut grids: Vec<Grid>) -> Box<[Grid]> {
    let mut deduplicated_grids = Vec::new();

    while let Some(grid) = grids.pop() {
        if !deduplicated_grids
            .iter()
            .any(|g: &Grid| g.is_duplicate_of(&grid))
        {
            deduplicated_grids.push(grid);
        }
    }

    deduplicated_grids.into()
}

/// A grid of cells which may be occupied by a present.
struct Grid {
    /// The width of the `Grid` in cells.
    width: usize,

    /// The height of the `Grid` in cells.
    height: usize,

    /// The cells.
    cells: Box<[bool]>,
}

impl Grid {
    /// Creates a new `Grid` from its width and height.
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height].into(),
        }
    }

    /// Returns `true` if the `Grid` is a duplicate of another `Grid`.
    fn is_duplicate_of(&self, other: &Self) -> bool {
        // This does not account for translation or rotations of non-square
        // shapes, but this probably doesn't apply to the input.
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for index in 0..self.width * self.height {
            if self.cells[index] != other.cells[index] {
                return false;
            }
        }

        true
    }

    /// Creates a new variant of the `Grid`, flipped horizontally.
    fn flip(&self) -> Self {
        let mut flipped_grid = Self::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                flipped_grid.cells[x + y * self.width] =
                    self.cells[self.width - x - 1 + y * self.width];
            }
        }

        flipped_grid
    }

    /// Creates a new variant of the `Grid`, rotated 90 degrees clockwise.
    fn rotate(&self) -> Self {
        let mut rotated_grid = Self::new(self.height, self.width);

        for y in 0..rotated_grid.height {
            for x in 0..rotated_grid.width {
                let source_x = y;
                let source_y = rotated_grid.width - x - 1;
                let source_index = source_x + source_y * self.width;
                let target_index = x + y * rotated_grid.width;
                rotated_grid.cells[target_index] = self.cells[source_index];
            }
        }

        rotated_grid
    }
}

/// A region where presents must be arranged.
struct Region {
    /// The width of the `Region` in cells.
    width: usize,

    /// The height of the `Region` in cells.
    height: usize,

    /// The amount of each shape expected in the `Region`.
    shape_counts: Box<[u8]>,
}

/// Parses shape [`Grid`]s and [`Region`]s from input. This function returns
/// [`None`] if shape [`Grid`]s and [`Region`]s could not be parsed.
fn parse_shapes_and_regions(input: &str) -> Option<(Box<[Grid]>, Box<[Region]>)> {
    let mut lines = input.lines().map(str::trim).peekable();
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    while let Some(line) = lines.next() {
        if line.ends_with(':') {
            let index = line.trim_end_matches(':').parse::<usize>().ok()?;

            if index != shapes.len() {
                return None;
            }

            let shape = parse_shape(&mut lines)?;
            shapes.push(shape);
        } else if !line.is_empty() {
            let region = parse_region(line)?;

            if region.shape_counts.len() != shapes.len() {
                return None;
            }

            regions.push(region);
        }
    }

    Some((shapes.into(), regions.into()))
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

/// Parses a region from a line of input. This function returns [`None`] if a
/// [`Region`] could not be parsed.
fn parse_region(line: &str) -> Option<Region> {
    let mut words = line.split(' ');
    let mut dimensions = words.next()?.trim_end_matches(':').split('x');
    let width = dimensions.next()?.parse().ok()?;
    let height = dimensions.next()?.parse().ok()?;
    let mut shape_counts = Vec::new();

    for count in words {
        let count = count.parse().ok()?;
        shape_counts.push(count);
    }

    Some(Region {
        width,
        height,
        shape_counts: shape_counts.into(),
    })
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
