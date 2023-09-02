# MessagePack CLI

Small and simple CLI tool to convert MessagePack data to JSON and vice versa.

## Example

```bash
# Convert to MsgPack and back to JSON using automatic encoding detection
$ echo '{ "a": 5 }' | mpc | mpc
{
  "a": 5
}
$ echo '{ "a": 5 }' > test.json
$ mpc -i test.json -o test.msgpack
# Explicitly specify the direction of conversion
$ mpc --m2j -i test.msgpack
{
  "a": 5
}
```

## Installation

```bash
cargo install messagepack-cli
# or
cargo install --git https://github.com/FlixCoder/msgpack-cli
```

## Usage

```text
Simple CLI to convert MessagePack to JSON and vice versa. Automatically attempts to detect the input format and outputs the respective other format. Use the config options to override the automatic detection

Usage: mpc [OPTIONS]

Options:
      --m2j              Convert MsgPack to JSON
      --j2m              Convert JSON to MsgPack
  -i, --input <INPUT>    Input file path to use. Stdin is used if not given
  -o, --output <OUTPUT>  Output file path to use. Stdout is used if not given
  -h, --help             Print help
  -V, --version          Print version
```

## Lints

This projects uses a bunch of clippy lints for higher code quality and style.

Install [`cargo-lints`](https://github.com/soramitsu/iroha2-cargo_lints) using `cargo install --git https://github.com/FlixCoder/cargo-lints`. The lints are defined in `lints.toml` and can be checked by running `cargo lints clippy --all-targets --workspace`.
