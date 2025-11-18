//! [Day 2: I Was Told There Would Be No Math][link]
//!
//! [link]: https://adventofcode.com/2015/day/2

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    let mut wrapping_paper_area = 0;

    // Add up the area of wrapping paper needed for each present.
    for line in input.lines() {
        let Some((length, width, height)) = parse_present(line) else {
            return Solution::ParseError;
        };

        // Calculate the areas of each kind of face on a present.
        let top_area = length * width;
        let front_area = width * height;
        let side_area = height * length;

        // Find the smallest face area for extra slack wrapping paper.
        let slack_area = top_area.min(front_area).min(side_area);

        // There needs to be enough wrapping paper to cover two of each kind of
        // face, plus the extra slack area.
        wrapping_paper_area += 2 * top_area + 2 * front_area + 2 * side_area + slack_area;
    }

    wrapping_paper_area.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let mut ribbon_length = 0;

    // Add up the length of ribbon needed for each present.
    for line in input.lines() {
        let Some((length, width, height)) = parse_present(line) else {
            return Solution::ParseError;
        };

        // Calculate the perimeters of each kind of face on a present.
        let top_perimeter = 2 * (length + width);
        let front_perimeter = 2 * (width + height);
        let side_perimeter = 2 * (height + length);

        // The ribbon is wrapped around the present with the shortest length
        // possible.
        let wrap_length = top_perimeter.min(front_perimeter).min(side_perimeter);

        // The length of ribbon needed to tie a bow is equal to the present's
        // volume.
        let bow_length = length * width * height;

        ribbon_length += wrap_length + bow_length;
    }

    ribbon_length.into()
}

/// Parses the length, width, and height of a present from a line of text. This
/// function returns [`None`] if the line of text could not be parsed.
fn parse_present(line: &str) -> Option<(u32, u32, u32)> {
    let mut line = line.split('x');
    let length = line.next()?.parse().ok()?;
    let width = line.next()?.parse().ok()?;
    let height = line.next()?.parse().ok()?;
    Some((length, width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one("2x3x4"), Solution::Solved(58));
        assert_eq!(part_one("1x1x10"), Solution::Solved(43));
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two("2x3x4"), Solution::Solved(34));
        assert_eq!(part_two("1x1x10"), Solution::Solved(14));
    }
}
