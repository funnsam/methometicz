#![allow(mutable_transmutes)]

use logos::Logos;

mod compiler;

fn main() {
    let src = std::fs::read_to_string("test.meth").unwrap();
    let mut l = compiler::lexer::Token::lexer(&src);
    let mut p = compiler::parser::Parser::new_from_lex(&mut l);

    println!("{:#?}", compiler::parser::parse_ast(&mut p));
}
