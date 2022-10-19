use crate::{expr::Expr, utils};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr
}

impl BindingDef {
    pub fn parse(s: &str) -> (&str, Self) {
        let s = utils::tag("let", s);
        let (s, _) = utils::extract_whitespaces(s);

        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespaces(s);

        let s = utils::tag("=", s);
        let (s, _) = utils::extract_whitespaces(s);

        let (s, val) = Expr::parse(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::parse("let a = 1 + 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(1),
                        rhs: Number(2),
                        op: Op::Add
                    }
                }
            )
        )
    }
}