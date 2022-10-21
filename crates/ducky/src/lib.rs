pub mod expr;
pub mod binding_def;
pub mod val;
pub mod env;
pub mod stmt;

mod utils;

pub use env::Env;
pub use val::Val;

#[derive(Debug)]
pub struct Parse(stmt::Stmt);

impl Parse {
    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::parse(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        dbg!(s);
        Err("input was not consumed fully by parser".to_string())
    }
}