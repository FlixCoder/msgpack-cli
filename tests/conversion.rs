use std::f64::consts::PI;

use error_stack::{Report, ResultExt};
use msgpack_cli::{ConversionDirection, Converter};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Root {
	a: Vec<SubPath>,
	b: Option<f32>,
	c: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SubPath {
	b: bool,
	floats: Vec<f64>,
	string: String,
	ints: Vec<i64>, // i128 is not supported :/
}

impl Root {
	fn test() -> Self {
		Self {
			a: vec![
				SubPath {
					b: true,
					floats: vec![1.0, PI, 3.57, 0.0],
					string: "\"\"™".to_owned(),
					ints: vec![0, 12, 99_999_999_999_999_999],
				},
				SubPath { b: false, floats: vec![], string: "".to_owned(), ints: vec![] },
			],
			b: Some(1.2345),
			c: None,
		}
	}
}

#[derive(Debug, Error)]
#[error("Test failed")]
struct Error;

#[test]
fn automatic() -> Result<(), Report<Error>> {
	let data = Root::test();

	let json = serde_json::to_vec(&data).change_context(Error)?;
	let msgpack = rmp_serde::to_vec_named(&data).change_context(Error)?;

	let mut output_msgpack = Vec::new();
	Converter::new(json.as_slice(), &mut output_msgpack, ConversionDirection::Auto)
		.execute()
		.change_context(Error)?;
	let mut output_json = Vec::new();
	Converter::new(msgpack.as_slice(), &mut output_json, ConversionDirection::Auto)
		.execute()
		.change_context(Error)?;

	let data_json2msgpack: Root = rmp_serde::from_slice(&output_msgpack).change_context(Error)?;
	let data_msgpack2json: Root = serde_json::from_slice(&output_json).change_context(Error)?;
	assert_eq!(data_json2msgpack, data);
	assert_eq!(data_msgpack2json, data);

	Ok(())
}

#[test]
fn json() -> Result<(), Report<Error>> {
	let data = Root::test();
	let json = serde_json::to_vec(&data).change_context(Error)?;

	let mut output_msgpack = Vec::new();
	Converter::new(json.as_slice(), &mut output_msgpack, ConversionDirection::Json2MsgPack)
		.execute()
		.change_context(Error)?;
	let output_data: Root = rmp_serde::from_slice(&output_msgpack).change_context(Error)?;
	assert_eq!(output_data, data);

	let mut output_json = Vec::new();
	Converter::new(output_msgpack.as_slice(), &mut output_json, ConversionDirection::MsgPack2Json)
		.execute()
		.change_context(Error)?;
	let output_data: Root = serde_json::from_slice(&output_json).change_context(Error)?;
	assert_eq!(output_data, data);

	Ok(())
}

#[test]
fn msgpack() -> Result<(), Report<Error>> {
	let data = Root::test();
	let msgpack = rmp_serde::to_vec_named(&data).change_context(Error)?;

	let mut output_json = Vec::new();
	Converter::new(msgpack.as_slice(), &mut output_json, ConversionDirection::MsgPack2Json)
		.execute()
		.change_context(Error)?;
	let output_data: Root = serde_json::from_slice(&output_json).change_context(Error)?;
	assert_eq!(output_data, data);

	let mut output_msgpack = Vec::new();
	Converter::new(output_json.as_slice(), &mut output_msgpack, ConversionDirection::Json2MsgPack)
		.execute()
		.change_context(Error)?;
	let output_data: Root = rmp_serde::from_slice(&output_msgpack).change_context(Error)?;
	assert_eq!(output_data, data);

	Ok(())
}
