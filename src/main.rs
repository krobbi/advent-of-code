mod solution;

pub use crate::solution::Solution;

use std::{
    env, fs,
    io::{self, Write as _},
    path::Path,
    time::Instant,
};

/// Defines the completed [`Puzzle`]s.
macro_rules! define_puzzles {
    {$(
        mod($year:literal) $event:ident {$(
            mod $puzzle:ident;
        )*}
    )*} => {
        $(
            mod $event {
                $(
                    mod $puzzle;
                )*

                use super::Puzzle;

                #[doc = "The event's puzzle input paths and [`Puzzle`]s."]
                pub static DATA: &[(&str, Puzzle)] = &[$((
                    concat!("inputs/", stringify!($event), "/", stringify!($puzzle), ".txt"),
                    ($puzzle::part_one, $puzzle::part_two),
                )),*];
            }
        )*

        #[doc = "The map of years to event data."]
        static YEARS: &[(u16, &[(&str, Puzzle)])] = &[$(
            ($year, $event::DATA)
        ),*];
    };
}

define_puzzles! {
    mod(2015) advent_of_code_2015 {
        mod day_1_not_quite_lisp;
        mod day_2_i_was_told_there_would_be_no_math;
        mod day_3_perfectly_spherical_houses_in_a_vacuum;
        mod day_4_the_ideal_stocking_stuffer;
        mod day_5_doesnt_he_have_intern_elves_for_this;
    }
}

/// A function which solves part of a puzzle.
type Part = fn(input: &str) -> Solution;

/// A pair of functions which solve a two-part puzzle.
type Puzzle = (Part, Part);

/// Solves every completed [`Puzzle`].
fn main() {
    let day_filter = parse_args();

    for (year, data) in YEARS {
        for (day, (path, puzzle)) in data.iter().copied().enumerate() {
            let day = (day + 1)
                .try_into()
                .expect("day should not be more than 25");

            if day_filter.is_some_and(|d| d != day) {
                continue;
            }

            print!("Advent of Code {year}, Day {day}: ");
            flush_stdout();

            let path = Path::new(path);
            assert!(path.is_relative(), "puzzle input path should be relative");

            if !Path::new(path).is_file() {
                println!("[puzzle input missing: {}]", path.display());
                continue;
            }

            let input = match fs::read_to_string(path) {
                Ok(input) => input,
                Err(error) => {
                    println!();
                    eprintln!("Error: puzzle input could not be read: {error}");
                    return;
                }
            };

            solve_part(puzzle.0, &input);
            print!(", ");
            flush_stdout();

            solve_part(puzzle.1, &input);
            println!();
        }
    }
}

/// Parses an optional day filter from command line arguments.
fn parse_args() -> Option<u8> {
    env::args().nth(1)?.parse().ok()
}

/// Flushes the standard output stream.
fn flush_stdout() {
    io::stdout()
        .flush()
        .expect("flushing stdout should not fail");
}

/// Solves a [`Part`] with a puzzle input and prints its result.
fn solve_part(part: Part, input: &str) {
    let bench = Instant::now();
    let solution = part(input);
    let duration = bench.elapsed();
    print!("[{solution}]");

    if solution.is_benchable() {
        print!(" in {duration:?}");
    }
}
