//! Conversion implementation. Executes conversion from a Reader to a Writer.

use std::{
	fmt::Debug,
	io::{Read, Write},
};

use error_stack::{report, Report, ResultExt};
use rmp_serde::config::{DefaultConfig, StructMapConfig};
use serde_json::ser::PrettyFormatter;

use crate::Error;

/// Direction of conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionDirection {
	/// Automatic detection of the input.
	Auto,
	/// Convert MsgPack to JSON.
	MsgPack2Json,
	/// Convert JSON to MsgPack.
	Json2MsgPack,
}

/// Converter instance, owning a [`Read`]er and a [`Write`]r to input and output
/// the data.
pub struct Converter<I, O> {
	/// Source of input data.
	input: I,
	/// Sink of output data.
	output: O,
	/// Direction of conversion.
	direction: ConversionDirection,
}

impl<I, O> Converter<I, O>
where
	I: Read,
	O: Write,
{
	/// Create new converter instance given input, output and conversion
	/// direction.
	#[must_use]
	pub fn new(input: I, output: O, direction: ConversionDirection) -> Self {
		Self { input, output, direction }
	}

	/// Execute the conversion.
	pub fn execute(mut self) -> Result<(), Report<Error>> {
		match self.direction {
			ConversionDirection::Auto => self.automatic_conversion()?,
			ConversionDirection::MsgPack2Json => {
				let mut deserializer = msgpack_deserializer(self.input);
				let mut serializer = json_serializer(&mut self.output);
				serde_transcode::transcode(&mut deserializer, &mut serializer)
					.change_context(Error::Transcoding)?;
			}
			ConversionDirection::Json2MsgPack => {
				let mut deserializer = json_deserializer(self.input);
				let mut serializer = msgpack_serializer(&mut self.output);
				serde_transcode::transcode(&mut deserializer, &mut serializer)
					.change_context(Error::Transcoding)?;
			}
		}
		self.output.write(&[b'\n']).change_context(Error::FileWrite)?;
		Ok(())
	}

	/// Execute automatically detected conversion. This reads the full input
	/// until the end and attempts to transcode JSON to MsgPack first, then try
	/// the other way if it did not work.
	fn automatic_conversion(&mut self) -> Result<(), Report<Error>> {
		let mut data = Vec::new();
		self.input.read_to_end(&mut data).change_context(Error::FileRead)?;
		let mut error = report!(Error::AutomaticDetection);

		// Try JSON to MsgPack first.
		let mut output = Vec::new();
		let mut deserializer = json_deserializer(data.as_slice());
		let mut serializer = msgpack_serializer(&mut output);
		let res = serde_transcode::transcode(&mut deserializer, &mut serializer)
			.change_context(Error::Transcoding);
		match res {
			Ok(_) => return self.output.write_all(&output).change_context(Error::FileWrite),
			Err(err) => error.extend_one(err),
		}
		drop(output); // Drop, we will write directly to the output, otherwise we would clear it.

		// It did not work, try MsgPack to JSON now.
		let mut deserializer = msgpack_deserializer(data.as_slice());
		let mut serializer = json_serializer(&mut self.output);
		let res = serde_transcode::transcode(&mut deserializer, &mut serializer)
			.change_context(Error::Transcoding);
		match res {
			Ok(_) => return Ok(()),
			Err(err) => error.extend_one(err),
		}

		// This didn't work either, return the error.
		Err(error)
	}
}

/// Construct a MsgPack Deserializer.
fn msgpack_deserializer<R: Read>(
	reader: R,
) -> rmp_serde::Deserializer<rmp_serde::decode::ReadReader<R>> {
	rmp_serde::Deserializer::new(reader)
}

/// Construct a MsgPack Serializer.
fn msgpack_serializer<W: Write>(
	writer: W,
) -> rmp_serde::Serializer<W, StructMapConfig<DefaultConfig>> {
	rmp_serde::Serializer::new(writer).with_struct_map()
}

/// Construct a JSON Deserializer.
fn json_deserializer<R: Read>(reader: R) -> serde_json::Deserializer<serde_json::de::IoRead<R>> {
	serde_json::Deserializer::from_reader(reader)
}

/// Construct a JSON Serializer.
fn json_serializer<W: Write>(writer: W) -> serde_json::Serializer<W, PrettyFormatter<'static>> {
	serde_json::Serializer::pretty(writer)
}

impl<I, O> Debug for Converter<I, O> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Converter")
			.field("input", &"<redacted>")
			.field("output", &"<redacted>")
			.field("direction", &self.direction)
			.finish()
	}
}
