mod data;
mod solution;

pub use crate::solution::Solution;

use std::{
    fs,
    io::{self, Write as _},
    path::Path,
    time::Instant,
};

use crate::data::{Data, Event, Part, Puzzle};

/// Defines the [`Data`].
macro_rules! define_data {
    {$($year:literal: $event:ident {$($day:literal: $puzzle:ident),* $(,)?}),* $(,)?} => {
        $(mod $event {
            $(pub mod $puzzle;)*
        })*

        #[doc = "Creates new [`Data`]."]
        fn create_data() -> Data {
            let mut data = Data::new();
            $(#[allow(unused_mut, reason = "some events may not have any completed puzzles")]
            let mut event = Event::new($year);
            $(event.insert_puzzle(Puzzle {
                #[allow(clippy::zero_prefixed_literal, reason = "consistent formatting")]
                day: $day,
                input_path: concat!(
                    "inputs/",
                    stringify!($event),
                    "/",
                    stringify!($puzzle),
                    ".txt",
                ),
                part_one: Part(crate::$event::$puzzle::part_one),
                part_two: Part(crate::$event::$puzzle::part_two),
            });)*

            data.insert_event(event);)*
            data
        }
    };
}

define_data! {
    2015: advent_of_code_2015 {
        01: day_01_not_quite_lisp,
        02: day_02_i_was_told_there_would_be_no_math,
        03: day_03_perfectly_spherical_houses_in_a_vacuum,
        04: day_04_the_ideal_stocking_stuffer,
        05: day_05_doesnt_he_have_intern_elves_for_this,
        06: day_06_probably_a_fire_hazard,
        07: day_07_some_assembly_required,
        08: day_08_matchsticks,
        09: day_09_all_in_a_single_night,
        10: day_10_elves_look_elves_say,
        11: day_11_corporate_policy,
        12: day_12_js_abacus_framework_io,
    },
}

/// Solves every completed [`Puzzle`].
fn main() {
    let data = create_data();

    for event in data.events() {
        run_event(event);
    }
}

/// Runs an [`Event`] and prints its result.
fn run_event(event: &Event) {
    println!("Advent of Code {}:", event.year);

    for puzzle in event.puzzles() {
        run_puzzle(puzzle);
    }
}

/// Runs a [`Puzzle`] and prints its result.
fn run_puzzle(puzzle: &Puzzle) {
    print!("  Day {:02} - ", puzzle.day);
    flush_stdout();

    let input_path = Path::new(puzzle.input_path);
    assert!(
        input_path.is_relative(),
        "puzzle input path should be relative"
    );

    if !input_path.is_file() {
        println!("Puzzle input missing - {}", input_path.display());
        return;
    }

    let input = match fs::read_to_string(input_path) {
        Ok(input) => input,
        Err(error) => {
            println!("Error");
            eprintln!("Could not read puzzle input: {error}");
            return;
        }
    };

    run_part(puzzle.part_one, &input);
    print!(", ");
    flush_stdout();

    run_part(puzzle.part_two, &input);
    println!();
}

/// Runs a [`Part`] with a puzzle input and prints its result.
fn run_part(part: Part, input: &str) {
    let start = Instant::now();
    let solution = part.0(input);
    let benchmark = start.elapsed();
    print!("{solution}");

    if solution.is_benchable() {
        print!(" in {benchmark:?}");
    }
}

/// Flushes the standard output stream.
fn flush_stdout() {
    io::stdout()
        .flush()
        .expect("flushing stdout should not fail");
}
