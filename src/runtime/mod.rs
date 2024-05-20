use crate::compiler::parser::*;
use std::collections::HashMap;

pub fn run<'a>(ast: &'a Ast<'a>) {
    let mut defs = HashMap::new();

    defs.insert("print", RtValue::IntLambda(&|i| {
        println!("{i:?}");
        RtValue::ChurchNumeral(0)
    }));

    let mut int = Interpreter {
        ast,
        defs,
    };

    int.run();
}

struct Interpreter<'a> {
    ast: &'a Ast<'a>,
    defs: HashMap<&'a str, RtValue<'a>>,
}

#[derive(Clone)]
enum RtValue<'a> {
    ChurchNumeral(u64),
    LambdaFn {
        state: HashMap<&'a str, RtValue<'a>>,
        arguments: &'a [&'a str],
        body: &'a Expr<'a>,
    },
    IntLambda(&'a dyn Fn(Vec<RtValue<'a>>) -> RtValue<'a>),
}

impl<'a> Interpreter<'a> {
    fn run(&mut self) {
        for n in self.ast.iter() {
            self.run_node(&n.kind);
        }
    }

    fn run_node(&mut self, n: &'a NodeKind) {
        match n {
            NodeKind::VariableAssign { var, expr } => {
                let val = self.run_expr(&expr.kind);
                self.defs.insert(var, val);
            },
            NodeKind::Expr(expr) => {
                self.run_expr(expr);
            },
        }
    }

    fn run_expr(&mut self, expr: &'a ExprKind) -> RtValue<'a> {
        match expr {
            ExprKind::Ident(id) => self.defs.get(id).unwrap_or_else(|| panic!("Unknown variable {id}")).clone(),
            ExprKind::Number(n) => RtValue::ChurchNumeral(*n),
            ExprKind::Lambda(args, body) => RtValue::LambdaFn { state: self.defs.clone(), arguments: args, body },
            ExprKind::Call(id, args) => {
                let id = self.run_expr(&id.kind);
                match id {
                    RtValue::LambdaFn { state, arguments, body } => {
                        assert_eq!(args.len(), arguments.len());

                        let ps = self.defs.clone();
                        let mut ns = state.clone();

                        for (n, v) in arguments.iter().zip(args) {
                            let v = self.run_expr(&v.kind);
                            ns.insert(n, v);
                        }

                        self.defs = ns;
                        let ret = self.run_expr(&body.kind);
                        self.defs = ps;
                        ret
                    },
                    RtValue::ChurchNumeral(n) => {
                        // λf. λx. f^on x
                        todo!();
                    },
                    RtValue::IntLambda(f) => f(args.iter().map(|a| self.run_expr(&a.kind)).collect()),
                }
            },
        }
    }
}

use core::fmt;

impl fmt::Debug for RtValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChurchNumeral(n) => write!(f, "{n}"),
            Self::LambdaFn { .. } => write!(f, "<lambda>"),
            Self::IntLambda(_) => write!(f, "<interpreter builtin>"),
        }
    }
}
