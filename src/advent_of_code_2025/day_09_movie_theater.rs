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
    // For my solution, I represent the border of the shape as a set of line
    // segments which must not intersect with the solution rectangle.
    let Some(points) = parse_points(input) else {
        return Solution::ParseError;
    };

    if points.len() < 4 {
        return Solution::SolveError;
    }

    let shape = Shape::new(&points);

    for rect in largest_rects(&points) {
        if shape.fits_rect(rect) {
            return rect.area().into();
        }
    }

    Solution::SolveError
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

/// A shape with horitontal and vertical [`Seg`]s.
#[derive(Default)]
struct Shape {
    /// The horizontal terminating [`Seg`]s.
    h_segs: Vec<Seg>,

    /// The vertical terminating [`Seg`]s.
    v_segs: Vec<Seg>,
}

impl Shape {
    /// Creates a new `Shape` from a loop of [`Point`]s wound clockwise.
    fn new(points: &[Point]) -> Self {
        let mut shape = Self::default();

        for (prev, a, b, next) in points.windows(4).map(|w| (w[0], w[1], w[2], w[3])) {
            shape.insert_seg(prev, a, b, next);
        }

        let len = points.len();
        shape.insert_seg(points[len - 3], points[len - 2], points[len - 1], points[0]);
        shape.insert_seg(points[len - 2], points[len - 1], points[0], points[1]);
        shape.insert_seg(points[len - 1], points[0], points[1], points[2]);
        shape
    }

    /// Returns `true` if a [`Rect`] fits into the `Shape`, assuming the
    /// top-left [`Point`] is on a vertex.
    fn fits_rect(&self, rect: Rect) -> bool {
        for seg in self.h_segs.iter().copied() {
            if seg.p >= rect.top
                && seg.p <= rect.bottom
                && seg.n_max >= rect.left
                && seg.n_min <= rect.right
            {
                return false;
            }
        }

        for seg in self.v_segs.iter().copied() {
            if seg.p >= rect.left
                && seg.p <= rect.right
                && seg.n_max >= rect.top
                && seg.n_min <= rect.bottom
            {
                return false;
            }
        }

        true
    }

    /// Inserts a new [`Seg`] into the `Shape` from its previous, start, end,
    /// and next [`Point`]s.
    fn insert_seg(&mut self, prev: Point, a: Point, b: Point, next: Point) {
        // This is horrible code, but it has been tested and ensures that there
        // are no gaps or spikes in the edges.
        if a.x == b.x {
            let is_right = a.y < b.y;

            let (p, above, top, bottom, below) = if is_right {
                (a.x + 1, prev, a, b, next)
            } else {
                (a.x - 1, next, b, a, prev)
            };

            let is_top_concave = (above.x > top.x) == is_right;
            let is_bottom_concave = (below.x > bottom.x) == is_right;
            let n_min = top.y + u32::from(is_top_concave);
            let n_max = bottom.y - u32::from(is_bottom_concave);
            self.v_segs.push(Seg { p, n_min, n_max });
        } else if a.y == b.y {
            let is_bottom = a.x > b.x;

            let (p, before, left, right, after) = if is_bottom {
                (a.y + 1, next, b, a, prev)
            } else {
                (a.y - 1, prev, a, b, next)
            };

            let is_left_concave = (before.y > left.y) == is_bottom;
            let is_right_concave = (after.y > right.y) == is_bottom;
            let n_min = left.x + u32::from(is_left_concave);
            let n_max = right.x - u32::from(is_right_concave);
            self.h_segs.push(Seg { p, n_min, n_max });
        }
    }
}

/// An axis-aligned line segment.
#[derive(Clone, Copy)]
struct Seg {
    /// The `Seg`'s plane co-ordinate.
    p: u32,

    /// The `Seg`'s minimum normal co-ordinate.
    n_min: u32,

    /// The `Seg`'s maximum normal co-ordinate.
    n_max: u32,
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
    Some(Point { x, y })
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
