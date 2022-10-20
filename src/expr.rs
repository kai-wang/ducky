pub mod binding_usage;
pub mod block;

use crate::{utils, val::Val};
use binding_usage::BindingUsage;
use block::Block;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        let (remaining, s) = utils::extract_digits(s)?;
        Ok((remaining, Self(s.parse().unwrap())))
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div
}

impl Op {
    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        utils::tag("+", s)
            .map(|s| (s, Self::Add))
            .or_else(|_| utils::tag("-", s).map(|s| (s, Self::Sub)))
            .or_else(|_| utils::tag("*", s).map(|s| (s, Self::Mul)))
            .or_else(|_| utils::tag("/", s).map(|s| (s, Self::Div)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Number),
    Operation { lhs: Number, rhs: Number, op: Op },
    BindingUsage(BindingUsage),
    Block(Block)
}

impl Expr {
    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::parse(s).map(|(s, number)| (s, Self::Number(number)))
    }

    fn new_operations(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = Number::parse(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, op) = Op::parse(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, rhs) = Number::parse(s)?;

        Ok((s, Self::Operation { lhs, rhs, op }))
    }

    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        Self::new_operations(s)
            .or_else(|_| Self::new_number(s))
            .or_else(|_| {
                BindingUsage::parse(s)
                    .map(|(s, binding_usage)| (s, Self::BindingUsage(binding_usage)))
                })
            .or_else(|_| Block::parse(s).map(|(s, block)| (s, Self::Block(block))))
    }

    pub(crate) fn eval(&self) -> Val {
        match self {
            Self::Number(Number(n)) => Val::Number(*n),
            Self::Operation { lhs, rhs, op } => {
                let Number(lhs) = lhs;
                let Number(rhs) = rhs;

                let result = match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs
                };

                Val::Number(result)
            }
            _ => todo!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::stmt::Stmt;

    #[test]
    fn parse_number() {
        assert_eq!(Number::parse("123"), Ok(("", Number(123))));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Op::parse("+"), Ok(("", Op::Add)));
    }

    #[test]
    fn parse_sub() {
        assert_eq!(Op::parse("-"), Ok(("", Op::Sub)));
    }

    #[test]
    fn parse_mul() {
        assert_eq!(Op::parse("*"), Ok(("", Op::Mul)));
    }

    #[test]
    fn parse_div() {
        assert_eq!(Op::parse("/"), Ok(("", Op::Div)));
    }

    #[test]
    fn parse_nubmer_as_expr() {
        assert_eq!(Expr::parse("456"), Ok(("", Expr::Number(Number(456)))));
    }

    #[test]
    fn parse_add_expr() {
        assert_eq!(
            Expr::parse("1+2"),
            Ok((
                "", 
                Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add
                }
            ))
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(10),
                rhs: Number(3),
                op: Op::Add
            }.eval(),
            Val::Number(13)
        )
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(1),
                rhs: Number(5),
                op: Op::Sub,
            }
            .eval(),
            Val::Number(-4),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(5),
                rhs: Number(6),
                op: Op::Mul,
            }
            .eval(),
            Val::Number(30),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Number(200),
                rhs: Number(20),
                op: Op::Div,
            }
            .eval(),
            Val::Number(10),
        );
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::parse("bar"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "bar".to_string()
                })
            ))
        )
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::parse("{ 200 }"),
            Ok((
                "",
                Expr::Block(Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(200)))],
                }),
            )),
        );
    }
}
