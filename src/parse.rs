#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Command {
    Inc,
    Dec,
    Shift,
    Unshift,
    Input,
    Output,
}

#[derive(Debug)]
pub enum Program {
    Command(Command),
    Commands(Box<Vec<Program>>),
}

peg::parser! {
    grammar brainfuck() for str {
        use super::{Command, Program};
        rule ignore() = quiet!{[' ' | '\t' | '\n']}
        rule inc()      -> Command = "+" {Command::Inc}
        rule dec()      -> Command = "-" {Command::Dec}
        rule shift()    -> Command = ">" {Command::Shift}
        rule unshift()  -> Command = "<" {Command::Unshift}
        rule input()    -> Command = "," {Command::Input}
        rule output()   -> Command = "." {Command::Output}

        rule command()  -> Program = c:(inc() / dec() / shift() / unshift() / input() / output()) {
            Program::Command(c)
        }
        rule lop()      -> Program = "[" l:(command() / lop())+ "]" {
            Program::Commands(Box::from(l))
        }

        pub rule program() -> Program = p:(command() / lop())* { Program::Commands(Box::from(p)) }
    }
}

pub fn parse(input: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
    brainfuck::program(input)
}