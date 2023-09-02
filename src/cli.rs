//! CLI implementation.

use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
	path::PathBuf,
};

use clap::{Args, Parser};
use error_stack::{report, Report, ResultExt};

use crate::{conversion::ConversionDirection, Converter, Error};

/// Simple CLI to convert MessagePack to JSON and vice versa. Automatically
/// attempts to detect the input format and outputs the respective other format.
/// Use the config options to override the automatic detection.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Direction of conversion.
	#[command(flatten)]
	direction: ConversionDirectionArgs,
	/// Input file path to use. Stdin is used if not given.
	#[arg(short, long)]
	input: Option<PathBuf>,
	/// Output file path to use. Stdout is used if not given.
	#[arg(short, long)]
	output: Option<PathBuf>,
}

/// Conversion direction argument group.
#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct ConversionDirectionArgs {
	/// Set the direction of conversion to "MsgPack to JSON".
	#[arg(long = "m2j")]
	msgpack2json: bool,
	/// Set the direction of conversion to "JSON to MsgPack".
	#[arg(long = "j2m")]
	json2msgpack: bool,
}

/// Validate the conversion direction to be sure not both directions are
/// set. Then return the conversion direction as enum.
impl TryFrom<ConversionDirectionArgs> for ConversionDirection {
	type Error = Report<Error>;

	fn try_from(direction: ConversionDirectionArgs) -> Result<Self, Self::Error> {
		if direction.msgpack2json && direction.json2msgpack {
			return Err(report!(Error::MultipleConversionDirections));
		}

		if direction.msgpack2json {
			Ok(Self::MsgPack2Json)
		} else if direction.json2msgpack {
			Ok(Self::Json2MsgPack)
		} else {
			Ok(Self::Auto)
		}
	}
}

impl Cli {
	/// Use the input configuration to construct the execution converter.
	pub fn into_converter(self) -> Result<Converter, Report<Error>> {
		let direction = self.direction.try_into()?;

		let input: Box<dyn Read> = if let Some(input_file) = self.input {
			let file = File::open(input_file).change_context(Error::FileRead)?;
			Box::new(BufReader::new(file))
		} else {
			Box::new(std::io::stdin().lock())
		};

		let output: Box<dyn Write> = if let Some(output_file) = self.output {
			let file = File::create(output_file).change_context(Error::FileWrite)?;
			Box::new(BufWriter::new(file))
		} else {
			Box::new(std::io::stdout().lock())
		};

		Ok(Converter::new(input, output, direction))
	}
}
