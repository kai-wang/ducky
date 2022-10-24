
use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, PartialOrd, Ord, Hash, Copy, Clone, PartialEq, Eq, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    Root,
    BinaryExpr,
    PrefixExpr,

    #[regex("[ \n]+")]
    Whitespace,

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

    #[regex("#.*")]
    Comment,

    #[error]
    Error,
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
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
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Lexeme { kind, text: input }));
    }
    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitespace);
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

    #[test]
    fn lex_comment() {
        check("# foo", SyntaxKind::Comment);
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", SyntaxKind::Whitespace);
    }
}