
extern crate parser;

use parser::lexer::*;
use parser::parser::*;
use parser::parser::recursive_descent::*;
use parser::parser::type_analyzer::*;

fn main() {
    let src = "
int main()
{
    for (a = 0; a != 10; a = a + 1);
    {
        if (a == 0)
            break;
    }

    return 0;
}
    ";
    let mut parser = RecursiveDescentParser::new(Lexer::new(src.as_bytes()));

    println!("\n{}\n", src);

    println!("result: {}\n", parser.run());
    parser.dump();

    let mut type_analyzer = TypeAnalyzer::new(parser.syntax_tree());
    println!();
    println!();
    type_analyzer.run();
}