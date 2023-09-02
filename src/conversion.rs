//! Conversion implementation. Executes conversion from a Reader to a Writer.

use std::{
	fmt::Debug,
	io::{Cursor, Read, Write},
};

use error_stack::{report, Report, ResultExt};

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
				let value = rmpv::decode::read_value(&mut self.input)
					.change_context(Error::ReadingMsgPack)?;
				serde_json::to_writer_pretty(self.output, &value)
					.change_context(Error::WritingJson)?;
			}
			ConversionDirection::Json2MsgPack => {
				let value: serde_json::Value =
					serde_json::from_reader(self.input).change_context(Error::ReadingJson)?;
				rmp_serde::encode::write_named(&mut self.output, &value)
					.change_context(Error::WritingMsgPack)?;
			}
		}
		Ok(())
	}

	/// Execute automatically detected conversion. This reads the full input
	/// until the end and attempts to deserialize with both deserializers.
	fn automatic_conversion(mut self) -> Result<(), Report<Error>> {
		let mut data = Vec::new();
		self.input.read_to_end(&mut data).change_context(Error::FileRead)?;

		let res_json =
			serde_json::from_slice::<serde_json::Value>(&data).change_context(Error::ReadingJson);

		let mut cursor = Cursor::new(data);
		let res_msgpack =
			rmpv::decode::read_value(&mut cursor).change_context(Error::ReadingMsgPack);
		drop(cursor);

		match (res_msgpack, res_json) {
			(Ok(_), Ok(_)) => return Err(report!(Error::AutomaticDetection)),
			(Err(err1), Err(err2)) => {
				let mut error = report!(Error::AutomaticDetection);
				error.extend_one(err1);
				error.extend_one(err2);
				return Err(error);
			}
			(Ok(msgpack), _) => serde_json::to_writer_pretty(self.output, &msgpack)
				.change_context(Error::WritingJson)?,
			(_, Ok(json)) => rmp_serde::encode::write_named(&mut self.output, &json)
				.change_context(Error::WritingMsgPack)?,
		}

		Ok(())
	}
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
