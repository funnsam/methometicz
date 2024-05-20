#![allow(mutable_transmutes)]

use logos::Logos;

mod compiler;
mod runtime;

fn main() {
    let src = std::fs::read_to_string("test.meth").unwrap();
    let mut l = compiler::lexer::Token::lexer(&src);
    let mut p = compiler::parser::Parser::new_from_lex(&mut l);

    let p = compiler::parser::parse_ast(&mut p);
    runtime::run(&p);
}
