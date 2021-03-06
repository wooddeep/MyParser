
extern crate parser;

use parser::lexer::*;
use parser::parser::*;
use parser::parser::recursive_descent::*;

fn main() {
    let src = "a + b != c + d || !e";
    let mut parser = RecursiveDescentParser::new(SimpleLexer::new(src.as_bytes()));

    println!("\n{}\n", src);

    parser.run().ok();
    parser.dump();
}