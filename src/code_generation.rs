use std::io::{self, Write};
use crate::parse::{Program, Command};


pub fn generate<W: Write>(prog: &Program, buf: &mut W) -> io::Result<()> {
  writeln!(buf, "MOV R0, #{}", calc_offset(prog));
  generate_code(prog, buf, 0)
}

fn calc_offset(prog: &Program) -> usize {
  match prog {
  Program::Command(Command::Shift)    => 1,
  Program::Command(Command::Unshift)  => 1,
  Program::Command(Command::Inc)      => 3,
  Program::Command(Command::Dec)      => 3,
  Program::Command(_)                 => 0,
  Program::Sequence(s) => {
      s.iter().map(calc_offset).fold(0, |acc, v| acc + v)
    }
  }
}

fn generate_code<W: Write>(prog: &Program, buf: &mut W, i: u8) -> io::Result<()> {
  match prog {
    Program::Command(c)  => write_asm_for_command(c, buf),
    Program::Sequence(s) => {
      let label = format!("label_{}", i);
      writeln!(buf, "\n{}:", label)?;
      for p in s.iter() {
        generate_code(p, buf, i+1);
        writeln!(buf, "\tCMP\tR0, #0")?;
        writeln!(buf, "\tBNEQ {}", label)?;
      }
      Ok(())
    }
  }
}

fn write_asm_for_command<W: Write>(cmd: &Command, buf: &mut W) -> io::Result<()> {
  match cmd {
    Command::Inc      => {
      writeln!(buf, "\tLDR R1, [R0]")?;
      writeln!(buf, "\tADD R1, #1")?;
      writeln!(buf, "\tSTR R1, [R0]")?;
      Ok(()) // equivalent to return Ok(());
    },
    Command::Dec      => {
      writeln!(buf, "\tLDR R1, [R0]")?;
      writeln!(buf, "\tSUB R1, #1")?;
      writeln!(buf, "\tSTR R1, [R0]");
      Ok(())
    }
    Command::Shift    => writeln!(buf, "\tADD R0, #1"),
    Command::Unshift  => writeln!(buf, "\tSUB R0, #1"),
    Command::Input    => Ok(()),
    Command::Output   => Ok(()),
  }
}