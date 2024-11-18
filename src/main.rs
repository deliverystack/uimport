mod cli;
mod fileutils;
mod processor;

use clap::ArgMatches;

fn main() {
    // Parse command-line arguments
    let matches: ArgMatches = cli::parse_args();

    // Execute the main processing logic
    processor::run(matches);
}
