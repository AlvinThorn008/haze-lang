use std::{iter::{Peekable}, str::CharIndices, ops::Range, fmt, error::Error, };

use crate::token::{Tag, Token};

pub struct Lexer<'a> {
    pub(crate) src: &'a str,
    it: CharIndices<'a>,
    /// Most recently returned item from `it`
    prev: (usize, char),
    line: u32
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

/// A lexer that turns a string literal to a stream of tokens.
/// 
/// The `Lexer` type borrows the initial string literal.
impl<'a> Lexer<'a> {

    /// Creates a `Lexer` from a string literal.
    pub fn from(src: &'a str) -> Self {
        let mut lexer = Self {
            src: src,
            it: src.char_indices(),
            prev: (0, '\x00'),
            line: 1
        };
        lexer.advance();
        lexer
    }

    /// Consumes the next character of the source.
    fn advance(&mut self) {
        if let Some(char_idx_tuple) = self.it.next() {
            self.prev = char_idx_tuple;
        } else {
            self.prev = (self.src.len(), '\x00') // '\x00' acts a null value - Don't worry about it
        }
    }

    /// Returns the next character of the source without consuming it.
    fn peek(&mut self) -> Option<(usize, char)> {
        self.it.clone().next()
    }

    /// Returns a `Token` where its `tag` is `matched_tag` if the next character in the source is equal to `next_char`, otherwise its tag is `tag`
    /// 
    /// This method is used internally to scan double-length tokens: '+=', '-=', '*=' and so on.
    fn dual_op(&mut self, next_char: char, tag: Tag, matched_tag: Tag) -> Token<'a> {
        let start = self.prev.0;
        match self.peek() {
            Some((_, char)) if char == next_char => {
                self.advance();
                Token::new(matched_tag, &self.src[start..start + 2], start, self.line)
            },
            _ => Token::new(tag, &self.src[start..start + 1],start, self.line), 
        }
    }

    /// Returns a 
    fn lex_string(&mut self) -> Token<'a> { 
        let start = self.prev.0;
        let mut closed = false;
        while !self.done() {
            self.advance();

            match self.prev.1 {
                '"' => {
                    closed = true;
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                '\\' => {
                    if matches!(self.peek(), Some((_, '\\' | '"'))) {
                        self.advance();
                    }
                    
                }
                _ => {}
            }
        }
        let end = self.prev.1.len_utf8() + self.prev.0;
        Token::new(Tag::String { closed }, &self.src[start..end], start, self.line)
    }

    fn lex_number(&mut self) -> Token<'a> {
        let start = self.prev.0;

        // Redundant digit check 
        while self.prev.1.is_ascii_digit() { 
            self.advance();
        }

        // so the length of the last char needn't be added
        Token::new(Tag::Number, &self.src[start..self.prev.0], start, self.line)
    }

    fn lex_ident(&mut self) -> Token<'a> {
        let start = self.prev.0;

        while let Some((_, char)) = self.peek() {
            if char.is_alphabetic() || char == '_'  { self.advance(); }
            else { break; }
        }

        let end = self.prev.0 + self.prev.1.len_utf8();
        let lit = &self.src[start..end];

        let tag = Self::match_keyword(lit);

        Token::new(tag, lit, start, self.line)
    }

    fn match_keyword(ident: &str) -> Tag {
        match ident {
            "fn" => Tag::Fn,
            "if" => Tag::If,
            "else" => Tag::Else,
            "return" => Tag::Return,
            "while" => Tag::While,
            "for" => Tag::For,
            "let" => Tag::Let,
            _ => Tag::Ident
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.prev.1 {
                '\n' => { 
                    self.advance();
                    self.line += 1;
                },
                '\t' | '\r' | ' ' => {
                    self.advance();
                },
                _ => break
            }
        }
    }

    fn done(&self) -> bool {
        self.prev.0 >= self.src.len() - 1
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();

        if self.done() { return None; }

        let start = self.prev.0;
        let tok = match self.prev.1 { // last char yieled

            '+' => Some(self.dual_op('=', Tag::Plus, Tag::PlusEqual)),
            '-' => Some(self.dual_op('=', Tag::Minus, Tag::MinusEqual)),
            '/' => Some(self.dual_op('=', Tag::Slash, Tag::SlashEqual)),
            '*' => Some(self.dual_op('=', Tag::Asterisk, Tag::AsteriskEqual)),

            '.' => Some(Token::new(Tag::Dot, &self.src[start..start + 1], start, self.line)),
            ',' => Some(Token::new(Tag::Comma, &self.src[start..start + 1], start, self.line)),

            '!' => Some(self.dual_op('=', Tag::Bang, Tag::BangEqual)),
            '=' => Some(self.dual_op('=', Tag::Equal, Tag::EqualEqual)),
            '>' => Some(self.dual_op('=', Tag::Greater, Tag::GreaterEqual)),
            '<' => Some(self.dual_op('=', Tag::Less, Tag::LessEqual)),

            '[' => Some(Token::new(Tag::LBracket, &self.src[start..start + 1], start, self.line)),
            ']' => Some(Token::new(Tag::RBracket, &self.src[start..start + 1], start, self.line)),
            '{' => Some(Token::new(Tag::LBrace, &self.src[start..start + 1], start, self.line)),
            '}' => Some(Token::new(Tag::RBrace, &self.src[start..start + 1], start, self.line)),
            '(' => Some(Token::new(Tag::LParen, &self.src[start..start + 1], start, self.line)),
            ')' => Some(Token::new(Tag::RParen, &self.src[start..start + 1], start, self.line)),

            '"' => {
                Some(self.lex_string())
            }
            
            n if n.is_ascii_digit() => {
                Some(self.lex_number())
            }

            n if n.is_alphabetic() || n == '_' => {
                Some(self.lex_ident())
            }
    
            _ => {
                let last_char_len = self.prev.1.len_utf8();
                Some(Token::new(Tag::Invalid, &self.src[start..start + last_char_len], start, self.line))
            }
        };
        self.advance();
        tok  
    }

    /// Determines if a string literal token is valid
    /// Panics if token is not `Token::String`.
    pub fn validate_string(token: Token) -> Result<(), StringError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum StringError {
    UnClosedDelimeter,
    InvalidEscape { loc: Range<usize> },
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Provide representations for each variant
        f.write_str("StringError")
    }
}

impl Error for StringError {}

