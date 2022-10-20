use crate::{stmt::Stmt, utils};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>
}

impl Block {
    pub fn parse(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespaces(s);

        let mut s = dbg!(s);
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Stmt::parse(s) {
            s = new_s;
            stmts.push(stmt);
            let (new_s, _) = utils::extract_whitespaces(s);
            s = new_s;
        }

        let (s, _) = utils::extract_whitespaces(s);
        let s = utils::tag("}", s)?;

        Ok((s, Block { stmts }))
    }
}


#[cfg(test)]
mod tests {
    use super::super::{BindingUsage, Expr, Number};
    use super::*;
    use crate::binding_def::BindingDef;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::parse("{}"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_empty_block_with_spaces() {
        assert_eq!(Block::parse("{   }"), Ok(("", Block { stmts: Vec::new() })));
    }

    #[test]
    fn parse_block_with_one_stmt() {
        assert_eq!(
            Block::parse("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))],
                },
            )),
        );
    }

    #[test]
    fn parse_block_with_multiple_stmts() {
        assert_eq!(
            Block::parse(
                "{ 
                    let a = 10
                    let b = a
                    b
                }"
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10)),
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string()
                            })
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string()
                        }))
                    ]
                }
            ))
        )
    }
}