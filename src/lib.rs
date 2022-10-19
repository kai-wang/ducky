
pub mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(i32);

impl Number {
    pub fn parse(s: &str) -> (&str, Self) {
        let(remaining, s) = utils::extract_digits(s);
        (remaining, Self(s.parse().unwrap()))
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
    pub fn parse(s: &str) -> (&str, Self) {
        let (remaining, s) = utils::extract_op(s);

        let op = match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };

        (remaining, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op
}

impl Expr {
    pub fn parse(s: &str) -> (&str, Self) {
        
        let (s, lhs) = Number::parse(s);
        let (s, op) = Op::parse(s);
        let (s, rhs) = Number::parse(s);

        (s, Self { lhs, rhs, op })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::parse("123"), ("", Number(123)));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Op::parse("+"), ("", Op::Add));
    }

    #[test]
    fn parse_sub() {
        assert_eq!(Op::parse("-"), ("", Op::Sub));
    }

    #[test]
    fn parse_mul() {
        assert_eq!(Op::parse("*"), ("", Op::Mul));
    }

    #[test]
    fn parse_div() {
        assert_eq!(Op::parse("/"), ("", Op::Div));
    }

    #[test]
    fn parse_add_expr() {
        assert_eq!(
            Expr::parse("1+2"),
            (
                "", 
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add
                }
            )
        );
    }
}
