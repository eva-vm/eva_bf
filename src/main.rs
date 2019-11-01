use std::io::Read;

use clap::{App, Arg};

use crate::utils::{Input, Output};

mod generate;
mod parse;
mod utils;

pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
	// let parsing_result = parse::parse("++[-]");
	/*let parsing_result = parse::parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
	match parsing_result {
		Ok(p) => {
			eprintln!("{:?}", p);
			match code_generation::generate(&p, &mut std::io::stdout()) {
			  Ok(_) => eprintln!("✅ Done."),
			  Err(_) => eprintln!("❌ Ouput Error while generating the code."),
			}
		},
		Err(err) => eprintln!("Parsing error at {}", err),
	}*/
	let matches = App::new("Eva Brainfuck compiler")
		.version(VERSION.unwrap_or("unstable"))
		.author("Nathan G. <solarliner@gmail.com>, Arthur C. <arthur.correnson@gmail.com>")
		.about("Compiles Brainfuck to Eva ASM")
		.arg(
			Arg::with_name("OUTPUT")
				.short("o")
				.long("output")
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
		.get_matches();
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
					parse::parse(&inbuf)
						.map_err(|err| format!("Couldn't parse input: {}", err))
						.and_then(|ast| {
							generate::generate(&ast, &mut output)
								.map_err(|err| format!("Couldn't generate assembly: {}", err))
						})
				});
			if let Err(e) = res {
				eprintln!("ERROR: {}", e);
			}
		}
	}
}
