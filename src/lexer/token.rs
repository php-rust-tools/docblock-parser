use super::span::Span;

#[derive(Debug, Clone)]
pub enum TokenKind {
    
}

pub struct Token<'a> {
    pub kind: TokenKind,
    pub slice: &'a str,
    pub span: Span,
}