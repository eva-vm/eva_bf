#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Command {
	Inc(usize),
	Dec(usize),
	Shift(usize),
	Unshift(usize),
	Input,
	Output,
}

#[derive(Clone, Debug)]
pub enum Program {
	Command(Command),
	Sequence(Box<Vec<Program>>),
	Program(Box<Vec<Program>>),
}

peg::parser! {
	grammar brainfuck() for str {
		use super::{Command, Program};
		rule ignore() = quiet!{[' ' | '\t' | '\n']+}
		rule inc()      -> Command = v:$("+"+) {Command::Inc(v.len())}
		rule dec()      -> Command = v:$("-"+) {Command::Dec(v.len())}
		rule shift()    -> Command = v:$(">"+) {Command::Shift(v.len())}
		rule unshift()  -> Command = v:$("<"+) {Command::Unshift(v.len())}
		rule input()    -> Command = "," {Command::Input}
		rule output()   -> Command = "." {Command::Output}

		rule command()  -> Program = c:(inc() / dec() / shift() / unshift() / input() / output()) {
			Program::Command(c)
		}
		rule lop()      -> Program = "[" l:(command() / lop())+ "]" {
			Program::Sequence(Box::from(l))
		}

		pub rule program() -> Program = p:(command() / lop())* { Program::Program(Box::from(p)) }
	}
}

pub fn parse(input: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
	brainfuck::program(input)
}
