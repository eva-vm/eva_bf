#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Command {
	Inc(usize),
	Dec(usize),
	Shift(usize),
	Unshift(usize),
	Loop(Vec<Command>),
	Input,
	Output,
}

pub type Program = Vec<Command>;

peg::parser! {
	grammar brainfuck() for str {
		use super::{Command, Program};
		rule ws() = quiet!{(" " / "\t" / "\n")*}
		rule inc()      -> Command = v:$("+"+) {Command::Inc(v.len())}
		rule dec()      -> Command = v:$("-"+) {Command::Dec(v.len())}
		rule shift()    -> Command = v:$(">"+) {Command::Shift(v.len())}
		rule unshift()  -> Command = v:$("<"+) {Command::Unshift(v.len())}
		rule input()    -> Command = "," {Command::Input}
		rule output()   -> Command = "." {Command::Output}

		rule lop()      -> Command = ws() "["  ws() l:(inc() / dec() / shift() / unshift() / input() / output() / lop())+ ws() "]" ws() {
			Command::Loop(l)
		}

		rule command()  -> Command = ws() c:(inc() / dec() / shift() / unshift() / input() / output() / lop()) ws() {
			c
		}

		pub rule program() -> Program = ws() p:(command())+ ws() { p }
	}
}

pub fn parse(input: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
	brainfuck::program(input)
}

#[cfg(test)]
mod tests {
	use super::parse;
	use super::Command;

	#[test]
	fn parses() {
		let input = "+++-[>-.]";
		let expected = vec![
			Command::Inc(3),
			Command::Dec(1),
			Command::Loop(vec![Command::Shift(1), Command::Dec(1), Command::Output]),
		];
		assert_eq!(Ok(expected), parse(input));
	}
}
