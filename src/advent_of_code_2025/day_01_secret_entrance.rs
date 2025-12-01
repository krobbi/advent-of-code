//! [Day 1: Secret Entrance][link]
//!
//! [link]: https://adventofcode.com/2025/day/1

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // We need the password to the North Pole base so we can finish decorating
    // it. It's locked in a safe with a circular dial with numbers between 0 and
    // 99. The dial starts at 50 and we have a document with a list of rotations
    // to open the safe. The actual password in the safe is a decoy. The real
    // password is the number of times the dial points to 0 after a rotation.
    let Some(rotations) = parse_document(input) else {
        return Solution::ParseError;
    };

    let mut dial = 50;
    let mut zero_hits = 0;

    for rotation in rotations {
        // Add 1000 (multiple of 100) to the rotation because to avoid negative
        // modulo.
        dial = (dial + rotation + 1000) % 100;

        if dial == 0 {
            zero_hits += 1;
        }
    }

    zero_hits.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // We used the wrong method for finding the password. We actually need to
    // count the number of clicks of the dial that point to 0.
    let Some(rotations) = parse_document(input) else {
        return Solution::ParseError;
    };

    let mut dial = 50;
    let mut zero_clicks = 0;

    for mut rotation in rotations {
        // Not a very nice solution, couldn't seem to get the right formula.
        while rotation > 0 {
            rotation -= 1;
            dial += 1;

            if dial == 100 {
                dial = 0;
                zero_clicks += 1;
            }
        }

        while rotation < 0 {
            rotation += 1;
            dial -= 1;

            if dial == 0 {
                zero_clicks += 1;
            } else if dial == -1 {
                dial = 99;
            }
        }
    }

    zero_clicks.into()
}

/// Parses a boxed slice of rotations from a document. This function returns
/// [`None`] if the document could not be parsed.
fn parse_document(document: &str) -> Option<Box<[i16]>> {
    let mut rotations = Vec::new();

    for rotation in document.lines() {
        rotations.push(parse_rotation(rotation)?);
    }

    Some(rotations.into())
}

/// Parses a rotation. This function returns [`None`] if the rotation could not
/// be parsed.
fn parse_rotation(rotation: &str) -> Option<i16> {
    let sign = match rotation.chars().next()? {
        'L' => -1,
        'R' => 1,
        _ => return None,
    };

    let magnitude = rotation[1..].parse::<i16>().ok()?;
    Some(sign * magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example document for testing.
    static DOCUMENT: &str = "L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82\n";

    /// Tests part one.
    #[test]
    fn part_one_works() {
        assert_eq!(part_one(DOCUMENT), 3.into());
    }

    /// Tests part two.
    #[test]
    fn part_two_works() {
        assert_eq!(part_two(DOCUMENT), 6.into());
    }
}
