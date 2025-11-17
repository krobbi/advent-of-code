//! [Day 5: Doesn't He Have Intern-Elves For This?][link]
//!
//! [link]: https://adventofcode.com/2015/day/5

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa wants to know how many strings in his text file are nice.
    input.lines().filter(|&l| is_string_nice(l)).count().into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Returns `true` if a string is nice.
fn is_string_nice(string: &str) -> bool {
    // Nice strings contain at least three vowels.
    let mut vowel_count = 0;

    // Nice strings contain at least one letter that appears twice in a row.
    let mut previous_letter = '\0';
    let mut has_double = false;

    // Nice strings must not contain the pairs "ab", "cd", "pq", or "xy".
    for letter in string.chars() {
        match (previous_letter, letter) {
            (_, 'a' | 'e' | 'i' | 'o'| 'u') => vowel_count += 1,
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            _ => (),
        }

        if letter == previous_letter {
            has_double = true;
        }

        previous_letter = letter;
    }

    has_double && vowel_count >= 3
}
