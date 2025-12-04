//! [Day 4: Printing Department][link]
//!
//! [link]: https://adventofcode.com/2025/day/4

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // We're in the printing department and need to break through the wall to
    // reach the cafeteria and the rest of the North Pole Base. We could use a
    // forklift, but the Elves are too busy using them to move rolls of paper
    // around. They have a diagram (grid of '@' and '.' characters) of paper
    // roll locations, so if we find which rolls are reachable
    // (fewer than 4 neighbouring rolls) they may have time to help us.
    let grid = parse_grid(input);
    let mut reachable_roll_count = 0;

    // We can't use the function from part two because it could make more rolls
    // reachable as it goes.
    for y in 0..grid.height {
        for x in 0..grid.width {
            reachable_roll_count += u16::from(grid.is_roll_reachable(x, y));
        }
    }

    reachable_roll_count.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // If a roll of paper is reachable, then it can be removed. The elves want
    // to know how many rolls can be removed in total.
    let mut grid = parse_grid(input);
    let mut total_removed_roll_count = 0;

    // Removing all the reachable rolls may take multiple passes.
    loop {
        let pass_removed_roll_count = remove_reachable_rolls_from_grid(&mut grid);
        total_removed_roll_count += pass_removed_roll_count;

        if pass_removed_roll_count == 0 {
            break total_removed_roll_count.into();
        }
    }
}

/// Removes some number of reachable paper rolls from a [`Grid`] and returns the
/// number of rolls that were removed. This function returns `0` if there are no
/// reachable rolls.
fn remove_reachable_rolls_from_grid(grid: &mut Grid) -> u16 {
    let mut removed_roll_count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.is_roll_reachable(x, y) {
                grid.remove_roll(x, y);
                removed_roll_count += 1;
            }
        }
    }

    removed_roll_count
}

/// A grid of paper rolls.
struct Grid {
    /// The width of the `Grid` in cells.
    width: usize,

    /// The height of the `Grid` in cells.
    height: usize,

    /// The cells of the `Grid`, which either have a paper roll or do not.
    cells: Vec<bool>,
}

impl Grid {
    /// Creates a new `Grid` from a width and height.
    fn new(width: usize, height: usize) -> Self {
        // Add padding around the grid to simplify counting neighbours.
        Self {
            width,
            height,
            cells: vec![false; (width + 2) * (height + 2)],
        }
    }

    /// Inserts a roll of paper into the `Grid` at a position.
    fn insert_roll(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.cells[index] = true;
    }

    /// Removes a roll of paper from the `Grid` at a position.
    fn remove_roll(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.cells[index] = false;
    }

    /// Returns `true` if a roll exists at a position and is reachable.
    fn is_roll_reachable(&self, x: usize, y: usize) -> bool {
        let index = self.index(x, y);

        if !self.cells[index] {
            return false; // There is no roll here.
        }

        // Move the index to the top-left neighbour.
        let index = index - 1 - (self.width + 2);
        let mut neighbour_count = 0;

        for y in 0..3 {
            for x in 0..3 {
                neighbour_count += u8::from(self.cells[index + x + y * (self.width + 2)]);
            }
        }

        // Allow up to 4 neighbours, the cell we are checking should also have
        // been counted.
        neighbour_count <= 4
    }

    /// Returns the index of a cell from its position.
    fn index(&self, x: usize, y: usize) -> usize {
        x + 1 + (y + 1) * (self.width + 2)
    }
}

/// Parses a [`Grid`] from input.
fn parse_grid(input: &str) -> Grid {
    let mut rows = Vec::new();

    for line in input.lines() {
        let row = line
            .chars()
            .filter(|c| !char::is_ascii_whitespace(c))
            .map(|c| c == '@')
            .collect::<Box<_>>();

        rows.push(row);
    }

    let width = rows[0].len();
    let height = rows.len();
    let mut grid = Grid::new(width, height);

    for (y, row) in rows.iter().enumerate() {
        for (x, cell) in row.iter().copied().enumerate() {
            if cell {
                grid.insert_roll(x, y);
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 13.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 43.into());
    }
}
