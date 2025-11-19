//! [Day 12: JSAbacusFramework.io][link]
//!
//! [link]: https://adventofcode.com/2015/day/12

use json::JsonValue;

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // The Accounting-Elves need help balancing the books and want to find the
    // sum of all the numbers in their JSON document. Luckily no numbers appear
    // in strings, so the actual structure can be ignored for now.
    let mut sum = 0;
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        if !is_char_number(char) {
            continue;
        }

        let mut number = char.to_string();

        while let Some(char) = chars.next_if(|c| is_char_number(*c)) {
            number.push(char);
        }

        sum += number.parse::<i32>().expect("number should be valid");
    }

    sum.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // The Accounting-Elves have double-counted everything red. Numbers in
    // objects (but not arrays) and their children that are red need to be
    // ignored.
    let value = json::parse(input).expect("input should be valid JSON");
    eval_json(&value).into()
}

/// Returns `true` if a [`char`] is part of a number.
fn is_char_number(char: char) -> bool {
    char.is_ascii_digit() || char == '-'
}

/// Evaluates a [`JsonValue`].
fn eval_json(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Number(number) => number
            .as_fixed_point_i64(0)
            .expect("numbers should be integers"),
        JsonValue::Array(values) => {
            let mut sum = 0;

            for value in values {
                sum += eval_json(value);
            }

            sum
        }
        JsonValue::Object(object) => {
            let mut sum = 0;

            for (_, value) in object.iter() {
                if is_value_red(value) {
                    return 0;
                }

                sum += eval_json(value);
            }

            sum
        }
        _ => 0,
    }
}

/// Returns `true` if a [`JsonValue`] is the string `"red"`.
fn is_value_red(value: &JsonValue) -> bool {
    match value {
        JsonValue::Short(short) => short.as_str() == "red",
        JsonValue::String(string) => string == "red",
        _ => false,
    }
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
