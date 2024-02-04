use crate::lexer::Lexer;
use crate::token::{Tag, Token, self};
use crate::ast::*;
use crate::bumping::{Vec, Box};
use crate::errors::*;
use bumpalo::Bump;

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

/// The Haze Parser
/// 
/// # Examples
pub struct Parser<'a, 'bump> {
    /// Token stream created from lexing a source file/string
    tokens: Lexer<'a>,
    /// backing allocator for node allocation
    bump: &'bump Bump,
    errors: std::vec::Vec<ParseError>
}

pub type Program<'a, 'bump> = Vec<'bump, Node<'a, 'bump>>;
type InfixParser = for<'a, 'bump> fn(&mut Parser<'a, 'bump>) -> Result<Expr<'a, 'bump>, ParseError>;
pub type PResult<Node> = Result<Node, ParseError>;

use ParseErrorKind::*;
use serde::de::Expected;

impl<'a, 'bump> Parser<'a, 'bump> {
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
    pub fn new(source: &'a str, allocator: &'bump Bump,) -> Self {
        let mut tokens = Lexer::from(source);
        Self {
            tokens: tokens,
            bump: allocator,
            errors: std::vec::Vec::with_capacity(50)
        }
    }

    /// Produces a AST 
    pub fn parse(&mut self) -> PResult<Program<'a, 'bump>> {
        let mut program = Program::new_in(self.bump);

        while let Some(_) = self.peek() {
            let item = self.parse_statement().map(Into::<Node>::into)?;
            program.0.push(item);
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> PResult<Stmt<'a, 'bump>> {
        let tok = self.peek().expect("self.token should have checked ");

        match tok.tag {
            Tag::Fn => Ok(Stmt::FuncDecl(Box::new_in(self.bump, self.parse_func_decl()?))),
            Tag::Let => Ok(Stmt::VarDecl(Box::new_in(self.bump, self.parse_var_decl()?))),
            Tag::Semicolon => next_and_return!(self, Ok(Stmt::Empty(EmptyStmt(tok)))),
            _ => Ok(Stmt::Expr(self.parse_expr_stmt()?)),
        }
    }

    fn parse_var_decl(&mut self) -> PResult<VarDecl<'a, 'bump>> {
        todo!()
    }

    fn parse_expr_stmt(&mut self) -> PResult<ExprStmt<'a, 'bump>> {
        todo!()
    }

    fn parse_func_decl(&mut self) -> PResult<FuncDecl<'a, 'bump>> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // consume`fn` keyword
        let loc = self.loc(1);
        let name = Ident(self.expect_token(Tag::Ident, Expected(Tag::Ident)));
        let mut params: Vec<'bump, Ident<'a>> = Vec::new_in(self.bump);
        self.expect_token(Tag::LParen, ExpectedLBrace);

        // Early return for functions without parameters
        match self.eat_token(Tag::RParen) {
            Some(_) => {
                return Ok(FuncDecl { 
                    name, 
                    params, 
                    body: self.parse_block_stmt()?
                })
            }
            None => {}
        }

        // Parameter parsing
        loop {
            params.0.push(Ident(
                self.expect_token(Tag::Ident, Expected(Tag::Ident))
            ));

            // RParen or Comma is expected after an ident
            match self.peek() {
                Some(tok) if tok.tag == Tag::RParen => {
                    self.next();
                    break;
                }
                Some(tok) if tok.tag == Tag::Comma => {
                    self.next();
                }
                _ => break self.add_error(Expected(Tag::Comma), self.loc(1)),
            }
        }

        Ok(FuncDecl { name, params, body: self.parse_block_stmt()? })
    }

    fn parse_block_stmt(&mut self) -> PResult<BlockStmt<'a, 'bump>> {
        self.parse_block_expr()
            .map(|expr| BlockStmt { body: expr.body })
    }

    fn parse_block_expr(&mut self) -> PResult<BlockExpr<'a, 'bump>> {
        let lbrace = self.expect_token(Tag::LBrace, Expected(Tag::LBrace));

        let mut stmts = Vec::new_in(self.bump);

        loop {
            match self.peek() {
                Some(tok) if tok.tag == Tag::RBrace => {
                    self.next();
                    break;
                }
                tok => {
                    stmts.0.push(self.parse_statement()?);
                }
                None => break self.add_error(Expected(Tag::RBrace), self.loc(1)),
            }
        }

        Ok(BlockExpr { body: Box(stmts.0.into_boxed_slice()) })
    }

    fn parse_expr(&mut self) -> PResult<Expr<'a, 'bump>> {
        self.parse_expr_bp(0)
    }

    fn parse_if_expr(&mut self) -> PResult<IfExpr<'a, 'bump>> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // if keyword
        let condition = self.parse_expr()?;
        let consequence = self.parse_block_expr()?;
        let mut alternate: Option<Box<IfAlt<'a, 'bump>>> = None;

        // null alternate if no else token follows
        match self.peek() {
            Some(tok) if tok.tag == Tag::Else => {
                self.next();
            }
            _ => {
                return Ok(IfExpr {
                    condition,
                    consequence,
                    alternate,
                })
            }
        };

        match self.peek() {
            Some(tok) if tok.tag == Tag::If => {
                // Parse else if
                alternate = Some(Box::new_in(self.bump, IfAlt::ElseIf(self.parse_if_expr()?)));
            }
            Some(tok) if tok.tag == Tag::LBrace => {
                // Parse else block
                alternate = Some(Box::new_in(self.bump, IfAlt::Else(self.parse_block_expr()?)))
            }
            // Bro I have no clue what happens here
            _ => return Err(ParseError { kind: Expected(Tag::LBrace), location: self.loc(1) })
            // _ => return Err("Abort!!! brace missing - ifs lost"),
        };

        Ok(IfExpr {
            condition,
            consequence,
            alternate,
        })
    }

