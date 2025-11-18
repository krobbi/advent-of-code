//! [Day 7: Some Assembly Required][link]
//!
//! [link]: https://adventofcode.com/2015/day/7

use std::collections::{HashMap, VecDeque};

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Keep track of the wires in Bobby's circuit.
    let mut circuit = Circuit::new();

    // Read all of the instructions to assemble the circuit.
    let mut instructions = VecDeque::new();

    for line in input.lines() {
        let Some(instruction) = parse_instruction(line, &mut circuit) else {
            return Solution::ParseError;
        };

        instructions.push_back(instruction);
    }

    // Follow the first instruction in the queue.
    while let Some(instruction) = instructions.pop_front() {
        match instruction.input.eval_signal(&circuit) {
            None => {
                // The instruction can't be evaluated yet, move it to the back
                // of the queue.
                instructions.push_back(instruction);
            }
            Some(signal) => {
                // A signal has been found from the instruction, set it in the
                // circuit and discard the instruction.
                circuit.set_signal(instruction.output, signal);
            }
        }
    }

    // Find the signal on wire "a".
    let a_wire_id = circuit.get_wire_id("a");

    if let Some(signal) = circuit.get_signal(a_wire_id) {
        signal.into()
    } else {
        Solution::SolveError
    }
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    let _ = input;
    Solution::default()
}

/// A circuit of wires with optional signals.
#[derive(Default)]
struct Circuit {
    /// The map of identifiers to [`WireId`]s.
    wire_ids: HashMap<String, WireId>,

    /// The signals of each wire.
    wire_signals: Vec<Option<u16>>,
}

impl Circuit {
    /// Creates a new `Circuit`.
    fn new() -> Self {
        Self::default()
    }

    /// Returns a [`WireId`] from an identifier.
    fn get_wire_id(&mut self, identifier: &str) -> WireId {
        if let Some(wire_id) = self.wire_ids.get(identifier).copied() {
            return wire_id;
        }

        let wire_id = WireId(
            self.wire_signals
                .len()
                .try_into()
                .expect("there should be less than 65536 wires"),
        );

        self.wire_signals.push(None);
        self.wire_ids.insert(identifier.to_owned(), wire_id);
        wire_id
    }

    /// Returns a wire's signal from its [`WireId`]. This function returns
    /// [`None`] if the wire has no signal.
    fn get_signal(&self, id: WireId) -> Option<u16> {
        self.wire_signals[usize::from(id.0)]
    }

    /// Sets a wire's signal from its [`WireId`].
    fn set_signal(&mut self, id: WireId, signal: u16) {
        self.wire_signals[usize::from(id.0)] = Some(signal);
    }
}

/// A unique identifier for a wire.
#[derive(Clone, Copy)]
struct WireId(u16);

/// An instruction for connecting wires.
struct Instruction {
    /// The input [`Gate`].
    input: Gate,

    /// The output [`WireId`].
    output: WireId,
}

/// A logic gate.
enum Gate {
    /// A unary operation.
    Unary(UnOp, Source),

    /// A binary operation.
    Binary(BinOp, Source, Source),
}

impl Gate {
    /// Evaluates the `Gate`'s signal. with a [`Circuit`]. This function returns
    /// [`None`] if the `Gate` has no signal.
    fn eval_signal(&self, circuit: &Circuit) -> Option<u16> {
        let signal = match self {
            Self::Unary(op, rhs) => {
                let rhs = rhs.eval_signal(circuit)?;
                op.eval_signal(rhs)
            }
            Self::Binary(op, lhs, rhs) => {
                let lhs = lhs.eval_signal(circuit)?;
                let rhs = rhs.eval_signal(circuit)?;
                op.eval_signal(lhs, rhs)
            }
        };

        Some(signal)
    }
}

/// A unary operator.
#[derive(Clone, Copy)]
enum UnOp {
    /// A direct source.
    Source,

    /// A bitwise not.
    Not,
}

impl UnOp {
    /// Evaluates the `UnOp`'s signal from its operand signal.
    fn eval_signal(self, rhs: u16) -> u16 {
        match self {
            Self::Source => rhs,
            Self::Not => !rhs,
        }
    }
}

/// A binary operation.
#[derive(Clone, Copy)]
enum BinOp {
    /// A bitwise and.
    And,

    /// A bitwise or.
    Or,

    /// A bitwise left shift.
    LeftShift,

    /// A bitwise right shift.
    RightShift,
}

impl BinOp {
    /// Evaluates the `BinOp`'s signal from its operand signals.
    fn eval_signal(self, lhs: u16, rhs: u16) -> u16 {
        match self {
            Self::And => lhs & rhs,
            Self::Or => lhs | rhs,
            Self::LeftShift => lhs << rhs,
            Self::RightShift => lhs >> rhs,
        }
    }
}

/// A signal source.
#[derive(Clone, Copy)]
enum Source {
    /// A specific value.
    Value(u16),

    /// A wire.
    Wire(WireId),
}

impl Source {
    /// Evaluates the `Source`'s signal with a [`Circuit`]. This function
    /// returns [`None`] if the `Source` has no signal.
    fn eval_signal(self, circuit: &Circuit) -> Option<u16> {
        match self {
            Self::Value(signal) => Some(signal),
            Self::Wire(id) => circuit.get_signal(id),
        }
    }
}

/// Parses an [`Instruction`] from a line of text with a [`Circuit`]. This
/// function returns [`None`] if an [`Instruction`] could not be parsed.
fn parse_instruction(line: &str, circuit: &mut Circuit) -> Option<Instruction> {
    let mut words = line.split(' ');

    let input = match words.next()? {
        "NOT" => {
            let rhs = parse_source(words.next()?, circuit)?;
            words.next(); // Skip "->".
            Gate::Unary(UnOp::Not, rhs)
        }
        word => {
            let lhs = parse_source(word, circuit)?;

            match words.next()? {
                "->" => Gate::Unary(UnOp::Source, lhs),
                word => {
                    let op = match word {
                        "AND" => BinOp::And,
                        "OR" => BinOp::Or,
                        "LSHIFT" => BinOp::LeftShift,
                        "RSHIFT" => BinOp::RightShift,
                        _ => return None,
                    };

                    let rhs = parse_source(words.next()?, circuit)?;
                    words.next(); // Skip "->".
                    Gate::Binary(op, lhs, rhs)
                }
            }
        }
    };

    let output = circuit.get_wire_id(words.next()?);
    Some(Instruction { input, output })
}

/// Parses a [`Source`] from a word with a [`Circuit`]. This function returns
/// [`None`] if a [`Source`] could not be parsed.
fn parse_source(word: &str, circuit: &mut Circuit) -> Option<Source> {
    let source = if word.chars().next()?.is_ascii_digit() {
        Source::Value(word.parse().ok()?)
    } else {
        Source::Wire(circuit.get_wire_id(word))
    };

    Some(source)
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
