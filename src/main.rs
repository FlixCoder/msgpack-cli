//! Main executable for the CLI.

use clap::Parser;
use error_stack::Report;
use msgpack_cli::{Cli, Error};

/// Execute the CLI.
fn main() -> Result<(), Report<Error>> {
	let cli = Cli::parse();
	let converter = cli.into_converter()?;
	converter.execute()?;
	Ok(())
}
