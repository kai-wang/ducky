use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Number(i32),
    Unit
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Unit => write!(f, "Unit")
        }
    }
}