use std::fmt::{self, Display, Formatter};

/// A value stored in a [`Solution`].
type Value = i32;

/// A solution to a [`Part`][crate::Part].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Solution {
    /// A solution was found.
    Solved(Value),

    /// A solution which overflows a [`Value`] was found.
    Overflowed,

    /// A [`Part`][crate::Part] was defined with no solution.
    #[allow(dead_code, reason = "all puzzles may be completed")]
    #[default]
    Incomplete,

    /// A [`Part`][crate::Part] could not parse its puzzle input.
    ParseError,

    /// A [`Part`][crate::Part] entered an unsolvable state.
    SolveError,
}

impl Solution {
    /// Returns `true` if the `Solution` is applicable for benchmarking.
    pub fn is_benchable(self) -> bool {
        self != Self::Incomplete
    }
}

impl<T: IntoSolution> From<T> for Solution {
    fn from(value: T) -> Self {
        match value.try_into() {
            Ok(value) => Self::Solved(value),
            Err(_) => Self::Overflowed,
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Solved(value) => return value.fmt(f),
            Self::Incomplete => "incomplete",
            Self::Overflowed => "overflowed",
            Self::ParseError => "failed to parse puzzle input",
            Self::SolveError => "failed to solve puzzle",
        };

        f.write_str(message)
    }
}

/// A trait for integers which may be converted to a [`Solution`].
trait IntoSolution: TryInto<Value> {}

impl IntoSolution for i8 {}
impl IntoSolution for u8 {}
impl IntoSolution for i16 {}
impl IntoSolution for u16 {}
impl IntoSolution for i32 {}
impl IntoSolution for u32 {}
impl IntoSolution for i64 {}
impl IntoSolution for u64 {}
impl IntoSolution for isize {}
impl IntoSolution for usize {}
