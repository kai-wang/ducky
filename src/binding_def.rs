use crate::{expr::Expr, utils};
use crate::env::Env;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub val: Expr
}

impl BindingDef {
    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("let", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let s = utils::tag("=", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, val) = Expr::parse(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                val,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_bindings(self.name.clone(), self.val.eval());
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
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(1),
                        rhs: Number(2),
                        op: Op::Add
                    }
                }
            ))
        )
    }
}