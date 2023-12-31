//! Main executable for the CLI.
#![allow(clippy::print_stdout)]

use clap::Parser;
use error_stack::Report;
use messagepack_cli::{Cli, Error};

/// Execute the CLI.
fn main() -> Result<(), Report<Error>> {
	let cli = Cli::parse();
	let converter = cli.into_converter()?;
	converter.execute()?;
	Ok(())
}
