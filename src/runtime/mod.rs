use crate::compiler::parser::*;
use std::collections::HashMap;

pub fn run<'a>(ast: &'a Ast<'a>) {
    let mut defs = HashMap::new();

    defs.insert("print_dbg", RtValue::IntLambda(&|i| {
        println!("{i:?}");
        RtValue::Numeral(0)
    }));

    defs.insert("print_char", RtValue::IntLambda(&|i| {
        for i in i.iter() {
            match i {
                RtValue::Numeral(n) | RtValue::ChurchNumeral { f: _, n } => print!(
                    "{}",
                    unsafe { char::from_u32_unchecked(*n as u32) }
                ),
                c => println!("<char {c:?}>"),
            }
        }

        RtValue::Numeral(0)
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
    Numeral(u64),
    ChurchNumeral {
        f: Box<RtValue<'a>>,
        n: u64,
    },
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
            ExprKind::Number(n) => RtValue::Numeral(*n),
            ExprKind::Lambda(args, body) => RtValue::LambdaFn { state: self.defs.clone(), arguments: args, body },
            ExprKind::Call(id, args) => {
                let id = self.run_expr(&id.kind);
                let args = args.iter().map(|e| self.run_expr(&e.kind)).collect();
                self.call_fn(&id, args)
            },
        }
    }

    fn call_fn(&mut self, id: &RtValue<'a>, mut args: Vec<RtValue<'a>>) -> RtValue<'a> {
        match id {
            RtValue::LambdaFn { state, arguments, body } => {
                assert_eq!(arguments.len(), args.len());

                let ps = self.defs.clone();
                let mut ns = state.clone();

                for (n, v) in arguments.iter().zip(args) {
                    ns.insert(n, v);
                }

                self.defs = ns;
                let ret = self.run_expr(&body.kind);
                self.defs = ps;
                ret
            },
            RtValue::Numeral(n) => {
                // LIGHT: λf. λx. f^on x
                // eg:
                //     0 = λf. λx. x
                //     1 = λf. λx. (f x)
                //     2 = λf. λx. (f (f x))
                // this do the `λf` part

                assert_eq!(args.len(), 1);
                RtValue::ChurchNumeral { f: Box::new(args.swap_remove(0)), n: *n }
            },
            RtValue::ChurchNumeral { f, n } => {
                assert_eq!(args.len(), 1);
                let mut acc = args.swap_remove(0);

                for _ in 0..*n {
                    acc = self.call_fn(f, vec![acc]);
                }

                acc
            },
            RtValue::IntLambda(f) => f(args),
        }
    }
}

use core::fmt;

impl fmt::Debug for RtValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeral(n) => write!(f, "{n}"),
            Self::ChurchNumeral { f: i, n } => write!(f, "{i:?} * {n}"),
            Self::LambdaFn { .. } => write!(f, "<lambda>"),
            Self::IntLambda(_) => write!(f, "<interpreter builtin>"),
        }
    }
}
