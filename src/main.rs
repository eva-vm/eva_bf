mod parse;
mod code_generation;

fn main() {
    // let parsing_result = parse::parse("++[-]");
    let parsing_result = parse::parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    match parsing_result {
        Ok(p) => {
            eprintln!("{:?}", p);
            match code_generation::generate(&p, &mut std::io::stdout()) {
              Ok(_) => eprintln!("✅ Done."),
              Err(_) => eprintln!("❌ Ouput Error while generating the code."),
            }
        },
        Err(err) => eprintln!("Parsing error at {}", err),
    }
}
