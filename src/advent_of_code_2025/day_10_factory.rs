//! [Day 10: Factory][link]
//!
//! [link]: https://adventofcode.com/2025/day/10

use std::{iter::Peekable, str::Chars};

use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};

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

    machines
        .iter()
        .map(solve_machine_part_one)
        .sum::<u32>()
        .into()
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

    let mut total_button_presses = 0;

    for machine in machines {
        let Some(machine_button_presses) = solve_machine_part_two(&machine) else {
            return Solution::SolveError;
        };

        total_button_presses += machine_button_presses;
    }

    total_button_presses.into()
}

/// Solves part one for one [`Machine`].
fn solve_machine_part_one(machine: &Machine) -> u32 {
    // My original solution for part one used a depth-first search with a cache
    // of the least number of button presses to reach a state of indicator
    // lights. Each button is essentially an XOR operation, so I optimised it
    // after making these observations:
    // * XOR is commutative - it does not matter what order a sequence of XOR
    //   operations is performed in because it always produces the same result.
    //   Therefore, it does not matter what order the buttons are pressed in.
    // * XOR is inversive - repeating an XOR operation will produce the input as
    //   a result. Therefore, pressing any button more than once undoes any work
    //   it did, and will not contribute to the solution.
    // This means that the solution must be a combination of each button either
    // being pressed or not pressed. We can iterate over every possible solution
    // to find the best one.
    let mut fewest_button_presses = u32::try_from(machine.buttons.len())
        .expect("there should be fewer than `u32::MAX` buttons on a machine");

    let combination_count = 1u16 << fewest_button_presses;

    for mask in 0..combination_count {
        let mut lights = machine.target;
        let mut bit = 1;

        for button in &machine.buttons {
            if mask & bit != 0 {
                lights ^= button;
            }

            bit <<= 1;
        }

        if lights == 0 {
            fewest_button_presses = fewest_button_presses.min(mask.count_ones());
        }
    }

    fewest_button_presses
}

/// Solves part two for one [`Machine`]. Returns [`None`] if the [`Machine`]
/// could not be solved.
fn solve_machine_part_two(machine: &Machine) -> Option<u32> {
    // I got stuck on this part. Looking online, I saw that some people were
    // using "Z3" or some other library. I did not know about linear solvers
    // before, so I think the experience offsets breaking my own rules about
    // dependencies.

    // There is a variable for every number of button presses.
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let mut button_vars = Vec::new();

    for _ in 0..machine.buttons.len() {
        let button_var = problem.add_integer_var(1.0, (0, i32::MAX));
        button_vars.push(button_var);
    }

    // Every joltage level should be equal to the number of button presses which
    // affect it.
    for (joltage_index, joltage) in machine.joltage_requirements.iter().copied().enumerate() {
        let button_mask = 1 << joltage_index;
        let mut constrained_vars = LinearExpr::empty();

        for (button_index, button) in machine.buttons.iter().copied().enumerate() {
            if button & button_mask == 0 {
                continue;
            }

            constrained_vars.add(button_vars[button_index], 1.0);
        }

        problem.add_constraint(constrained_vars, ComparisonOp::Eq, f64::from(joltage));
    }

    let solution = problem.solve().ok()?;

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "solution should be a positive integer"
    )]
    Some(solution.objective().round() as u32)
}

/// A machine.
struct Machine {
    /// The target pattern of indicator lights.
    target: u16,

    /// The buttons on the `Machine`, which are bit masks for toggling indicator
    /// lights and incrementing joltage.
    buttons: Box<[u16]>,

    /// The joltage requirements.
    joltage_requirements: Box<[u16]>,
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

    if buttons.is_empty() {
        return None;
    }

    let joltage_requirements = parse_joltage_requirements(&mut chars)?;

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

/// Parses a set of joltage requirements from a character iterator after
/// consuming its opening brace. This function returns [`None`] if joltages
/// could not be parsed.
fn parse_joltage_requirements(chars: &mut Peekable<Chars>) -> Option<Box<[u16]>> {
    let mut joltages = Vec::new();

    loop {
        let mut joltage = String::new();

        while let Some(digit) = chars.next_if(char::is_ascii_digit) {
            joltage.push(digit);
        }

        let joltage = joltage.parse().ok()?;
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

    Some(joltages.into())
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
