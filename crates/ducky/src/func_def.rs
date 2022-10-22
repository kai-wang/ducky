use crate::{stmt::Stmt, utils, env::Env};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Stmt>,
}

impl FuncDef {
    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;
        let (s, _) = utils::extract_whitespaces(s);
        
        let (s, name) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(s, ident)| (s, ident.to_string())),
            utils::extract_whitespaces,
            s,
        )?;

        let s = utils::tag("=>", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let (s, body) = Stmt::parse(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
                params: params,
                body: Box::new(body)
            }
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_func(self.name.clone(), self.params.clone(), *self.body.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Expr, Number, Op, BindingUsage, Block};

    #[test]
    fn parse_func_def_with_multiple_params() {
        assert_eq!(
            FuncDef::parse("fn add x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_string()
                        })),
                        op: Op::Add
                    }))
                }
            ))
        );
    }
}