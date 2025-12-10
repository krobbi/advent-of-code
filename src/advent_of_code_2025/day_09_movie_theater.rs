//! [Day 9: Movie Theater][link]
//!
//! [link]: https://adventofcode.com/2025/day/9

use std::cmp::Reverse;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Elves are redecorating the movie theater by switching out some of the
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
    let Some(points) = parse_points(input) else {
        return Solution::ParseError;
    };

    largest_rects(&points)
        .first()
        .map_or(Solution::SolveError, |r| r.area().into())
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // The list of red tiles is actually a loop with 90-degree corners. The
    // inside of the loop (to the right of each line segment) is filled with
    // green tiles. The line segments between the vertices are also filled with
    // green tiles. The largest rectangle of tiles must now also fit in the area
    // of red and green tiles.

    // The area of each rectangle can be checked as before, but when a new
    // largest rectangle is found it must be tested. The loop may be concave so
    // this is difficult to test. My first attempt searched for concave corners,
    // and made them into rectangles, but this did not solve every possibility.
    let _ = input;
    Solution::default()
}

/// Returns a boxed slice of every [`Rect`] formed between two different
/// [`Point`]s in a slice of [`Point`]s. The [`Rect`]s are sorted from the
/// largest to the smallest area.
fn largest_rects(points: &[Point]) -> Box<[Rect]> {
    let mut rects = Vec::new();

    for index_a in 0..points.len() - 1 {
        let a = points[index_a];

        for b in points.iter().copied().skip(index_a + 1) {
            rects.push(Rect::new(a, b));
        }
    }

    rects.sort_unstable_by_key(|r| Reverse(r.area()));
    rects.into()
}

/// A tile position.
#[derive(Clone, Copy)]
struct Point {
    /// The `Point`'s X co-ordinate.
    x: u32,

    /// The `Point`'s Y co-ordinate.
    y: u32,
}

impl Point {
    /// Creates a new `Point` from X and Y co-ordinates.
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

/// A rectangle.
#[derive(Clone, Copy)]
struct Rect {
    /// The X co-ordinate of the `Rect`'s left edge, inclusive.
    left: u32,

    /// The X co-ordinate of the `Rect`'s right edge, inclusive.
    right: u32,

    /// The Y co-ordinate of the `Rect`'s top edge, inclusive.
    top: u32,

    /// The Y co-ordinate of the `Rect`'s bottom edge, inclusive.
    bottom: u32,
}

impl Rect {
    /// Creates a new `Rect` from two corner [`Point`]s.
    fn new(a: Point, b: Point) -> Self {
        let left = a.x.min(b.x);
        let right = a.x.max(b.x);
        let top = a.y.min(b.y);
        let bottom = a.y.max(b.y);

        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Returns the `Rect`'s area in tiles.
    fn area(self) -> u64 {
        let width = u64::from(self.right - self.left + 1);
        let height = u64::from(self.bottom - self.top + 1);
        width * height
    }
}

/// Parses a boxed slice of [`Point`]s from input. This function returns
/// [`None`] if the [`Point`]s could be parsed.
fn parse_points(input: &str) -> Option<Box<[Point]>> {
    let mut points = Vec::new();

    for line in input.lines().map(str::trim) {
        let point = parse_point(line)?;
        points.push(point);
    }

    Some(points.into())
}

/// Parses a [`Point`] from a line of input. This function returns [`None`] if a
/// [`Point`] could not be parsed.
fn parse_point(line: &str) -> Option<Point> {
    let mut numbers = line.split(',');
    let x = numbers.next()?.parse().ok()?;
    let y = numbers.next()?.parse().ok()?;
    Some(Point::new(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "\
        7,1\n\
        11,1\n\
        11,7\n\
        9,7\n\
        9,5\n\
        2,5\n\
        2,3\n\
        7,3\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 50.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 24.into());
    }
}
