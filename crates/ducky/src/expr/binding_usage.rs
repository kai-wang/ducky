use crate::utils;
use crate::env::Env;
use crate::val::Val;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BindingUsage {
    pub(crate) name: String
}

impl BindingUsage {
    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((
            s,
            Self {
                name: name.to_string()
            }
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::parse("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        )
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_bindings("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string()
            }
            .eval(&env),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn eval_non_existent_binding_usage() {
        let env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "dont_exist".to_string()
            }
            .eval(&env).is_err(),
            true
        )
    }
}