//! [Day 4: The Ideal Stocking Stuffer][link]
//!
//! [link]: https://adventofcode.com/2015/day/4

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // This is a tricky one. A number needs to be appended to a string so that
    // its MD5 hash starts with at least five zeroes. The number also needs to
    // be as low as possible. I know that MD5 is considered vulnerable, but I
    // don't know how exactly.

    // The given examples might be helpful. It says that for `abcdef`, the
    // number is `609043`. For `pqrstuv`, the number is `1048970`. I have
    // noticed that these have the same number of characters as the strings.
    // Maybe the bits in the number somehow "cancel out" the ones in the string.

    // How is it possible to know if the number is the lowest possible? Maybe
    // each character of the number can be compared with a each character of the
    // string independently, and the smallest useful character is chosen. Null
    // characters might also have to be tested at the end in case the number is
    // shorter than the string. The number is probably not longer.

    // Despite this, brute force is fast enough for finding five zeroes in under
    // a second.
    part_one_brute_force(input)
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// Solves part one with brute force.
fn part_one_brute_force(input: &str) -> Solution {
    // Keep the secret key length in a variable. This might be a meaningless
    // micro-optimization, but speed is important here.
    let len = input.len();

    // Store the secret key in a byte buffer.
    let mut buffer = input.as_bytes().to_vec();

    // The number is probably the same length as the secret key, so reserve that
    // many bytes to avoid reallocating.
    buffer.reserve(len);

    // Check every number without leading zeroes.
    for number in 1..=u32::MAX {
        // Truncate the buffer to the length of the secret key. Don't bother
        // emptying and refilling it.
        buffer.truncate(len);

        buffer.extend_from_slice(number.to_string().as_bytes());
        let hash = md5::compute(&buffer).0;

        // Check if the first 5 hex digits of the hash are zero.
        if u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]) <= 0x00000fff {
            return number.into();
        }
    }

    Solution::SolveError
}
