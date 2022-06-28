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
pub struct Token<'s> {
    pub tag: Tag,
    pub value: &'s str,
    pub line: u32
}



impl<'s> Token<'s> {
    pub fn new(tag: Tag, value: &'s str, line: u32) -> Self {
        Self {
            tag,
            value,
            line
        }
    }
}