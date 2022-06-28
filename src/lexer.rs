use std::{iter::{Peekable}, str::CharIndices, ops::Range, fmt, error::Error, };

use crate::token::{Tag, Token};

pub struct Lexer<'a> {
    pub(crate) src: &'a str,
    it: Peekable<CharIndices<'a>>,
    offset: usize,
    line: u32
}

impl<'a> Lexer<'a> {
    pub fn from(src: &'a str) -> Self {
        Self {
            src: src,
            it: src.char_indices().peekable(),
            offset: 0,
            line: 1
        }
    }

    fn range_lexed<T>(&mut self, lex_fn: impl FnOnce(&mut Self) -> T) -> (Range<usize>, T) {
        let start = self.offset;
        let return_value = lex_fn(self);
        (start..self.offset+1, return_value)        
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        let next = self.it.next()?;
        self.offset = next.0;
        Some(next)
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.it.peek()
    }

    fn dual_op(&mut self, next_char: char, tag: Tag, matched_tag: Tag) -> Token {
        match self.peek() {
            Some(&(_, char)) if char == next_char => {
                let _ = self.advance();
                Token::new(matched_tag, self.offset..self.offset + 2, self.line)
            },
            _ => Token::new(tag, self.offset..self.offset + 1, self.line), 
        }
    }

    fn lex_string(&mut self) -> bool {
        while let Some((_, c)) = self.advance() {
            match c {
                '"' => return true,
                'n' => {
                    self.line += 1;
                    let _ = self.advance();
                },
                '\\' if matches!(self.peek(), Some(&(_, '\\' | '"'))) => {
                    let _ = self.advance();
                },
                _ => {}
            }
        }
        false
    }

    fn lex_number(&mut self) {
        while let Some(&(_,char)) = self.peek() {
            if char.is_ascii_digit() { let _ = self.advance(); }
            else { break; }
        }
    }

    fn lex_ident(&mut self) {
        while let Some(&(_, char)) = self.peek() {
            if char.is_alphabetic() || char == '_'  { let _ = self.advance(); }
            else { break; }
        }
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
            match self.peek() {
                Some(&(_, '\n')) => { 
                    self.advance();
                    self.line += 1;
                    break;
                },
                Some(&(_, ch)) if ch.is_ascii_whitespace() => {
                    self.advance();
                    break;
                },
                _ => break
            }
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some((index, char)) = self.advance() {
            match char {
                '+' => Some(self.dual_op('=', Tag::Plus, Tag::PlusEqual)),
                '-' => Some(self.dual_op('=', Tag::Minus, Tag::MinusEqual)),
                '/' => Some(self.dual_op('=', Tag::Slash, Tag::SlashEqual)),
                '*' => Some(self.dual_op('=', Tag::Asterisk, Tag::AsteriskEqual)),

                '.' => Some(Token::new(Tag::Dot, index..index + 1, self.line)),
                ',' => Some(Token::new(Tag::Comma, index..index + 1, self.line)),

                '!' => Some(self.dual_op('=', Tag::Bang, Tag::BangEqual)),
                '=' => Some(self.dual_op('=', Tag::Equal, Tag::EqualEqual)),
                '>' => Some(self.dual_op('=', Tag::Greater, Tag::GreaterEqual)),
                '<' => Some(self.dual_op('=', Tag::Less, Tag::LessEqual)),

                '[' => Some(Token::new(Tag::LBracket, index..index + 1, self.line)),
                ']' => Some(Token::new(Tag::RBracket, index..index + 1, self.line)),
                '{' => Some(Token::new(Tag::LBrace, index..index + 1, self.line)),
                '}' => Some(Token::new(Tag::RBrace, index..index + 1, self.line)),
                '(' => Some(Token::new(Tag::LParen, index..index + 1, self.line)),
                ')' => Some(Token::new(Tag::RParen, index..index + 1, self.line)),

                '"' => {
                    let (loc, closed) = self.range_lexed(|lexer| lexer.lex_string());

                    Some(Token::new(Tag::String { closed }, loc, self.line))
                }
                
                n if n.is_ascii_digit() => {
                    let loc = self.range_lexed(|lexer| lexer.lex_number()).0;

                    Some(Token::new(Tag::Number, loc, self.line))
                }

                n if n.is_alphabetic() || n == '_' => {
                    let loc = self.range_lexed(|lexer| lexer.lex_ident()).0;
                    dbg!(loc.clone(), self.src);
                    let lit = &self.src[loc.clone()];
                
                    let tag = Self::match_keyword(lit);

                    Some(Token::new(tag, loc, self.line))                    
                }
            
                _ => Some(Token::new(Tag::Invalid, self.offset..self.offset + char.len_utf8(), self.line)),
            }
        } else {
            None
        }
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

