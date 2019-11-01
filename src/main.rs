mod parse;
mod code_generation;

fn main() {
    let tree = parse::parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    println!("{:?}", tree);
    // code_generation::generate(tree, &mut std::io::stdout());
}
