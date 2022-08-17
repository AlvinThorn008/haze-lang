use std::ops::Range;
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    Eof,
    

    Invalid
}

#[derive(Debug, Clone)]
pub struct Token<'s> {
    pub tag: Tag,
    pub pos: usize,
    pub value: &'s str,
    pub line: u32
}

impl Serialize for Token<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.value)
    }
}



impl<'s> Token<'s> {
    pub fn new(tag: Tag, value: &'s str, pos: usize, line: u32) -> Self {
        Self {
            tag,
            value,
            pos,
            line
        }
    }
}