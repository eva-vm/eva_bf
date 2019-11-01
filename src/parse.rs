#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Command {
	Inc(usize),
	Dec(usize),
	Shift(usize),
	Unshift(usize),
	Input,
	Output,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Program {
	Command(Command),
	Sequence(Box<Vec<Program>>),
	Program(Box<Vec<Program>>),
}

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

		rule command()  -> Program = ws() c:(inc() / dec() / shift() / unshift() / input() / output()) ws() {
			Program::Command(c)
		}
		rule lop()      -> Program = ws() "["  ws() l:(command() / lop())+ ws() "]" ws() {
			Program::Sequence(Box::from(l))
		}

		pub rule program() -> Program = ws() p:(command() / lop())+ ws() { Program::Program(Box::from(p)) }
	}
}

pub fn parse(input: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
	brainfuck::program(input)
}

#[cfg(test)]
mod tests {
	use super::parse;
	use super::{Command, Program};

	#[test]
	fn parses() {
		let input = "+++-[>-.]";
		let expected = Program::Program(Box::new(vec![
			Program::Command(Command::Inc(3)),
			Program::Command(Command::Dec(1)),
			Program::Sequence(Box::new(vec![
				Program::Command(Command::Shift(1)),
				Program::Command(Command::Dec(1)),
				Program::Command(Command::Output),
			])),
		]));
		assert_eq!(Ok(expected), parse(input));
	}
}
