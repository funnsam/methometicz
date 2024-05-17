pub use logos::*;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"\s")]
#[logos(skip r"<|[^\n]*")]
pub enum Token {
    #[token("λ")]
    Lambda,
    #[token("assi")]
    Store,
    #[token(".")]
    Dot,

    #[regex(r"[a-zA-Z_]+")]
    Ident,
    #[regex(r"\d+(\.\d+)?", callback = |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[token("+")]
    Plus,

    #[token("(")]
    AppliStart,
    #[token(")")]
    AppliEnd,
}
