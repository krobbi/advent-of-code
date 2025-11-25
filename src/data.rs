use crate::Solution;

/// Advent of Code data.
#[derive(Default)]
pub struct Data {
    /// The [`Event`]s.
    events: Vec<Event>,
}

impl Data {
    /// Creates new `Data`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an [`Iterator`] over the [`Event`]s.
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    /// Inserts an [`Event`] into the `Data`.
    pub fn insert_event(&mut self, event: Event) {
        assert!(
            self.event(event.year).is_none(),
            "event year should not already exist"
        );

        let previous_year = self.events.last().map_or(0, |e| e.year);
        assert!(
            event.year > previous_year,
            "events should be in chronological order"
        );

        self.events.push(event);
    }

    /// Returns a reference to an [`Event`] from its year. This function returns
    /// [`None`] if the [`Event`] does not exist.
    fn event(&self, year: u16) -> Option<&Event> {
        self.events.iter().find(|e| e.year == year)
    }
}

/// An Advent of Code event.
pub struct Event {
    /// The year.
    pub year: u16,

    /// The [`Puzzle`]s.
    puzzles: Vec<Puzzle>,
}

impl Event {
    /// Creates a new `Event` from its year.
    pub fn new(year: u16) -> Self {
        Self {
            year,
            puzzles: Vec::with_capacity(year_event_day_count(year).into()),
        }
    }

    /// Returns `true` if the `Event` has a day.
    pub fn has_day(&self, day: u8) -> bool {
        (1..=self.day_count()).contains(&day)
    }

    /// Returns a reference to a [`Puzzle`] from its day. This function returns
    /// [`None`] if the [`Puzzle`] does not exist.
    pub fn puzzle(&self, day: u8) -> Option<&Puzzle> {
        self.puzzles.iter().find(|p| p.day == day)
    }

    /// Returns an [`Iterator`] over the [`Puzzle`]s.
    pub fn puzzles(&self) -> impl Iterator<Item = &Puzzle> {
        self.puzzles.iter()
    }

    /// Inserts a [`Puzzle`] into the `Event`.
    pub fn insert_puzzle(&mut self, puzzle: Puzzle) {
        assert!(self.has_day(puzzle.day), "puzzle day should be in range");
        assert!(
            self.puzzle(puzzle.day).is_none(),
            "puzzle day should not already exist"
        );

        let previous_day = self.puzzles.last().map_or(0, |p| p.day);
        assert!(
            puzzle.day > previous_day,
            "puzzles should be in chronological order"
        );

        self.puzzles.push(puzzle);
    }

    /// Returns the number of days in the `Event`.
    fn day_count(&self) -> u8 {
        year_event_day_count(self.year)
    }
}

/// A puzzle with two [`Part`]s.
pub struct Puzzle {
    /// The day.
    pub day: u8,

    /// The puzzle input path.
    pub input_path: &'static str,

    /// The first [`Part`].
    pub part_one: Part,

    /// The second [`Part`].
    pub part_two: Part,
}

/// A part of a [`Puzzle`].
#[derive(Clone, Copy)]
pub struct Part(pub fn(input: &str) -> Solution);

/// Returns the number of days in an [`Event`] from its year.
fn year_event_day_count(year: u16) -> u8 {
    // https://adventofcode.com/2025/about#faq_num_days
    if year >= 2025 { 12 } else { 25 }
}
