//! [Day 9: Movie Theater][link]
//!
//! [link]: https://adventofcode.com/2025/day/9

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
    // The list of red tiles is actually a loop with 90-degree corners. The
    // inside of the loop (to the right of each line segment) is filled with
    // green tiles. The line segments between the vertices are also filled with
    // green tiles. The largest rectangle of tiles must now also fit in the area
    // of red and green tiles.

    // The area of each rectangle can be checked as before, but when a new
    // largest rectangle is found it must be tested. The loop may be concave so
    // this is difficult to test. My first attempt searched for concave corners,
    // and made them into rectangles, but this did not solve every possibility.
    let Some(vertices) = parse_tiles(input) else {
        return Solution::ParseError;
    };

    if vertices.is_empty() {
        return Solution::SolveError;
    } else if vertices.len() == 1 {
        return 1.into();
    } else if vertices.len() > 10 {
        // TODO: Optimise this solution and solve 2025 day 9 part two.
        return Solution::TooSlow;
    }

    // This is my second attempt. When a line segment facing the opposite
    // direction is crossed, we toggle between inside the shape and outside the
    // shape. Using this it is possible to detect whether any point is inside or
    // outside of the shape.
    let shape = Shape::new(&vertices);

    // Start by finding all rectangles from largest to smallest.
    let rects = find_largest_rects(&vertices);

    // Use the first one that fits.
    for (a, b) in rects {
        if shape.contains_rect(a, b) {
            return find_area(a, b).into();
        }
    }

    Solution::SolveError
}

/// A shape made of horizontal and vertical line [`Segment`]s.
#[derive(Default)]
struct Shape {
    /// The line [`Segment`]s which enter the `Shape`.
    opening_segments: Vec<Segment>,

    /// The line [`Segment`]s which exit the `Shape`.
    closing_segments: Vec<Segment>,
}

impl Shape {
    /// Creates a new `Shape` from a loop of vertices.
    fn new(vertices: &[(u32, u32)]) -> Self {
        let mut shape = Self::default();

        for (a, b) in vertices.windows(2).map(|w| (w[0], w[1])) {
            shape.insert_segment(a, b);
        }

        shape.insert_segment(vertices[vertices.len() - 1], vertices[0]);
        shape
    }

    /// Returns `true` if the shape contains a point.
    fn contains_point(&self, x: u32, y: u32) -> bool {
        let mut is_inside_shape = false;

        for x in 0..=x {
            if is_inside_shape {
                if self.closing_segments.iter().any(|s| s.contains_point(x, y)) {
                    is_inside_shape = false;
                }
            } else if self.opening_segments.iter().any(|s| s.contains_point(x, y)) {
                is_inside_shape = true;
            }
        }

        is_inside_shape
    }

    /// Returns `true` if the shape contains a rectangle.
    fn contains_rect(&self, a: (u32, u32), b: (u32, u32)) -> bool {
        let left = a.0.min(b.0);
        let right = a.0.max(b.0);
        let top = a.1.min(b.1);
        let bottom = a.1.max(b.1);

        for y in top..=bottom {
            // Check that the left edge of the rectangle is inside the shape.
            if !self.contains_point(left, y) {
                return false;
            }

            // After that, no row should intersect with a closing segment.
            for x in left..=right {
                if self.closing_segments.iter().any(|s| s.contains_point(x, y)) {
                    return false;
                }
            }
        }

        true
    }

    /// Inserts a new [`Segment`] into the `Shape` from its vertices.
    fn insert_segment(&mut self, a: (u32, u32), b: (u32, u32)) {
        if a.0 != b.0 {
            // Horizontal line segments are ignored.
            return;
        }

        // Upward line segments are opening, downward line segments are closing.
        let (is_opening, x, y_min, y_max) = if a.1 > b.1 {
            (true, a.0, b.1, a.1)
        } else {
            (false, a.0 + 1, a.1, b.1)
        };

        let segment = Segment { x, y_min, y_max };

        if is_opening {
            self.opening_segments.push(segment);
        } else {
            self.closing_segments.push(segment);
        }
    }
}

/// A vertical line segment.
struct Segment {
    /// The X co-ordinate of the `Segment`.
    x: u32,

    /// The minimum Y co-ordinate of the `Segment`.
    y_min: u32,

    /// The maximum Y co-ordinate of the `Segment`.
    y_max: u32,
}

impl Segment {
    /// Returns `true` if the `Segment` contains a point on its orthogonal and
    /// plane axis.
    fn contains_point(&self, x: u32, y: u32) -> bool {
        x == self.x && y >= self.y_min && y <= self.y_max
    }
}

/// Returns the largest rectangles in a slice of tile co-ordinates, sorted from
/// largest to smallest.
fn find_largest_rects(tiles: &[(u32, u32)]) -> Vec<((u32, u32), (u32, u32))> {
    let mut rects = Vec::new();

    for index_a in 0..tiles.len() - 1 {
        for index_b in index_a + 1..tiles.len() {
            rects.push((tiles[index_a], tiles[index_b]));
        }
    }

    rects.sort_unstable_by(|a, b| find_area(b.0, b.1).cmp(&find_area(a.0, a.1)));
    rects
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
