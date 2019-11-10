use std::io::Write;

mod generate;
mod parse;

pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub fn parse(input: &str) -> Result<parse::Program, peg::error::ParseError<peg::str::LineCol>> {
	parse::parse(input)
}

pub fn generate<W: Write>(
	input: parse::Program,
	output: &mut W,
	quiet: bool,
) -> std::io::Result<usize> {
	generate::generate(input, output, quiet)
}
