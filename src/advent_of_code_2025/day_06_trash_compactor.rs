//! [Day 6: Trash Compactor][link]
//!
//! [link]: https://adventofcode.com/2025/day/6

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // We accidentally jumped into the kitchen's garbage chute while re-enacting
    // a movie scene. Luckily, there is a family of cephalopods who can open the
    // door, but they want to know if we can help with the youngest cephalopod's
    // math homework. The worksheet consists of columns of numbers separated
    // with spaces, with a "+" or "*" operator at the end. We need to apply the
    // operators to the corresponding columns, and find the sum of all the
    // columns.
    let mut lines = input.lines();

    let Some(mut sums) = parse_row(lines.next().unwrap_or("*")) else {
        return Solution::ParseError;
    };

    let mut products = sums.clone();

    for line in lines {
        if matches!(line.chars().next(), Some('+' | '*')) {
            let mut total = 0;

            for (index, operator) in line.split_whitespace().enumerate() {
                match operator {
                    "+" => total += sums[index],
                    "*" => total += products[index],
                    _ => return Solution::ParseError,
                }
            }

            return total.into();
        }

        let Some(row) = parse_row(line) else {
            return Solution::ParseError;
        };

        for (index, number) in row.iter().copied().enumerate() {
            sums[index] += number;
            products[index] *= number;
        }
    }

    Solution::ParseError
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // We got the wrong answer. Cephalopod numbers are written in vertically
    // with the most significant digit at the top. The same columns need to be
    // solved.

    // Now the input will be treated as a grid of ascii characters for
    // convenience.
    let Some(grid) = parse_grid(input) else {
        return Solution::ParseError;
    };

    let mut operator = Operator::default();
    let mut accumulator = 0;
    let mut total = 0;

    for x in 0..grid.width {
        operator = match grid.get_char(x, grid.height - 1) {
            b'*' => {
                accumulator = 1;
                Operator::Multiply
            }
            b'+' => {
                accumulator = 0;
                Operator::Add
            }
            _ => operator,
        };

        let mut number = 0;

        for y in 0..grid.height - 1 {
            let c = grid.get_char(x, y);

            if c.is_ascii_digit() {
                number = number * 10 + u64::from(grid.get_char(x, y) - b'0');
            }
        }

        // Empty column.
        if number == 0 {
            total += accumulator;
            continue;
        }

        match operator {
            Operator::Multiply => accumulator *= number,
            Operator::Add => accumulator += number,
        }

        // End of worksheet.
        if x == grid.width - 1 {
            total += accumulator;
        }
    }

    total.into()
}

/// An operator used in cephalopod homework.
#[derive(Default)]
#[repr(u8)]
enum Operator {
    #[default]
    Multiply = b'*',

    Add = b'+',
}

/// A grid of worksheet characters for part two.
struct Grid {
    /// The width of the grid in characters.
    width: usize,

    /// The height of the grid in characters.
    height: usize,

    /// The characters.
    chars: Vec<u8>,
}

impl Grid {
    /// Creates a new `Grid` with a width and height.
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chars: vec![b' '; width * height],
        }
    }

    /// Sets a character in the `Grid`.
    fn set_char(&mut self, x: usize, y: usize, c: u8) {
        self.chars[x + y * self.width] = c;
    }

    /// Returns a character from the `Grid`.
    fn get_char(&self, x: usize, y: usize) -> u8 {
        self.chars[x + y * self.width]
    }
}

/// Parses a row of numbers from a line of input for part one. This function
/// returns [`None`] if a row could not be parsed.
fn parse_row(line: &str) -> Option<Vec<u64>> {
    let mut row = Vec::new();

    for number in line.split_whitespace() {
        let number = number.parse().ok()?;
        row.push(number);
    }

    Some(row)
}

/// Parses a [`Grid`] from input for part two. This function returns [`None`] if
/// a [`Grid`] could not be parsed.
fn parse_grid(input: &str) -> Option<Grid> {
    let mut lines = input.lines();
    let mut rows = vec![lines.next()?.as_bytes().to_owned()];
    let width = rows[0].len();

    for line in lines {
        let row = line.as_bytes().to_owned();

        if row.len() != width {
            return None;
        }

        rows.push(row);
    }

    let height = rows.len();
    let mut grid = Grid::new(width, height);

    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.iter().copied().enumerate() {
            grid.set_char(x, y, c);
        }
    }

    Some(grid)
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
