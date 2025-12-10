//! [Day 10: Factory][link]
//!
//! [link]: https://adventofcode.com/2025/day/10

use std::{collections::HashMap, iter::Peekable, str::Chars};

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
    let Some(machines) = parse_machines(input) else {
        return Solution::ParseError;
    };

    let mut total_button_presses = 0;

    for machine in &machines {
        let Some(button_presses) = solve_machine_part_one(machine) else {
            return Solution::SolveError;
        };

        total_button_presses += button_presses;
    }

    total_button_presses.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now that the machines have been initialized, we need to set their joltage
    // requirements. Instead of toggling indicator lights, each button
    // increments the corresponding joltage by one. Again, we need to find the
    // fewest button presses to reach the joltage level.
    let Some(machines) = parse_machines(input) else {
        return Solution::ParseError;
    };

    for machine in machines {
        println!("{:?}", machine.joltage_requirements.0);
    }

    Solution::default()
}

/// Solves part one for one [`Machine`]. Returns [`None`] if the [`Machine`]
/// cannot be solved.
fn solve_machine_part_one(machine: &Machine) -> Option<u32> {
    let mut light_costs = HashMap::new();
    let mut unexplored_states = vec![(0, 0)];

    while let Some((lights, cost)) = unexplored_states.pop() {
        if let Some(explored_cost) = light_costs.get(&lights).copied()
            && explored_cost <= cost
        {
            // This state was already reached with a better or equal cost.
            continue;
        }

        // A state was discovered or was reached with a better cost.
        light_costs.insert(lights, cost);

        // Explore all the states that can be reached from this state.
        for button in &machine.buttons {
            let lights = lights ^ button;
            let cost = cost + 1;
            unexplored_states.push((lights, cost));
        }
    }

    light_costs.get(&machine.target).copied()
}

/// A machine.
struct Machine {
    /// The target pattern of indicator lights.
    target: u16,

    /// The buttons on the `Machine` for toggling indicator lights and
    /// incrementing joltage.
    buttons: Box<[u16]>,

    /// The joltage requirements.
    joltage_requirements: Joltages,
}

/// A set of joltages.
#[derive(Clone, PartialEq, Eq, Hash)]
struct Joltages([u16; 10]);

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
    let mut lights = String::new();

    while *chars.peek()? != ']' {
        lights.push(chars.next()?);
    }

    for light in lights.chars().rev() {
        target <<= 1;

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

    let joltage_requirements = parse_joltages(&mut chars)?;

    Some(Machine {
        target,
        buttons: buttons.into(),
        joltage_requirements,
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

/// Parses a set of [`Joltages`] from a character iterator after consuming its
/// opening brace. This function returns [`None`] if [`Joltages`] could not be
/// parsed.
fn parse_joltages(chars: &mut Peekable<Chars>) -> Option<Joltages> {
    let mut joltages = Vec::new();

    loop {
        let mut joltage = String::new();

        while let Some(digit) = chars.next_if(char::is_ascii_digit) {
            joltage.push(digit);
        }

        let joltage = joltage.parse().ok()?;

        if joltage == 0xffff {
            // We need room to check for overjoltage.
            return None;
        }

        joltages.push(joltage);

        match chars.next()? {
            '}' => break,
            ',' => (),
            _ => return None,
        }
    }

    if joltages.len() > 10 {
        return None;
    }

    let mut raw_joltages = [0; 10];

    for (index, joltage) in joltages.iter().copied().enumerate() {
        raw_joltages[index] = joltage;
    }

    Some(Joltages(raw_joltages))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example input for testing.
    static INPUT: &str = "\
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 7.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 33.into());
    }
}