    fn parse_while_expr(&mut self) -> PResult<WhileExpr<'a, 'bump>> {
        let _ = self.next().expect("self.tokens shouldn't be consumed");
        let condition = self.parse_expr()?;
        let consequence = self.parse_block_expr()?;

        Ok(WhileExpr {
            condition,
            consequence,
        })
    }

    fn parse_return_expr(&mut self) -> PResult<ReturnExpr<'a, 'bump>> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // consume return keyword

        match self.peek() {
            Some(tok) if matches!(tok.tag, Tag::Semicolon | Tag::RBrace) => {
                Ok(ReturnExpr { value: None })
            }
            Some(tok) => Ok(ReturnExpr {
                value: Some(self.parse_expr()?),
            }),
            None => {
                // For now, we'll group errors based on their location.
                self.add_error(ExpectedExpr, self.loc(1));
                self.add_error(Expected(Tag::Semicolon), self.loc(1));
                Ok(ReturnExpr { value: None })
            }
            // None => Err("Expected `;`, `}` or an operator"),
        }
    }

    fn parse_break_expr(&mut self) -> PResult<BreakExpr<'a, 'bump>> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // consume return keyword

        match self.peek() {
            Some(tok) if matches!(tok.tag, Tag::Semicolon | Tag::RBrace) => {
                Ok(BreakExpr { value: None })
            }
            Some(tok) => Ok(BreakExpr {
                value: Some(self.parse_expr()?),
            }),
            None => {
                // For now, we'll group errors based on their location.
                self.add_error(ExpectedExpr, self.loc(1));
                self.add_error(Expected(Tag::Semicolon), self.loc(1));
                Ok(BreakExpr { value: None })
            }
            // None => Err("Expected `:`, `}` or an operator"),
        }
    }

    // Core expression parser based on Pratt parsing
    fn parse_expr_bp(&mut self, min_bp: u8) -> PResult<Expr<'a, 'bump>> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let op = match self.peek() {
                Some(t) if tag_is_binop(t.tag) => t,
                _ => break,
            };

            let (l_bp, r_bp) = expr::infix_bp(op.tag);

            if l_bp < min_bp {
                break;
            }

            self.next();
            let rhs = self.parse_expr_bp(r_bp)?;

            lhs = Expr::Infix(Box::new_in(self.bump, Infix {
                left: lhs,
                op,
                right: rhs,
            }));
        }
        Ok(lhs)
    }

    // fn parse_postfix(&mut self, tag: Tag) -> Option<(u8, InfixParser)> {

    //     macro_rules! infix_expr {
    //         ($self:ident,$bp:literal) => {{
    //             ($bp, |self_| { self_.parse_expr_bp($bp + 1) })
    //         }};
    //     }

    //     // Notice how the binding powers are consecutive odd numbers. 
    //     // This is because the right binding power of any infix operator is
    //     // its left binding power plus 1. This prevents clashing.
    //     // e.g. + and - have a left bp of 1 and a right bp of 2
    //     match tag {
    //         Tag::Minus | Tag::Plus => Some(infix_expr!(self, 1)),
    //         Tag::Slash | Tag::Asterisk => Some(infix_expr!(self, 3)),
    //         Tag::BangEqual
    //         | Tag::Greater
    //         | Tag::GreaterEqual
    //         | Tag::Less
    //         | Tag::LessEqual
    //         | Tag::EqualEqual => Some(infix_expr!(self, 5)),
    //         _ => None
    //     }
    // }

    fn parse_prefix(&mut self) -> PResult<Expr<'a, 'bump>> {
        let tok = self.peek().expect("self.tokens should not be consumed");
        match tok.tag {
            Tag::Ident => {
                self.next(); // ident
                let next_tok = if let Some(tok) = self.peek() { tok } else { return Ok(Expr::Id(Ident(tok))); };
                match next_tok.tag {
                    Tag::Equal => self.consume_assignment(tok),
                    Tag::LParen => self.consume_call_expr(tok),
                    _ => Ok(Expr::Id(Ident(tok)))
                }
            },
            Tag::String => next_and_return!(self, Ok(Expr::Str(Str(tok)))),
            Tag::Bool => next_and_return!(self, Ok(Expr::Bool(Bool(tok)))),
            Tag::Number => next_and_return!(self, Ok(Expr::Int(Int(tok)))),
            tag if tag_is_unaryop(tag) => {
                self.next();
                let ((), _r_bp) = expr::prefix_bp(tag);
                let rhs = self.parse_expr()?;
                Ok(Expr::Prefix(Box::new_in(self.bump, Prefix {
                    op: tok,
                    right: rhs,
                })))
            }
            Tag::LParen => {
                self.next();
                let lhs = self.parse_expr()?;
                self.add_error(Expected(Tag::RParen), self.loc(1));
                // self.eat_token(Tag::RParen)
                //     .ok_or("Missing closing parenthesis")?;
                Ok(Expr::Group(Box::new_in(self.bump, Group(lhs))))
            }
            Tag::LBracket => self.consume_array_expr(),
            Tag::If => Ok(Expr::If(Box::new_in(self.bump, self.parse_if_expr()?))),
            Tag::While => Ok(Expr::While(Box::new_in(self.bump, self.parse_while_expr()?))),
            Tag::Return => Ok(Expr::Return(Box::new_in(self.bump, self.parse_return_expr()?))),
            Tag::LBrace => Ok(Expr::Block(Box::new_in(self.bump, self.parse_block_expr()?))),
            Tag::Break => Ok(Expr::Break(Box::new_in(self.bump, self.parse_break_expr()?))),
            _ => {
                // We really need null nodes
                return Err(ParseError { kind: ExpectedExpr, location: self.loc(1) })
            }
        }
    }

    fn consume_assignment(&mut self, tok: Token<'a>) -> PResult<Expr<'a, 'bump>> {
        self.next();
        Ok(Expr::Assign(Box::new_in(self.bump, AssignExpr { 
            ident: Ident(tok), 
            value: self.parse_expr()? 
        })))
    }

    fn consume_call_expr(&mut self, tok: Token<'a>) -> PResult<Expr<'a, 'bump>> {
        self.next(); // LParen
        let mut args = Vec::new_in(self.bump);

        while self.eat_token(Tag::RParen).is_none() {
            let arg = match self.parse_expr() {
                Ok(expr) => expr,
                Err(_) => { self.add_error(Expected(Tag::Ident), self.loc(1)); break; }
                // Err(_) => return Err("Expected Identifier or RParen")
            };

            args.0.push(arg);

            match self.peek() {
                Some(tok) if tok.tag == Tag::Comma => {
                    self.next();
                }
                Some(tok) if tok.tag == Tag::RParen => {
                    self.next();
                    break;
                }
                _ => {}
            }
        }
        Ok(Expr::Call(Box::new_in(self.bump, CallExpr { 
            name: Ident(tok), 
            args 
        })))
    }

    fn consume_array_expr(&mut self) -> PResult<Expr<'a, 'bump>> {
        self.next(); // LParen
        let mut items = Vec::new_in(self.bump);

        while self.eat_token(Tag::RBracket).is_none() {
            let item = match self.parse_expr() {
                Ok(expr) => expr,
                Err(_) => { self.add_error(Expected(Tag::Ident), self.loc(1)); break; }
                // Err(_) => return Err("Expected Identifier or RParen")
            };

            items.0.push(item);

            match self.peek() {
                Some(tok) if tok.tag == Tag::Comma => {
                    self.next();
                }
                Some(tok) if tok.tag == Tag::RBracket => {
                    self.next();
                    break;
                }
                _ => {}
            }
        }
        Ok(Expr::Array(Box::new_in(self.bump, ArrayExpr { 
            items
        })))
    }

    fn add_error(&mut self, kind: ParseErrorKind, loc: Loc) {
        self.errors.push(ParseError {
            kind,
            location: loc
        });
    }


    pub fn lazy_eat(&mut self, tag: Tag) {
        match self.peek() {
            Some(tok) if tok.tag == tag => {
                self.next();
            }
            _ => {}
        };
    }

    fn peek(& self) -> Option<Token<'a>> {
        self.tokens.clone().next()
    }

    fn expect_token_loc(&mut self, token_tag: Tag, kind: ParseErrorKind, loc: Loc) -> Token<'a> {
        self.eat_token(token_tag)
            .unwrap_or_else(|| {
                self.add_error(kind, loc);
                Token::empty()
            })
    }

    fn expect_token(&mut self, token_tag: Tag, kind: ParseErrorKind) -> Token<'a> {
        let loc = self.loc(1);
        self.eat_token(token_tag)
            .unwrap_or_else(|| {
                self.add_error(kind, loc);
                Token::empty()
            })
    }

    fn eat_token(&mut self, token_tag: Tag) -> Option<Token<'a>> {
        (self.peek()?.tag == token_tag).then(|| self.tokens.next().unwrap())
    }

    fn next(&mut self) -> Option<Token<'a>> {
        self.tokens.next()
    }

    fn loc(&self, len: u32) -> Loc { Loc::new(self.tokens.offset, len, self.tokens.line) }
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