//! Perform dna reverse complement

#![warn(missing_docs)]

/* std use */

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use rust_template::cli;
use rust_template::error;
use rust_template::rev_comp;

fn main() -> error::Result<()> {
    // Parse argument
    let arguments = cli::Arguments::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(arguments.quiet())
        .verbosity(arguments.verbosity())
        .timestamp(arguments.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    log::info!("Start reverse complement sequence.");
    println!("{}", String::from_utf8(rev_comp(&arguments.input()))?);
    log::info!("End reverse complement sequence.");

    Ok(())
}
