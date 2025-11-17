//! [Day 6: Probably a Fire Hazard][link]
//!
//! [link]: https://adventofcode.com/2015/day/6

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Keep track of non-overlapping rectangles of lights that are turned on.
    let mut grid = Vec::new();

    for instruction in input.lines() {
        let Some((action, rect)) = parse_instruction(instruction) else {
            return Solution::ParseError;
        };

        apply_action(action, rect, &mut grid);
    }

    let mut light_count = 0;

    for rect in grid {
        light_count += rect.area();
    }

    light_count.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// An action that can be applied with a [`Rect`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Action {
    /// Turn on the lights in the [`Rect`].
    TurnOn,

    /// Turn off the lights in the [`Rect`].
    TurnOff,

    /// Toggle the lights in the [`Rect`].
    Toggle,
}

/// A rectangular area of lights.
#[derive(Clone, Copy)]
struct Rect {
    /// The position of the leftmost light.
    left: u16,

    /// The position of the rightmost light.
    right: u16,

    /// The position of the topmost light.
    top: u16,

    /// The position of the bottommost light.
    bottom: u16,
}

impl Rect {
    /// Returns the area of the `Rect` in lights.
    fn area(self) -> u32 {
        let width: u32 = (self.right + 1 - self.left).into();
        let height: u32 = (self.bottom + 1 - self.top).into();
        width * height
    }

    /// Splits the `Rect` into pieces so it doesn't overlap with another `Rect`.
    /// This function returns [`None`] if there was no overlap.
    fn remove_overlap(self, other: Self) -> Option<Box<[Self]>> {        
        // Do nothing if there is no overlap (AABB algorithm).
        if other.left > self.right || other.right < self.left || other.top > self.bottom || other.bottom < self.top {
            return None;
        }

        let mut pieces = Vec::with_capacity(4);

        // Find the overlapping area.
        let overlap = Rect {
            left: self.left.max(other.left),
            right: self.right.min(other.right),
            top: self.top.max(other.top),
            bottom: self.bottom.min(other.bottom),
        };

        // Add any space to the left of the overlap.
        if overlap.left > self.left {
            pieces.push(Rect {
                left: self.left,
                right: overlap.left - 1,
                top: self.top,
                bottom: self.bottom,
            });
        }

        // Add any space to the right of the overlap.
        if overlap.right < self.right {
            pieces.push(Rect {
                left: overlap.right + 1,
                right: self.right,
                top: self.top,
                bottom: self.bottom,
            });
        }

        // Add any space above the overlap.
        if overlap.top > self.top {
            pieces.push(Rect {
                left: overlap.left,
                right: overlap.right,
                top: self.top,
                bottom: overlap.top - 1,
            });
        }

        // Add any space below the overlap.
        if overlap.bottom < self.bottom {
            pieces.push(Rect {
                left: overlap.left,
                right: overlap.right,
                top: overlap.bottom + 1,
                bottom: self.bottom,
            });
        }

        Some(pieces.into_boxed_slice())
    }
}

/// Parses an [`Action`] and a [`Rect`] from an instruction. This function
/// returns [`None`] if an instruction could not be parsed.
fn parse_instruction(instruction: &str) -> Option<(Action, Rect)> {
    let mut words = instruction.split(' ');

    let action = match words.next()? {
        "turn" => match words.next()? {
            "on" => Action::TurnOn,
            "off" => Action::TurnOff,
            _ => return None,
        }
        "toggle" => Action::Toggle,
        _ => return None,
    };

    let (left, top) = parse_position(words.next()?)?;
    words.next(); // Skip "through".
    let (right, bottom) = parse_position(words.next()?)?;

    (left <= right && top <= bottom).then_some((action, Rect { left, right, top, bottom }))
}

/// Parses a light position from a word. This function returns [`None`] if a
/// position could not be parsed.
fn parse_position(word: &str) -> Option<(u16, u16)> {
    let mut numbers = word.split(',');
    let x = numbers.next()?.parse().ok()?;
    let y = numbers.next()?.parse().ok()?;
    Some((x, y))
}

/// Applies an [`Action`] with a [`Rect`] to the grid.
fn apply_action(action: Action, rect: Rect, grid: &mut Vec<Rect>) {
    let mut new_grid = Vec::new();
    let mut overlaps = Vec::new();

    // Any overlaps with the new rectangle need to be removed from the grid.
    while let Some(grid_rect) = grid.pop() {
        if let Some(pieces) = grid_rect.remove_overlap(rect) {
            overlaps.push(grid_rect);
            new_grid.extend_from_slice(&pieces);
        } else {
            new_grid.push(grid_rect);
        }
    }

    if action == Action::TurnOn {
        grid.push(rect);
    } else if action == Action::Toggle {
        let mut rects = vec![rect];
        let mut new_rects = Vec::new();

        for overlap in overlaps {
            while let Some(rect) = rects.pop() {
                if let Some(pieces) = rect.remove_overlap(overlap) {
                    new_rects.extend_from_slice(&pieces);
                } else {
                    new_rects.push(rect);
                }
            }

            rects.append(&mut new_rects);
        }
    }

    grid.append(&mut new_grid);
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
