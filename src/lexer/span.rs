#[derive(Debug)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub position: usize,
}