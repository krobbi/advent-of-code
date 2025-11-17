# Advent of Code
These are my attempts at solving puzzles from
[Advent of Code](https://adventofcode.com/) - an annual advent calendar of
programming puzzles. I am using [Rust](https://rust-lang.org/).

Puzzle inputs are not included in this repository by request of the
[Advent of Code FAQ](https://adventofcode.com/about#faq_copying).

Puzzle inputs are given similar names to the code that solves them. For
example, the puzzle input for `src/advent_of_code_2015/day_1_not_quite_lisp.rs`
should be located at `inputs/advent_of_code_2015/day_1_not_quite_lisp.txt`.

# Issues
Puzzle solution modules are declared in a macro to avoid repetition. These
modules will not be formatted if you use `cargo fmt`.

# Dependencies
Dependencies are mostly avoided for puzzle solutions, but they are sometimes
used to avoid "reinventing the wheel":
* [md5](https://crates.io/crates/md5) - MD5 hashing in 2015 day 4

# Credits
Advent of Code was created by [Eric Wastl](https://was.tl/) and is a registered
trademark in the United States. This repository is not affiliated with Advent
of Code or Eric Wastl.

The Rust code in this repository is not released under any specific license. It
may be used freely, but I suggest you do not use it to fill in solutions for
your own attempts.
