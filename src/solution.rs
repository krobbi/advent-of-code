use std::fmt::{self, Display, Formatter};

/// A solution to a [`Part`][crate::Part].
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Solution {
    /// A solution was found.
    Solved(String),

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
    pub fn is_benchable(&self) -> bool {
        matches!(self, Self::Solved(_) | Self::SolveError)
    }
}

impl<T: IntoSolution> From<T> for Solution {
    fn from(value: T) -> Self {
        value.into_solution()
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Solved(value) => return write!(f, "[{value}]"),
            Self::Incomplete => "incomplete",
            Self::ParseError => "parse error",
            Self::SolveError => "solve error",
        };

        f.write_str(message)
    }
}

/// A trait for values which may be converted to [`Solution`]s.
trait IntoSolution: ToString + Sized {
    /// Consumes the value and converts it to a [`Solution`].
    fn into_solution(self) -> Solution {
        Solution::Solved(self.to_string())
    }
}

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

impl IntoSolution for &str {
    fn into_solution(self) -> Solution {
        Solution::Solved(self.to_owned())
    }
}

impl IntoSolution for String {
    fn into_solution(self) -> Solution {
        Solution::Solved(self)
    }
}
