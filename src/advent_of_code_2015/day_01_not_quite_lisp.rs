//! [Day 1: Not Quite Lisp][link]
//!
//! [link]: https://adventofcode.com/2015/day/1

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa starts on the ground floor.
    let mut floor = 0;

    // He follows the instructions one character at a time.
    for instruction in input.chars() {
        floor += instruction_effect(instruction);
    }

    // Find the floor he ends up on.
    floor.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Again, Santa starts on the ground floor.
    let mut floor = 0;

    // Find the position of the first instruction that causes him to enter the
    // basement.
    for (position, instruction) in input.chars().enumerate() {
        floor += instruction_effect(instruction);

        if floor < 0 {
            // The solution should be 1-indexed.
            return (position + 1).into();
        }
    }

    // Santa never entered the basement.
    Solution::SolveError
}

/// Returns the effect of an instruction on Santa's floor number.
fn instruction_effect(instruction: char) -> i16 {
    // Santa goes up one floor on '(' and down one floor on ')'. Any other
    // instructions can be ignored.
    match instruction {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one("(())"), Solution::Solved(0));
        assert_eq!(part_one("()()"), Solution::Solved(0));
        assert_eq!(part_one("((("), Solution::Solved(3));
        assert_eq!(part_one("(()(()("), Solution::Solved(3));
        assert_eq!(part_one("))((((("), Solution::Solved(3));
        assert_eq!(part_one("())"), Solution::Solved(-1));
        assert_eq!(part_one("))("), Solution::Solved(-1));
        assert_eq!(part_one(")))"), Solution::Solved(-3));
        assert_eq!(part_one(")())())"), Solution::Solved(-3));
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(")"), Solution::Solved(1));
        assert_eq!(part_two("()())"), Solution::Solved(5));
    }
}
