//! [Day 6: Probably a Fire Hazard][link]
//!
//! [link]: https://adventofcode.com/2015/day/6

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    for instruction in input.lines() {
        let Some((action, rect)) = parse_instruction(instruction) else {
            return Solution::ParseError;
        };

        println!("{action:?} {rect:?}");
    }

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// An action that can be applied with a [`Rect`].
#[derive(Clone, Copy, Debug)]
enum Action {
    /// Turn on the lights in the [`Rect`].
    TurnOn,

    /// Turn off the lights in the [`Rect`].
    TurnOff,

    /// Toggle the lights in the [`Rect`].
    Toggle,
}

/// An rectangular area of lights.
#[derive(Clone, Copy, Debug)]
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
