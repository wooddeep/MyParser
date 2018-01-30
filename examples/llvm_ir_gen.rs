
extern crate parser;
extern crate env_logger;

use parser::lexer::*;
use parser::parser::*;
use parser::parser::recursive_descent::*;
use parser::parser::llvm_ir_generater::*;

fn main() {

    env_logger::init().unwrap();

    let src = "
int f(int a, int b)
{
    // if (a > 0)
        // return a;
    // else
        // b = b + 1;

    // return;
    // return 1;
    return b;
    // return 1 + 2;
    // return a + b;
}
    ";
    let mut parser = RecursiveDescentParser::new(Lexer::new(src.as_bytes()));

    println!("\n{}\n", src);

    println!("result: {:?}\n", parser.run());
    parser.dump();

    let mut generater = LLVMIRGenerater::new(parser.syntax_tree());
    generater.ir_gen();

    println!();
    generater.dump();
}