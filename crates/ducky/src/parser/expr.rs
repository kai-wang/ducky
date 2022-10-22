use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, mininum_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => p.bump(),
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => Op::Add,
            Some(SyntaxKind::Minus) => Op::Sub,
            Some(SyntaxKind::Star) => Op::Mul,
            Some(SyntaxKind::Slash) => Op::Div,
            _ => return,
        };

        let (lbp, rbp) = op.binding_power();
        if lbp < mininum_binding_power {
            return;
        }
    
        p.bump();
        p.start_node_at(checkpoint, SyntaxKind::BinOp);
        expr_binding_power(p, rbp);
        p.finish_node();
    }

}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}