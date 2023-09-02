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
}
