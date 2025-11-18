//! [Day 7: Some Assembly Required][link]
//!
//! [link]: https://adventofcode.com/2015/day/7

use std::collections::{HashMap, VecDeque};

use crate::Solution;

/// Solves part one.
pub fn part_one(input: &str) -> Solution {
    // Keep track of Bobby's circuit and instruction booklet.
    let Some((mut circuit, instructions)) = create_circuit(input) else {
        return Solution::ParseError;
    };

    // Follow the instructions until they are all complete and find the signal
    // on wire "a".
    if let Some(signal) = follow_instructions(instructions, &mut circuit) {
        signal.into()
    } else {
        Solution::SolveError
    }
}

/// Solves part two.
pub fn part_two(input: &str) -> Solution {
    // Start by doing the same thing as part one.
    let Some((mut circuit, instructions)) = create_circuit(input) else {
        return Solution::ParseError;
    };

    let Some(signal) = follow_instructions(instructions.clone(), &mut circuit) else {
        return Solution::SolveError;
    };

    // Override the signal on wire "b" with the signal from wire "a" and reset
    // the other wires.
    circuit.clear();
    let wire_b_id = circuit.get_wire_id("b");
    circuit.set_signal(wire_b_id, signal);

    // Follow the instructions again and find the new signal on wire "a".
    if let Some(signal) = follow_instructions(instructions, &mut circuit) {
        signal.into()
    } else {
        Solution::SolveError
    }
}

/// Follows a queue of [`Instruction`]s on a circuit and returns the signal on
/// wire "a". This function returns [`None`] if wire "a" has no signal.
fn follow_instructions(
    mut instructions: VecDeque<Instruction>,
    circuit: &mut Circuit,
) -> Option<u16> {
    // Keep following instructions until there are none left to follow.
    while let Some(instruction) = instructions.pop_front() {
        if let Some(signal) = instruction.input.eval_signal(circuit) {
            // A signal was found from the instruction, set it in the circuit
            // and discard the instruction.
            circuit.set_signal(instruction.output, signal);
        } else {
            // The instruction can't be evaluated yet, move it to the back of
            // the queue.
            instructions.push_back(instruction);
        }
    }

    let wire_a_id = circuit.get_wire_id("a");
    circuit.get_signal(wire_a_id)
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

    /// Clears the `Circuit`'s signals.
    fn clear(&mut self) {
        self.wire_signals.fill(None);
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
        if self.get_signal(id).is_none() {
            self.wire_signals[usize::from(id.0)] = Some(signal);
        }
    }
}

/// A unique identifier for a wire.
#[derive(Clone, Copy)]
struct WireId(u16);

/// An instruction for connecting wires.
#[derive(Clone)]
struct Instruction {
    /// The input [`Gate`].
    input: Gate,

    /// The output [`WireId`].
    output: WireId,
}

/// A logic gate.
#[derive(Clone)]
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

/// Creates a new [`Circuit`] and queue of [`Instruction`]s from an instruction
/// booklet. This function returns [`None`] if the instruction booklet could not
/// be parsed.
fn create_circuit(booklet: &str) -> Option<(Circuit, VecDeque<Instruction>)> {
    let mut circuit = Circuit::new();
    let mut instructions = VecDeque::new();

    for line in booklet.lines() {
        let instruction = parse_instruction(line, &mut circuit)?;
        instructions.push_back(instruction);
    }

    Some((circuit, instructions))
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
