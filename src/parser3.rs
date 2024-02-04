use crate::ast::Ident;
use crate::lexer::Lexer;
use crate::token::{Tag, Token, self};
use crate::ast2::*;
use crate::bumping::{Vec, Box};
use crate::errors::*;
use bumpalo::Bump;
use core::iter::Peekable;
use std::ops::Range;

/// Advance the lexer and return the expr
/// 
/// This macro mainly exists for consuming a token in the branch
/// it was identified. See `Parser::parse_prefix`
macro_rules! next_and_return {
    ($parser:ident, $exp:expr) => {{
        $parser.next();
        $exp
    }};
}

/// Advance the lexer without calling next
macro_rules! commit {
    ($parser:ident, $tok:ident) => {
        $parser.tokens.offset += $tok.value.len() as u32
    };
}
#[macro_export]
macro_rules! peek_matches {
    ($parser:ident, $tags:pat) => {
        matches!($parser.peek(), Some(Token { tag: $tags , .. }))
    };
}

/// The Haze Parser
/// 
/// # Examples
pub struct Parser<'a, 'bump> {
    /// Token stream created from lexing a source file/string
    tokens: Lexer<'a>,
    /// Last token,
    tok: Option<Token<'a>>,
    /// backing allocator for node allocation
    pub(crate) bump: &'bump Bump,
    pub(crate) errors: std::vec::Vec<ParseError>
}

pub type Program<'a, 'bump> = Vec<'bump, Node<'a, 'bump>>;
// type InfixParser = for<'a, 'bump> fn(&mut Parser<'a, 'bump>) -> Result<Expr<'a, 'bump>, ParseError>;
pub type PResult<Node> = Result<Node, ParseError>;

use ParseErrorKind::*;
use serde::de::Expected;

impl<'s, 'b> Parser<'s, 'b> {
    /// Constructs a new parser given a source string
    /// 
    /// # Examples
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let mut parser = Parser::new("
    /// let g = 1 * 2 + 5;
    /// ", &bump);
    /// let tree = parser.parse();
    /// ```
    pub fn new(source: &'s str, allocator: &'b Bump,) -> Self {
        let mut tokens = Lexer::from(source);
        let tok = tokens.next();
        Self {
            tokens: tokens,
            tok: tok,
            bump: allocator,
            errors: std::vec::Vec::with_capacity(50)
        }
    }

    /// Produces a AST 
    pub fn parse(&mut self) -> Node<'s, 'b> {
        let mut node = NodeBuilder::from_type(NodeType::TopDeclList, self.bump);

        while let Some(_) = self.peek() {
            let top_decl = match self.top_level_declaration() {
                Ok(decl) => decl,
                Err(ParsingError::Fatal) => return break,
                Err(err) => continue
            };
            node.add(top_decl);
        }

        return node.finish(false);
    }

    pub(crate) fn top_level_declaration(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let tok = self.peek().unwrap();

        match tok.tag {
            Tag::Module => self.module(),
            Tag::Import => self.import(),
            Tag::Fn => self.function_def(),
            Tag::Struct => self.struct_decl(),
            Tag::Type => self.type_alias(),
            Tag::Const => self.const_decl(),
            _ => todo!("top level don't want {:?}", tok),
        }
    }

    pub(crate) fn debug_errors(&self) {
        for err in self.errors.iter() {
            println!("{:?} found `{}` @ {}:{}\n>   {}\n", err.kind, 
            &self.tokens.src[err.location.start as usize..(err.location.start + err.location.len) as usize],
            err.location.line, err.location.start, self.tokens.src.lines().nth(err.location.line as usize - 1).unwrap_or("Empty line"));
        }
    }

    pub(crate) fn add_error(&mut self, kind: ParseErrorKind, loc: Loc) {
        self.errors.push(ParseError {
            kind,
            location: loc
        });
    }

    /// Checks whether the next token is of the specified tag
    /// without consuming the token itself
    pub(crate) fn peek_is(&mut self, tag: Tag) -> bool {
        matches!(self.peek(), Some(tok) if tok.tag == tag)
    }


    /// Consume the next token if the tag matches, otherwise do nothing.
    /// 
    /// This method is rarely used in the parser. Its main purpose is to parse
    /// an optional token but these rarely show up in the grammar.
    pub fn lazy_eat(&mut self, tag: Tag) {
        match self.peek() {
            Some(tok) if tok.tag == tag => {
                self.next();
            }
            _ => {}
        };
    }

    pub(crate) fn peek(&self) -> Option<Token<'s>> {
        self.tok
    }

