//! Errors that happen during the execution.

use thiserror::Error;

/// CLI error.
#[derive(Debug, Error)]
pub enum Error {
	/// Multiple conversion directions specified.
	#[error("Multiple conversion directions specified")]
	MultipleConversionDirections,

	/// Error reading input file.
	#[error("Error reading input file")]
	FileRead,
	/// Error writing output file.
	#[error("Error writing output file")]
	FileWrite,

	/// Automatic conversion direction detection error.
	#[error("Failed automatic converstion direction detection")]
	AutomaticDetection,

	/// Error reading MessagePack data.
	#[error("Error reading MessagePack")]
	ReadingMsgPack,
	/// Error writing MessagePack data.
	#[error("Error writing MessagePack")]
	WritingMsgPack,

	/// Error reading JSON data.
	#[error("Error reading JSON")]
	ReadingJson,
	/// Error writing JSON data.
	#[error("Error writing JSON")]
	WritingJson,
}
