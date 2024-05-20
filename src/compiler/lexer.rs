pub use logos::*;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"\s")]
#[logos(skip r"<\|[^\n]*")]
#[logos(skip r"\|>[^\n]*")]
pub enum Token {
    #[token("λ")]
    Lambda,
    #[token("def")]
    Store,
    #[token(".")]
    Dot,

    #[regex(r"[a-zA-Z_\+\-\*\/]+")]
    Ident,
    #[regex(r"\d+", callback = |lex| lex.slice().parse::<u64>().unwrap())]
    Number(u64),

    #[token("(")]
    AppliStart,
    #[token(")")]
    AppliEnd,
}
