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

    /// Returns the optional day filter.
    pub fn day_filter(&self) -> Option<u8> {
        self.args.day_filter
    }
}

/// Command line arguments.
#[derive(Parser)]
#[command(about)]
struct Args {
    /// The optional day filter.
    #[arg(id = "day", help = "Filter day", short, long)]
    day_filter: Option<u8>,
}
