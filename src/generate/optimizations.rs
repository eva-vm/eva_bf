use crate::parse::{Command, Program};

pub fn combine_increment_decrement(prog: Program) -> Program {
	fn opt_once(p: Program) -> (Program, usize) {
		let mut changes = 0usize;
		let mut out = vec![];
		for tuple in p.chunks(2) {
			match tuple {
				[Command::Dec(d), Command::Inc(i)] | [Command::Inc(i), Command::Dec(d)] => {
					let combined_amt = *i as isize - *d as isize;
					if combined_amt < 0 {
						out.push(Command::Dec(combined_amt.abs() as usize));
						changes += 1;
					} else if combined_amt > 0 {
						out.push(Command::Inc(combined_amt.abs() as usize));
						changes += 1;
					}
				}
				[Command::Unshift(u), Command::Shift(s)]
				| [Command::Shift(s), Command::Unshift(u)] => {
					let combined_amt = *s as isize - *u as isize;
					if combined_amt < 0 {
						out.push(Command::Unshift(combined_amt.abs() as usize));
						changes += 1;
					} else if combined_amt > 0 {
						out.push(Command::Shift(combined_amt.abs() as usize));
						changes += 1;
					}
				}
				x => {
					for v in x {
						out.push(v.clone())
					}
				}
			}
		}

		return (out, changes);
	}

	let mut changes = 1usize;
	let mut p = prog;
	while changes > 0 {
		let res = opt_once(p);
		p = res.0;
		changes = res.1;
	}

	return p;
}

#[cfg(test)]
mod tests {
	use super::combine_increment_decrement;
	use super::Command;

	#[test]
	fn optimizes_inc_dec() {
		let input = vec![Command::Inc(6), Command::Dec(3)];
		let output = vec![Command::Inc(3)];

		assert_eq!(output, combine_increment_decrement(input));
	}

	#[test]
	fn optimizes_shift() {
		let input = vec![Command::Unshift(4), Command::Shift(3)];
		let output = vec![Command::Unshift(1)];
		assert_eq!(output, combine_increment_decrement(input));
	}

	#[test]
	fn no_opts_found() {
		let input = vec![
			Command::Shift(3),
			Command::Inc(10),
			Command::Loop(vec![Command::Input, Command::Shift(1), Command::Output]),
		];
		let output = input.clone();

		assert_eq!(output, combine_increment_decrement(input));
	}
}
