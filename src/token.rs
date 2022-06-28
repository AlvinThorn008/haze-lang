use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub enum Tag {
    // Operators
    Plus, PlusEqual,
    Minus, MinusEqual,
    Slash, SlashEqual,
    Asterisk, AsteriskEqual,
    
    Dot,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Meta tokens
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,

    // Literals
    Ident,
    String { closed: bool },
    Bool,
    Number,

    // Keywords
    Fn,
    If,
    Else,
    Return,
    While,
    For,
    Let,
    

    Invalid
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: Tag,
    pub loc: Range<usize>,
    pub line: u32
}



impl Token {
    pub fn new(tag: Tag, location: Range<usize>, line: u32) -> Self {
        Self {
            tag,
            loc: location,
            line
        }
    }
}