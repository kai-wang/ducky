use super::Expr;
use crate::{utils, Env, Val};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn parse(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let (s, _) = utils::take_while2(|c| c == ' ', s);

        let (s, params) = utils::sequence1(Expr::parse, |s| utils::take_while2(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();

        let (param_names, body) = env.get_func(&self.callee).unwrap();
        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.store_bindings(param_name, param_val);
        }

        body.eval(&mut child_env)
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Number;
    use crate::expr::BindingUsage;
    use crate::stmt::Stmt;

    #[test]
    fn parse_func_call_with_one_params() {
        assert_eq!(
            FuncCall::parse("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expr::Number(Number(10))],
                }
            ))
        )
    }

    #[test]
    fn eval_func_call() {
        let mut env = Env::default();

        env.store_func(
            "id".to_string(),
            vec!["x".to_string()],
            Stmt::Expr(Expr::BindingUsage(BindingUsage {
                name: "x".to_string(),
            })),
        );

        assert_eq!(
            FuncCall {
                callee: "id".to_string(),
                params: vec![Expr::Number(Number(10))],
            }
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }
}