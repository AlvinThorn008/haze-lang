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
    DotDot, // Range
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
    Colon,
    Comma,

    // Literals
    Ident,
    String,
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
    Break,
    Continue,

    UnexpectedEof,
    Invalid
}

#[derive(Debug, Clone)]
pub struct Token<'s> {
    pub tag: Tag,
    pub pos: u32,
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
    pub fn new(tag: Tag, value: &'s str, pos: u32, line: u32) -> Self {
        Self {
            tag,
            value,
            pos,
            line
        }
    }
}