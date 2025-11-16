//! [Day 2: I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2)

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    let mut area = 0;

    // Add up the area of wrapping paper needed for each present.
    for line in input.lines() {
        let (length, width, height) = parse_present(line);

        // Calculate the areas of wrapping paper needed for each kind of face.
        let top_area = length * width;
        let front_area = width * height;
        let side_area = height * length;

        // Find the smallest face area for extra slack wrapping paper.
        let slack = top_area.min(front_area).min(side_area);

        // There are two of each kind of face.
        area += 2 * top_area + 2 * front_area + 2 * side_area + slack;
    }

    area.into()
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let mut ribbon_length = 0;

    // Add up the length of ribbon needed for each present.
    for line in input.lines() {
        let (length, width, height) = parse_present(line);

        // Calculate the perimeters for each kind of face.
        let top_perimeter = 2 * (length + width);
        let front_perimeter = 2 * (width + height);
        let side_perimeter = 2 * (height + length);

        // Find the minimum length of ribbon to wrap the present.
        let wrap_length = top_perimeter.min(front_perimeter).min(side_perimeter);

        // The volume of the present is equal to the length of ribbon needed to
        // tie a bow.
        let volume = length * width * height;

        ribbon_length += wrap_length + volume;
    }

    ribbon_length.into()
}

/// Parses the length, width, and height of a present from a line of text.
fn parse_present(line: &str) -> (u32, u32, u32) {
    let mut line = line.split('x');
    let length = line.next().unwrap().parse().unwrap();
    let width = line.next().unwrap().parse().unwrap();
    let height = line.next().unwrap().parse().unwrap();
    (length, width, height)
}
