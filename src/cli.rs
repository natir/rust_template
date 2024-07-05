//! Command Line Interface of rust_template

/* std use */

/* crate use */

/* project use */

#[derive(clap::Parser, std::fmt::Debug)]
#[clap(
    name = "rust_template",
    bin_name = "rust_template",
    version = "0.1.0",
    author = "Pierre Marijon <pierre@marijon.fr>"
)]
pub struct Arguments {
    /// Input sequence
    #[clap(short = 'i', long = "input")]
    input: String,

    // Generic parameter
    /// Silence all output
    #[clap(short = 'q', long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[clap(short = 'v', long = "verbosity", action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Timestamp (sec, ms, ns, none)
    #[clap(short = 'T', long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
}

impl Arguments {
    /// Get input sequence
    pub fn input(&self) -> Vec<u8> {
        self.input.as_bytes().to_vec()
    }

    /// Get verbosity level
    pub fn verbosity(&self) -> usize {
        self.verbosity as usize
    }

    /// Get quiet
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    /// Get timestamp granularity
    pub fn timestamp(&self) -> stderrlog::Timestamp {
        self.ts.unwrap_or(stderrlog::Timestamp::Off)
    }
}
