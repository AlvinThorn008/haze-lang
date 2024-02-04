use serde::Serialize;
use std::{marker::PhantomData, ops::Range};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tag {
    // Operators
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Slash,
    SlashEqual,
    Asterisk,
    AsteriskEqual,

    Dot,
    DotDot, // Range
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

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
    Arrow,

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
    Module,
    Struct,
    Enum,
    Import,
    Type, // type keyword, not an actual type
    Const,

    UnexpectedEof,
    Invalid,
}

pub struct Span<'a, T: 'a> {
    start: u32,
    len: u32,
    _marker: PhantomData<&'a T>,
}

impl<'a> Span<'a, &'a str> {
    fn new(start: u32, len: u32) -> Self {
        Self {
            start,
            len,
            _marker: PhantomData,
        }
    }
}

impl<'a> From<Range<u32>> for Span<'a, &'a str> {
    fn from(range: Range<u32>) -> Self {
        Self {
            start: range.start,
            len: range.len() as u32,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'s> {
    pub tag: Tag,
    pub pos: u32,
    pub value: &'s str,
    pub line: u32,
}

impl Serialize for Token<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value)
    }
}

impl<'s> Token<'s> {
    pub fn new(tag: Tag, value: &'s str, pos: u32, line: u32) -> Self {
        Self {
            tag,
            value,
            pos,
            line,
        }
    }

    pub const fn empty() -> Self {
        Self {
            tag: Tag::Invalid,
            value: "",
            pos: u32::MAX,
            line: u32::MAX,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tag == Tag::Invalid && self.pos == u32::MAX
    }
}
