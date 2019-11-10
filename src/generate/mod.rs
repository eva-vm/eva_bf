use crate::parse::{Command, Program};
use std::io::{self, Write};

mod optimizations;

pub fn generate<W: Write>(prog: Program, buf: &mut W, quiet: bool) -> io::Result<usize> {
	if !quiet {
		writeln!(
			buf,
			" ;----------------------------------------------------\n \
			 ;- This code has been generated with eva_bf {:8}-    \n \
			 ;-                                                  -\n \
			 ;- Authors:                                         -\n \
			 ;-                                                  -\n \
			 ;- Nathan Graule <solarliner@gmail.com>             -\n \
			 ;- Arthur Correnson <arthur.correnson@gmail.com>    -\n \
			 ;----------------------------------------------------\n",
			crate::VERSION.unwrap_or("unstable")
		)?;
	}
	// let offset = calc_offset(prog);
	let offset = 15000; // Constant offset in memory
	if !quiet {
		writeln!(buf, "; Data pointer initialisation\n")?;
	}
	writeln!(buf, "\tMOV\tR0, #{}", offset)?;
	if !quiet {
		writeln!(buf, "\n; Main program")?;
	}
	generate_code(optimizations::combine_increment_decrement(prog), buf)
}

#[allow(dead_code)]
fn calc_offset(prog: &Program) -> usize {
	#[rustfmt::skip]
	fn step(com: &Command) -> usize {
		match &*com {
			Command::Shift(_) 	=> 1,
			Command::Unshift(_) => 1,
			Command::Inc(_) 	=> 3,
			Command::Dec(_) 	=> 3,
			Command::Loop(seq) 	=> 8 + calc_offset(seq),
			Command::Input 		=> 2,
			Command::Output 	=> 2,
		}
	}
	prog.iter().map(step).fold(0, |acc, v| acc + v)
}

fn generate_code<W: Write>(prog: Program, buf: &mut W) -> io::Result<usize> {
	let mut nlabels = 0usize;
	for c in prog {
		nlabels += write_asm_for_command(c, buf, nlabels)?;
	}
	Ok(nlabels)
}

fn write_asm_for_command<W: Write>(cmd: Command, buf: &mut W, i: usize) -> io::Result<usize> {
	match cmd {
		Command::Inc(n) => {
			writeln!(buf, "\tLDR\tR1, [R0]")?;
			writeln!(buf, "\tADD\tR1, #{}", n)?;
			writeln!(buf, "\tSTR\tR1, [R0]")?;
			Ok(0)
		}
		Command::Dec(n) => {
			writeln!(buf, "\tLDR\tR1, [R0]")?;
			writeln!(buf, "\tSUB\tR1, #{}", n)?;
			writeln!(buf, "\tSTR\tR1, [R0]")?;
			Ok(0)
		}
		Command::Shift(n) => {
			writeln!(buf, "\tADD\tR0, #{}", n)?;
			Ok(0)
		}
		Command::Unshift(n) => {
			writeln!(buf, "\tSUB\tR0, #{}", n)?;
			Ok(0)
		}
		Command::Input => {
			writeln!(buf, "\tIN\tR1")?;
			writeln!(buf, "\tSTR\tR1, [R0]")?;
			Ok(0)
		}
		Command::Output => {
			writeln!(buf, "\tLDR\tR1, [R0]")?;
			writeln!(buf, "\tOUT\tR1")?;
			Ok(0)
		}
		Command::Loop(seq) => {
			let label = format!("label_{}", i);
			let label_out = format!("label_{}_out", i);
			let mut nlabels = 1usize;
			writeln!(buf, "\tLDR\tR1, [R0]")?;
			writeln!(buf, "\tCMP\tR1, #0")?;
			writeln!(buf, "\tMOV\tR1, {}", label_out)?;
			writeln!(buf, "\tBEQ\tR1")?;
			writeln!(buf, "\n{}:", label)?;
			for p in seq {
				nlabels += write_asm_for_command(p, buf, i + nlabels)?;
			}
			writeln!(buf, "\tLDR\tR1, [R0]")?;
			writeln!(buf, "\tCMP\tR1, #0")?;
			writeln!(buf, "\tMOV\tR1, {}", label)?;
			writeln!(buf, "\tBNEQ\tR1")?;
			writeln!(buf, "\n{}:", label_out)?;
			Ok(nlabels)
		}
	}
}