    pub(crate) fn expect_token_loc(&mut self, token_tag: Tag, kind: ParseErrorKind, loc: Loc) -> Token<'s> {
        self.eat_token(token_tag)
            .unwrap_or_else(|| {
                self.add_error(kind, loc);
                Token::empty()
            })
    }

    pub(crate) fn expect_token_(&mut self, token_tag: Tag, kind: ParseErrorKind) -> Token<'s> {
        let loc = self.loc(1);
        self.eat_token(token_tag)
            .unwrap_or_else(|| {
                self.add_error(kind, loc);
                Token::empty()
            })
    }

    pub(crate) fn expect_token(&mut self, token_tag: Tag) -> Token<'s> {
        let loc = self.loc(1);
        self.eat_token(token_tag)
            .unwrap_or_else(|| {
                self.add_error(Expected(token_tag), loc);
                Token::empty()
            })
    }

    pub(crate) fn eat_token(&mut self, token_tag: Tag) -> Option<Token<'s>> {
        (self.peek()?.tag == token_tag).then(|| self.next().unwrap())
    }

    /// The simplest parser synchronizer.
    /// 
    /// Skips every token until a token matching the specified tag is found.
    /// 
    /// Notably, it doesn't consume the matched token.
    pub(crate) fn skip_until(&mut self, token_tag: Tag) {
        while let Some(tok) = self.peek() {
            if tok.tag == token_tag { break; }
        }
    }

    pub(crate) fn next(&mut self) -> Option<Token<'s>> {
        let next = self.tok;
        self.tok = self.tokens.next();
        next
    }

    pub(crate) fn loc(&self, len: u32) -> Loc { Loc::new(self.tokens.offset, len, self.tokens.line) }

}

mod expr {
    use super::Tag;

    pub fn infix_bp(tag: Tag) -> (u8, u8) {
        match tag {
            Tag::Minus | Tag::Plus => (1, 2),
            Tag::Slash | Tag::Asterisk => (3, 4),
            Tag::BangEqual
            | Tag::Greater
            | Tag::GreaterEqual
            | Tag::Less
            | Tag::LessEqual
            | Tag::EqualEqual => (5, 6),
            _ => panic!("tag should be a binary operator"),
        }
    }

    pub fn prefix_bp(tag: Tag) -> ((), u8) {
        match tag {
            Tag::Minus | Tag::Bang => ((), 7),
            _ => panic!("tag should be an unary operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::{Parser, Bump};

    const WHOLE_SOURCE: &str = r#"let PI = 3.14;

    fn area_circle(radius) {
        return PI * radius * radius;
    }
    
    let radius = int(input("What is the radius"));
    print(area_circle(radius));
    
    let students = [];
    
    while true {
        print("Enter student record");
        let name = input("Student Name: ");
        let age = int(input("Student Age: "));
        let class = input("Student's class");
    
    
        if age > 18 {
            return print("This person is too old");
        }
        if class.len() > 3 { return print("Invalid class name"); }
    
        students.push((name, age, class));
    
        if input("Exit? ") {
            return print("Exiting record system")
        }
    }"#;

    // fn bench_parser(b: &mut test::Bencher) {
    //     let bump = Bump::new();
    //     let parser = Parser::new("", &bump);

    //     b.bench(|| {
            
    //     })
    // }
}