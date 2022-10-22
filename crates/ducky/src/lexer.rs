
use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    Root,
    BinaryExpr,
    PrefixExpr,

    #[regex(" +")]
    Whitepsace,

    #[token("fn")]
    FnKw,

    #[token("let")]
    LetKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[error]
    Error,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        Some((kind, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((kind, input)));
    }
    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitepsace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_indent() {
        check("abc", SyntaxKind::Ident);
        check("abc1231", SyntaxKind::Ident);

    }

    #[test]
    fn lex_number() {
        check("123", SyntaxKind::Number);
    }

    #[test]
    fn lex_op() {
        check("+", SyntaxKind::Plus);
        check("-", SyntaxKind::Minus);
        check("*", SyntaxKind::Star);
        check("/", SyntaxKind::Slash);
        check("=", SyntaxKind::Equals);
        check("{", SyntaxKind::LBrace);
        check("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", SyntaxKind::RParen);
    }
}