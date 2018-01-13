
extern crate parser;
extern crate env_logger;

use parser::lexer::*;
use parser::parser::*;
use parser::parser::recursive_descent::*;
use parser::parser::syntax_node::*;
use parser::parser::llvm_ir_generater::*;

fn main() {

    env_logger::init().unwrap();

    let src = "
int f()
{
    return 0;
}
    ";
    let mut parser = RecursiveDescentParser::new(Lexer::new(src.as_bytes()));

    println!("\n{}\n", src);

    println!("result: {:?}\n", parser.run());
    parser.dump();

    let syntax = SyntaxType::Expr;
    syntax.generate();
}