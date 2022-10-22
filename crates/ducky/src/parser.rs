mod expr;

use crate::lexer::{Lexer, SyntaxKind};
use rowan::{Checkpoint, GreenNodeBuilder, Language, GreenNode};
use crate::syntax::{DuckLang, SyntaxNode};
use std::iter::Peekable;
use expr::expr;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        expr(&mut self);

        self.finish_node();

        Parse {
            green_node: self.builder.finish()
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder.token(DuckLang::kind_to_raw(kind), text.into());
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, DuckLang::kind_to_raw(kind));
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(DuckLang::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        //trim the last newline character
        formatted[0..formatted.len() - 1].to_string()
    }
}

