//! [Day 9: Movie Theater][link]
//!
//! [link]: https://adventofcode.com/2025/day/9

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are decorating the movie theater by switching out some of the
    // floor (2D co-ordinates) tiles with red tiles. Some of the floor tiles are
    // already red, and they want to find the area of the largest rectangle
    // whose opposite corners are red. In other words, they want to find the
    // distance squared between the furthest tiles.

    // I was considering searching for one corner on an edge of the bounding
    // rectangle of all of the tiles, but this example shows that it may not
    // always be possible:
    // .....#.....
    // .A.........
    // ...........
    // ...........
    // ...........
    // #.........#
    // ...........
    // ...........
    // ...........
    // .........B.
    // .....#.....
    // The largest area is between 'A' and 'B' with 81 tiles, but 'A' and 'B'
    // are not on any of the bounding edges. Maybe the solution would involve a
    // point furthest from the centre of all the points, but I have decided to
    // check every pair.
    let Some(tiles) = parse_tiles(input) else {
        return Solution::ParseError;
    };

    if tiles.is_empty() {
        return Solution::SolveError;
    } else if tiles.len() == 1 {
        return 1.into();
    }

    find_largest_area(&tiles).into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Returns the largest area between two tiles co-ordinates from a slice of tile
/// co-ordinates.
fn find_largest_area(tiles: &[(u32, u32)]) -> u64 {
    let mut largest_area = 1;

    for index_a in 0..tiles.len() - 1 {
        for index_b in index_a + 1..tiles.len() {
            largest_area = largest_area.max(find_area(tiles[index_a], tiles[index_b]));
        }
    }

    largest_area
}

/// Returns the area between two corner tile co-ordinates.
fn find_area(a: (u32, u32), b: (u32, u32)) -> u64 {
    let width = u64::from(a.0.abs_diff(b.0) + 1);
    let height = u64::from(a.1.abs_diff(b.1) + 1);
    width * height
}

/// Parses a boxed slice of tile co-ordinates from input. This function returns
/// [`None`] if tile co-ordinates could not be parsed.
fn parse_tiles(input: &str) -> Option<Box<[(u32, u32)]>> {
    let mut tiles = Vec::new();

    for line in input.lines().map(str::trim) {
        let tile = parse_tile(line)?;
        tiles.push(tile);
    }

    Some(tiles.into())
}

/// Parses a tile co-ordinate from a line of input. This function returns
/// [`None`] if a tile co-ordinate could not be parsed.
fn parse_tile(line: &str) -> Option<(u32, u32)> {
    let mut numbers = line.split(',');
    let x = numbers.next()?.parse().ok()?;
    let y = numbers.next()?.parse().ok()?;
    Some((x, y))
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
