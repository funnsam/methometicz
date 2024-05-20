use super::lexer::*;

pub fn parse_ast<'a>(ps: &'a mut Parser<'a>) -> Ast<'a> {
    let mut ast = Ast::new();

    let ps = &*ps;

    while let Some(n) = unsafe { core::mem::transmute::<&'a Parser<'a>, &'a mut Parser<'a>>(ps) }.parse_node() {
        ast.push(n);
    }

    ast
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub lex: &'a mut Lexer<'a, Token>,
    pub peeked: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new_from_lex(lex: &'a mut Lexer<'a, Token>) -> Self {
        Self {
            lex,
            peeked: None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        if self.peeked.is_some() {
            return core::mem::take(&mut self.peeked);
        }

        self.lex.next().map(|a| a.map_or_else(|()| panic!("Lexer error at {:?}!", self.lex.span()), |a| a))
    }

    fn peek(&mut self) -> Option<Token> {
        if let Some(peeked) = &self.peeked {
            return Some(peeked.clone());
        }

        let peeks = self.next();
        self.peeked = peeks.clone();
        peeks
    }

    fn parse_node(&mut self) -> Option<Node> {
        match self.peek()? {
            Token::Store => {
                self.next();
                let id = self.parse_ident();
                let id = self.must(id);
                let span_start = self.lex.span();
                let expr = self.parse_expr();
                let expr = self.must(expr);
                let span_end = expr.span.end;

                Some(Node { kind: NodeKind::VariableAssign { var: id, expr }, span: span_start.start..span_end })
            },
            _ => self.parse_expr().map(|a| Node { kind: NodeKind::Expr(a.kind), span: a.span }),
        }
    }

    fn parse_ident(&mut self) -> Option<&'a str> {
        match self.next()? {
            Token::Ident => Some(self.lex.slice()),
            _ => panic!("Expected ident at {:?}!", self.lex.span()),
        }
    }

    // fn assert(&mut self, f: Token, e: Token) {
    //     if f != e {
    //         panic!("Expected {e:?}, found {f:?} somewhere!");
    //     }
    // }

    fn parse_expr(&mut self) -> Option<Expr<'a>> {
        match self.next()? {
            Token::Ident => Some(Expr { kind: ExprKind::Ident(self.lex.slice()), span: self.lex.span() }),
            Token::Number(n) => Some(Expr { kind: ExprKind::Number(n), span: self.lex.span() }),
            Token::Lambda => {
                let start = self.lex.span().start;

                let mut ids = Vec::new();
                loop {
                    let id_f = self.peek();
                    let id_f = self.must(id_f);
                    if matches!(id_f, Token::Dot) {
                        self.next();
                        break;
                    }

                    let id = self.parse_ident();
                    let id = self.must(id);
                    ids.push(id);
                }

                let bd = self.parse_expr();
                let bd = self.must(bd);
                let end = bd.span.end;

                Some(Expr { kind: ExprKind::Lambda(ids, Box::new(bd)), span: start..end })
            },
            Token::AppliStart => {
                let start = self.lex.span().start;

                let f = self.parse_expr();
                let f = self.must(f);

                let mut ops = Vec::new();
                loop {
                    let op_f = self.peek();
                    let op_f = self.must(op_f);
                    if matches!(op_f, Token::AppliEnd) {
                        self.next();
                        break;
                    }

                    let op = self.parse_expr();
                    let op = self.must(op);
                    ops.push(op);
                }

                let end = self.lex.span().end;
                Some(Expr { kind: ExprKind::Call(Box::new(f), ops), span: start..end })
            },
            _ => todo!(),
        }
    }

    fn must<T>(&self, opt: Option<T>) -> T {
        opt.map_or_else(|| panic!("Expected more at {:?}!", self.lex.span()), |a| a)
    }
}

pub type Ast<'a> = Vec<Node<'a>>;

#[derive(Debug)]
pub struct Node<'a> {
    pub kind: NodeKind<'a>,
    pub span: Span,
}

#[derive(Debug)]
pub enum NodeKind<'a> {
    VariableAssign {
        var: &'a str,
        expr: Expr<'a>,
    },
    Expr(ExprKind<'a>),
}

#[derive(Debug)]
pub struct Expr<'a> {
    pub kind: ExprKind<'a>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind<'a> {
    Ident(&'a str),
    Number(u64),
    Lambda(Vec<&'a str>, Box<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
}
