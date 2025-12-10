//! [Day 10: Factory][link]
//!
//! [link]: https://adventofcode.com/2025/day/10

use std::{iter::Peekable, str::Chars};

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // All of the machines in the factory are offline and the Elves don't have
    // the initialization procedure. To initialize a machine, a specific pattern
    // of indicator lights (on and off) must appear on the machine. Each machine
    // has a set of buttons which toggle a set of the ligths. The lights on
    // every machine all start in the off state, and we need to find the fewest
    // button presses to turn on every machine.

    // This pattern toggling system seems like a good candidate for bitwise XOR.
    let Some(_machines) = parse_machines(input) else {
        return Solution::ParseError;
    };

    Solution::default()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A machine.
struct Machine {
    /// The target pattern of indicator lights.
    target: u16,

    /// The indicator lights toggled by the buttons on the `Machine`.
    buttons: Box<[u16]>,
}

/// Parses a boxed slice of [`Machine`]s from input. This function returns
/// [`None`] if the [`Machine`]s could not be parsed.
fn parse_machines(input: &str) -> Option<Box<[Machine]>> {
    let mut machines = Vec::new();

    for line in input.lines().map(str::trim) {
        let machine = parse_machine(line)?;
        machines.push(machine);
    }

    Some(machines.into())
}

/// Parses a [`Machine`] from a line of input. This function returns [`None`] if
/// a [`Machine`] could not be parsed.
fn parse_machine(line: &str) -> Option<Machine> {
    let mut chars = line.chars().peekable();

    if chars.next()? != '[' {
        return None;
    }

    let mut target = 0;

    while *chars.peek()? != ']' {
        target <<= 1;
        let light = chars.next()?;

        if light == '#' {
            target |= 1;
        } else if light != '.' {
            return None;
        }
    }

    if !(0..1024).contains(&target) {
        return None;
    }

    if chars.next()? != ']' {
        return None;
    }

    let mut buttons = Vec::new();

    loop {
        match chars.next()? {
            c if c.is_whitespace() => (),
            '(' => buttons.push(parse_button(&mut chars)?),
            '{' => break,
            _ => return None,
        }
    }

    Some(Machine {
        target,
        buttons: buttons.into(),
    })
}

/// Parses a button from a character iterator after consuming its opening
/// parenthesis. This function returns [`None`] if a button could not be parsed.
fn parse_button(chars: &mut Peekable<Chars>) -> Option<u16> {
    let mut button = 0;

    loop {
        let bit = chars.next()?;

        if !bit.is_ascii_digit() {
            return None;
        }

        let bit = u16::try_from(bit).expect("bit should be ASCII") - u16::from(b'0');
        button |= 1 << bit;

        match chars.next()? {
            ')' => break,
            ',' => (),
            _ => return None,
        }
    }

    (1..1024).contains(&button).then_some(button)
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
