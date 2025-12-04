# Advent of Code
These are my attempts at solving puzzles from
[Advent of Code](https://adventofcode.com/)
(an annual advent calendar of programming puzzles) using
[Rust](https://rust-lang.org/).

Puzzle text and inputs are not included in this repository by request of the
[Advent of Code FAQ](https://adventofcode.com/about#faq_copying). However, the
example inputs and outputs have been copied for use in unit tests.

Puzzle inputs are given similar names to the code that solves them. For
example, the puzzle input for `src/advent_of_code_2025/day_03_lobby.rs` should
be located at `inputs/advent_of_code_2025/day_03_lobby.txt`.

## Usage
The solutions are run from the command line:
```shell
cargo run --release -- [YEAR] [DAY]
```

### Arguments
| Argument | Usage       |
| :------- | :---------- |
| `[YEAR]` | Filter year |
| `[DAY]`  | Filter day  |

The solutions can optionally be filtered to a single year, or filtered further
to a single day.

### Options
| Short | Long     | Usage      |
| :---- | :------- | :--------- |
| `-h`  | `--help` | Print help |

## Dependencies
Dependencies are mostly avoided for puzzle solutions, but they are sometimes
used to avoid "reinventing the wheel":
* [clap](https://crates.io/crates/clap) - Command line argument parsing
* [json](https://crates.io/crates/json) - JSON parsing in 2015 day 12
* [md5](https://crates.io/crates/md5) - MD5 hashing in 2015 day 4

## Credits
Advent of Code was created by [Eric Wastl](https://was.tl/) and is a registered
trademark in the United States. This repository is not affiliated with Advent
of Code or Eric Wastl.

The Rust code in this repository is not released under any specific license. It
may be used freely, but I suggest you do not use it to fill in solutions for
your own attempts.
