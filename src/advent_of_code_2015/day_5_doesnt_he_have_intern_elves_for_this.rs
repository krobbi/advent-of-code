//! [Day 5: Doesn't He Have Intern-Elves For This?][link]
//!
//! [link]: https://adventofcode.com/2015/day/5

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Santa wants to know how many strings in his text file are nice.
    solve_part(input, is_string_nice_part_one)
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Now he has changed his rules for nice strings.
    solve_part(input, is_string_nice_part_two)
}

/// Solves a part with a predicate for whether a string is nice.
fn solve_part(input: &str, is_string_nice: fn(string: &str) -> bool) -> Solution {
    input.lines().filter(|&l| is_string_nice(l)).count().into()
}

/// Returns `true` if a string is nice for part one.
fn is_string_nice_part_one(string: &str) -> bool {
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

/// Returns `true` if a string is nice for part two.
fn is_string_nice_part_two(string: &str) -> bool {
    // Convert the string to fixed-size characters. This allows sliding windows
    // and faster indexing.
    let string: Box<[char]> = string.chars().collect();

    // A string is nice if any pair of letters appears at least twice. Overlaps
    // of the same three letters don't count.
    let mut has_matching_pairs = false;

    'check_pairs: for (index, pair) in string.windows(2).map(|w| (w[0], w[1])).enumerate() {
        for other_pair in string[index + 2..].windows(2).map(|w| (w[0], w[1])) {
            if pair == other_pair {
                has_matching_pairs = true;
                break 'check_pairs;
            }
        }
    }

    if !has_matching_pairs {
        return false;
    }

    // Nice strings must also have a repeated letter with any letter between
    // them.
    for window in string.windows(3) {
        if window[0] == window[2] {
            return true;
        }
    }

    false
}
