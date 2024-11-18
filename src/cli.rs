use clap::{Arg, ArgMatches, Command};

/// Parse command-line arguments and return them as ArgMatches
pub fn parse_args() -> ArgMatches {
    Command::new("uimport")
        .version("1.1")
        .author("Your Name")
        .about("Processes files from source to target with options for cleanup and organization")
        .arg(
            Arg::new("batch")
                .short('b')
                .long("batch")
                .value_name("BATCH_SIZE")
                .help("Processes files in concurrent batches of the specified size"),
        )
        .arg(
            Arg::new("dated")
                .short('d')
                .long("dated")
                .help("Includes the file's year and month in the target file path"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Forces creation of the target directory if it does not exist"),
        )
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .value_name("SOURCE")
                .help("Specifies the source directory")
                .required(true),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET")
                .help("Specifies the target directory")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enables verbose mode"),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help("Displays the version information")
                .action(clap::ArgAction::Version), // Automatically shows version information
        )
        .get_matches()
}
