//! [Day 6: Probably a Fire Hazard][link]
//!
//! [link]: https://adventofcode.com/2015/day/6

use crate::Solution;

/// A function for applying an [`Action`] to a slice of lights.
type Adjuster = fn(action: Action, lights: &mut [u16]);

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    solve_part(input, adjust_lights_part_one)
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    solve_part(input, adjust_lights_part_two)
}

/// Solves a part with an [`Adjuster`].
fn solve_part(input: &str, adjuster: Adjuster) -> Solution {
    let mut grid = Grid::new();

    for instruction in input.lines() {
        let Some((action, rect)) = parse_instruction(instruction) else {
            return Solution::ParseError;
        };

        grid.apply_action(action, rect, adjuster);
    }

    grid.brightness().into()
}

/// Applies an [`Action`] to a slice of lights for part one.
fn adjust_lights_part_one(action: Action, lights: &mut [u16]) {
    match action {
        Action::TurnOn => lights.fill(1),
        Action::TurnOff => lights.fill(0),
        Action::Toggle => {
            for light in lights {
                *light = u16::from(*light == 0);
            }
        }
    }
}

/// Applies an [`Action`] to a slice of lights for part two.
fn adjust_lights_part_two(action: Action, lights: &mut [u16]) {
    match action {
        Action::TurnOn => {
            for light in lights {
                *light += 1;
            }
        }
        Action::TurnOff => {
            for light in lights {
                *light = light.saturating_sub(1);
            }
        }
        Action::Toggle => {
            for light in lights {
                *light += 2;
            }
        }
    }
}

/// A grid of lights.
struct Grid {
    /// The brightness of each light.
    lights: Box<[u16]>,
}

impl Grid {
    /// Creates a new `Grid`.
    fn new() -> Self {
        Self { lights: vec![0; 1_000_000].into_boxed_slice() }
    }

    /// Applies an [`Action`] to a [`Rect`] of the grid with an [`Adjuster`].
    fn apply_action(&mut self, action: Action, rect: Rect, adjuster: Adjuster) {
        let left = usize::from(rect.left);
        let right = usize::from(rect.right);

        for y in rect.top..=rect.bottom {
            let index = usize::from(y) * 1000;
            let slice = &mut self.lights[(index + left)..=(index + right)];
            adjuster(action, slice);
        }
    }

    /// Returns the total brightness of the grid.
    fn brightness(&self) -> u32 {
        let mut brightness = 0;

        for light in &self.lights {
            brightness += u32::from(*light);
        }

        brightness
    }
}

/// An action that can be applied to lights.
#[derive(Clone, Copy)]
enum Action {
    /// Turn on the lights.
    TurnOn,

    /// Turn off the lights.
    TurnOff,

    /// Toggle the lights.
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
