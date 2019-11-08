use clap::{App, Arg};
use evabf::{generate, parse, VERSION};
use std::io::{Cursor, Read, Seek};
use wbuf::{Input, Output};

fn main() {
	let matches = App::new("Eva Brainfuck compiler")
		.version(VERSION.unwrap_or("unstable"))
		.author("Nathan G. <solarliner@gmail.com>, Arthur C. <arthur.correnson@gmail.com>")
		.about("Compiles Brainfuck to Eva ASM")
		.arg(
			Arg::with_name("OUTPUT")
				.short("o")
				.long("output")
				.takes_value(true)
				.help("Output compiled/asm program"),
		)
		.arg(
			Arg::with_name("INPUT")
				.index(1)
				.help("Input brainfuck program"),
		)
		.arg(
			Arg::with_name("v")
				.short("v")
				.multiple(true)
				.help("Sets the level of verbosity"),
		)
		.arg(
			Arg::with_name("quiet")
				.short("q")
				.long("quiet")
				.help("Don't include comments in generated assembly"),
		)
		.arg(Arg::with_name("asm").short("a").long("asm"))
		.get_matches();
	if matches.is_present("asm") {
		let input_file = Input::from_arg(matches.value_of("INPUT"));
		let output_file = Output::from_arg(matches.value_of("OUTPUT"));

		match (input_file, output_file) {
			(Err(e), _) | (_, Err(e)) => eprintln!("{}", e),
			(Ok(mut input), Ok(mut output)) => {
				let mut inbuf = String::new();
				let res = input
					.read_to_string(&mut inbuf)
					.map_err(|err| format!("Couldn't read input: {}", err))
					.and_then(|_| {
						parse(&inbuf).map_err(|err| format!("Couldn't parse input: {}", err))
					})
					.and_then(|ast| {
						generate(&ast, &mut output, matches.is_present("quiet"))
							.map_err(|err| format!("Couldn't generate assembly: {}", err))
					});
				match res {
					Ok(_) => eprintln!("✔ Done."),
					Err(e) => eprintln!("❌ Assembly error: {}", e),
				}
			}
		}
	} else {
		let input_file = Input::from_arg(matches.value_of("INPUT"));
		let mut temp_asm: Cursor<Vec<u8>> = Cursor::new(Vec::new());
		let output_file = Output::from_arg(matches.value_of("OUTPUT"));

		match (input_file, output_file) {
			(Err(e), _) | (_, Err(e)) => eprintln!("{}", e),
			(Ok(mut input), Ok(mut output)) => {
				let mut input_buffer = String::new();
				let res = input
					.read_to_string(&mut input_buffer)
					.map_err(|err| format!("Couldn't read input: {}", err))
					.and_then(|_| {
						evabf::parse(&input_buffer)
							.map_err(|err| format!("Couldn't parse input: {}", err))
					})
					.and_then(|ast| {
						evabf::generate(&ast, &mut temp_asm, true)
							.map_err(|err| format!("Couldn't generate assembly: {}", err))
					})
					.and_then(|_| {
						input_buffer.clear();
						temp_asm
							.seek(std::io::SeekFrom::Start(0))
							.map_err(|e| format!("Couldn't seek back to beginning: {}", e))?;
						temp_asm
							.read_to_string(&mut input_buffer)
							.map_err(|e| format!("ASM read error: {}", e))
					})
					.and_then(|_| {
						println!("Debug: temp_asm\n{}", input_buffer);
						evasm::assemble(&input_buffer, &mut output)
							.map_err(|e| format!("Couldn't assemble: {}", e))
					});
				match res {
					Err(e) => eprintln!("❌ Generation error: {}", e),
					Ok(_) => eprintln!("✔ Done."),
				}
			}
		}
	}
}
