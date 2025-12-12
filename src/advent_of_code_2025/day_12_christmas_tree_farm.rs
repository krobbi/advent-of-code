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

    // The actual solution is faster than the test. Presumably the actual input
    // has plenty of space.
    let Some((shapes, regions)) = parse_shapes_and_regions(input) else {
        return Solution::ParseError;
    };

    let shapes = process_shapes(shapes);

    regions
        .iter()
        .map(|r| region_can_be_filled(&shapes, r))
        .map(u16::from)
        .sum::<u16>()
        .into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Part two can't be solved until day 10 part two is solved.
    let _ = input;
    Solution::default()
}

/// Processes a boxed slice of shape [`Grid`]s into [`ShapeData`].
fn process_shapes(shapes: Box<[Grid]>) -> ShapeData {
    let shapes = shapes
        .into_iter()
        .map(grid_variants)
        .map(deduplicate_grids)
        .map(Shape::new)
        .collect();

    ShapeData::new(shapes)
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

/// Returns `true` if a [`Region`] can be filled with its expected shapes.
fn region_can_be_filled(shapes: &ShapeData, region: &Region) -> bool {
    let grid_area = region.width * region.height;
    let shape_area = shape_count_area(shapes, &region.shape_counts);

    if shape_area > grid_area {
        return false;
    }

    let mut grid = Grid::new(region.width, region.height);
    shapes_fit_grid(shapes, &mut grid, &region.shape_counts)
}

/// Returns the total area of a slice of shape counts.
fn shape_count_area(shapes: &ShapeData, shape_counts: &[u8]) -> usize {
    let mut total_area = 0;

    for (index, count) in shape_counts.iter().copied().enumerate() {
        let shape = shapes.shape(index);

        total_area += shape.area() * usize::from(count);
    }

    total_area
}

/// Returns `true` if shape counts can fit into a [`Grid`].
fn shapes_fit_grid(shapes: &ShapeData, grid: &mut Grid, shape_counts: &[u8]) -> bool {
    let Some((shape_index, shape_counts)) = pop_shape_index(shape_counts) else {
        // No more shapes to fit.
        return true;
    };

    let shape = shapes.shape(shape_index);

    for y in -shape.height..grid.height {
        for x in -shape.width..grid.width {
            for variant in &shape.variants {
                if !grid.fits_grid(variant, x, y) {
                    continue;
                }

                grid.stamp_grid(variant, x, y, true);

                if shapes_fit_grid(shapes, grid, &shape_counts) {
                    return true;
                }

                grid.stamp_grid(variant, x, y, false);
            }
        }
    }

    false
}

/// Pops a shape index from a slice of shape counts and returns the shape index
/// and new shape counts. This function returns [`None`] if there are no more
/// shape counts.
fn pop_shape_index(shape_counts: &[u8]) -> Option<(usize, Box<[u8]>)> {
    for (index, shape_count) in shape_counts.iter().copied().enumerate() {
        if shape_count > 0 {
            let mut shape_counts = shape_counts.to_owned().into_boxed_slice();
            shape_counts[index] -= 1;
            return Some((index, shape_counts));
        }
    }

    None
}

/// A database of [`Shape`]s.
struct ShapeData {
    /// The [Shape]s.
    shapes: Box<[Shape]>,
}

impl ShapeData {
    /// Creates new `ShapeData` from a boxed slice of [`Shape`]s.
    fn new(shapes: Box<[Shape]>) -> Self {
        Self { shapes }
    }

    /// Returns a reference to a [`Shape`] from its shape index.
    fn shape(&self, shape_index: usize) -> &Shape {
        &self.shapes[shape_index]
    }
}

/// A processed shape.
struct Shape {
    /// The width of the `Shape` in cells.
    width: i8,

    /// The height of the `Shape` in cells.
    height: i8,

    /// The `Shape`'s variant [`Grid`]s.
    variants: Box<[Grid]>,
}

impl Shape {
    /// Creates a new `Shape` from its variant [`Grid`]s.
    fn new(variants: Box<[Grid]>) -> Self {
        let first_variant = &variants[0];

        Self {
            width: first_variant.width,
            height: first_variant.height,
            variants,
        }
    }

    /// Returns the area of the [`Shape`] in cells.
    fn area(&self) -> usize {
        self.variants[0].cells.iter().filter(|c| **c).count()
    }
}

/// A grid of cells which may be occupied by a present.
struct Grid {
    /// The width of the `Grid` in cells.
    width: i8,

    /// The height of the `Grid` in cells.
    height: i8,

    /// The cells.
    cells: Box<[bool]>,
}

impl Grid {
    /// Creates a new `Grid` from its width and height.
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![false; width * height].into();
        let width = i8::try_from(width).expect("width should be less than `i8::MAX`");
        let height = i8::try_from(height).expect("height should be less than `i8::MAX`");

        Self {
            width,
            height,
            cells,
        }
    }

    /// Returns `true` if the `Grid` could fit another `Grid` on top of it with
    /// X and Y co-ordinate offsets.
    fn fits_grid(&self, other: &Self, offset_x: i8, offset_y: i8) -> bool {
        for y in 0..other.height {
            for x in 0..other.width {
                if !other.is_occupied(x, y) {
                    continue;
                }

                let x = x + offset_x;
                let y = y + offset_y;

                if self.is_occupied(x, y) {
                    return false;
                }
            }
        }

        true
    }

    /// Stamps another `Grid` onto the `Grid`
    fn stamp_grid(&mut self, other: &Self, offset_x: i8, offset_y: i8, value: bool) {
        for y in 0..other.height {
            for x in 0..other.width {
                if !other.is_occupied(x, y) {
                    continue;
                }

                let x = x + offset_x;
                let y = y + offset_y;
                let index = self.index(x, y);
                self.cells[index] = value;
            }
        }
    }

    /// Returns `true` if the cell at an X and Y co-ordinate is occupied.
    fn is_occupied(&self, x: i8, y: i8) -> bool {
        x < 0 || x >= self.width || y < 0 || y >= self.height || self.cells[self.index(x, y)]
    }

    /// Returns an index from an X and Y co-ordinate.
    fn index(&self, x: i8, y: i8) -> usize {
        let x = usize::try_from(x).expect("x should be positive");
        let y = usize::try_from(y).expect("y should be positive");
        let width = usize::try_from(self.width).expect("width should be positive");
        x + y * width
    }

    /// Returns `true` if the `Grid` is a duplicate of another `Grid`.
    fn is_duplicate_of(&self, other: &Self) -> bool {
        // This does not account for translation or rotations of non-square
        // shapes, but this probably doesn't apply to the input.
        self.width == other.width
            && self.height == other.height
            && self.cells.iter().zip(&other.cells).all(|(a, b)| a == b)
    }

    /// Creates a new variant of the `Grid`, flipped horizontally.
    fn flip(&self) -> Self {
        let width = usize::try_from(self.width).expect("width should be positive");
        let height = usize::try_from(self.height).expect("height should be positive");
        let mut flipped_grid = Self::new(width, height);

        for y in 0..height {
            for x in 0..width {
                flipped_grid.cells[x + y * width] = self.cells[width - x - 1 + y * width];
            }
        }

        flipped_grid
    }

    /// Creates a new variant of the `Grid`, rotated 90 degrees clockwise.
    fn rotate(&self) -> Self {
        let width = usize::try_from(self.width).expect("width should be positive");
        let height = usize::try_from(self.height).expect("height should be positive");
        let mut rotated_grid = Self::new(height, width);

        for y in 0..width {
            for x in 0..height {
                let source_x = y;
                let source_y = height - x - 1;
                let source_index = source_x + source_y * width;
                let target_index = x + y * height;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "\
        0:\n\
        ###\n\
        ##.\n\
        ##.\n\
        \n\
        1:\n\
        ###\n\
        ##.\n\
        .##\n\
        \n\
        2:\n\
        .##\n\
        ###\n\
        ##.\n\
        \n\
        3:\n\
        ##.\n\
        ###\n\
        ##.\n\
        \n\
        4:\n\
        ###\n\
        #..\n\
        ###\n\
        \n\
        5:\n\
        ###\n\
        .#.\n\
        ###\n\
        \n\
        4x4: 0 0 0 0 2 0\n\
        12x5: 1 0 1 0 2 2\n\
        12x5: 1 0 1 0 3 2\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 2.into());
    }

    /*
    /// Tests part two.
    #[test]
    fn part_two_works() {}
    */
}
