//! Conversion implementation. Executes conversion from a Reader to a Writer.

use std::{
	fmt::Debug,
	io::{Read, Write},
};

use error_stack::Report;

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
pub struct Converter {
	/// Source of input data.
	input: Box<dyn Read>,
	/// Sink of output data.
	output: Box<dyn Write>,
	/// Direction of conversion.
	direction: ConversionDirection,
}

impl Converter {
	/// Create new converter instance given input, output and conversion
	/// direction.
	#[must_use]
	pub fn new(
		input: impl Into<Box<dyn Read>>,
		output: impl Into<Box<dyn Write>>,
		direction: ConversionDirection,
	) -> Self {
		Self { input: input.into(), output: output.into(), direction }
	}

	/// Execute the conversion.
	pub fn execute(self) -> Result<(), Report<Error>> {
		todo!()
	}
}

impl Debug for Converter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Converter")
			.field("input", &"<redacted>")
			.field("output", &"<redacted>")
			.field("direction", &self.direction)
			.finish()
	}
}
