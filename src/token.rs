#[derive(Debug, PartialEq)]
pub enum TokenType {
    UNDEF,

    Float(f64),
    Integer(u32),

    LParen,
    RParen,
    Negative,
    EOL,

    RX,
    RZ,
    CZ,
    MEASURE,
}

// Wrap token type with line+pos info
#[derive(Debug, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub line: u32,
    pub pos: u32,
    pub len: usize,
}
