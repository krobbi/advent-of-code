use clap::Parser;

/// Configuration data.
pub struct Config {
    /// The [`Args`].
    args: Args,
}

impl Config {
    /// Creates a new `Config` from command line arguments or exits.
    pub fn new() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    /// Returns the optional year filter.
    pub fn year_filter(&self) -> Option<u16> {
        self.args.year
    }

    /// Returns the optional day filter.
    pub fn day_filter(&self) -> Option<u8> {
        self.args.day
    }
}

/// Command line arguments.
#[derive(Parser)]
#[command(about)]
struct Args {
    /// The optional year filter.
    #[arg(help = "Filter year")]
    year: Option<u16>,

    /// The optional day filter.
    #[arg(help = "Filter day")]
    day: Option<u8>,
}
