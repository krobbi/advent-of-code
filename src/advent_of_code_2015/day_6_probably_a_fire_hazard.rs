//! [Day 6: Probably a Fire Hazard][link]
//!
//! [link]: https://adventofcode.com/2015/day/6

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Keep track of the grid of lights.
    let mut grid = Grid::new();

    for instruction in input.lines() {
        let Some((action, rect)) = parse_instruction(instruction) else {
            return Solution::ParseError;
        };

        grid.apply_action(action, rect);
    }

    grid.count().into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A grid of lights.
struct Grid {
    /// The lights.
    lights: Box<[bool]>,
}

impl Grid {
    /// Creates a new `Grid`.
    fn new() -> Self {
        Self { lights: vec![false; 1_000_000].into_boxed_slice() }
    }

    /// Applies an [`Action`] to the `Grid` with a [`Rect`].
    fn apply_action(&mut self, action: Action, rect: Rect) {
        let left = usize::from(rect.left);
        let right = usize::from(rect.right);

        for y in rect.top..=rect.bottom {
            let index = usize::from(y) * 1000;
            let slice = &mut self.lights[(index + left)..=(index + right)];

            match action {
                Action::TurnOn => slice.fill(true),
                Action::TurnOff => slice.fill(false),
                Action::Toggle => {
                    for light in slice.iter_mut() {
                        *light = !*light;
                    }
                }
            }
        }
    }

    /// Returns the number of lights that are turned on.
    fn count(&self) -> u32 {
        let mut count = 0;

        for light in &self.lights {
            count += u32::from(*light);
        }

        count
    }
}

/// An action that can be applied with a [`Rect`].
#[derive(Clone, Copy)]
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
    Some((action, Rect { left, right, top, bottom }))
}

/// Parses a light position from a word. This function returns [`None`] if a
/// position could not be parsed.
fn parse_position(word: &str) -> Option<(u16, u16)> {
    let mut numbers = word.split(',');
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
