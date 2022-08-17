use crate::lexer::Lexer;
use crate::token::{Tag, Token};
// use crate::ast::*;
use crate::ast::*;
use crate::bumping::{Vec, Box};
use bumpalo::Bump;

/// The Haze Parser
/// 
/// # Examples
pub struct Parser<'a, 'bump> {
    /// Token stream created from lexing a source file/string
    tokens: Lexer<'a>,
    /// backing allocator for node allocation
    bump: &'bump Bump
}

pub type Program<'a, 'bump> = Vec<'bump, Node<'a, 'bump>>;

impl<'a, 'bump> Parser<'a, 'bump> {
    /// Constructs a new parser given a source string 
    pub fn new(source: &'a str, allocator: &'bump Bump) -> Self {
        Self {
            tokens: Lexer::from(source),
            bump: allocator
        }
    }

    pub fn parse(&mut self) -> Result<Program<'a, 'bump>, &'static str> {
        let mut program = Program::new_in(self.bump);

        while let Some(t) = self.peek() {
            let stmt = self.parse_statement().map(Into::<Node>::into)?;
            program.0.push(stmt);
        }

        return Ok(program);
    }

    fn parse_statement(&mut self) -> Result<Stmt<'a, 'bump>, &'static str> {
        let tok = self.peek().expect("self.tokens shouldn't be consumed");

        match tok.tag {
            Tag::Fn => Ok(Stmt::FuncDecl(Box::new_in(self.bump, self.parse_func_decl()?))),
            Tag::Let => Ok(Stmt::VarDecl(Box::new_in(self.bump, self.parse_var_decl()?))),
            Tag::Semicolon => self.next_after(Ok(Stmt::Empty(EmptyStmt(tok)))),
            _ => Ok(Stmt::Expr(self.parse_expr_stmt()?)),
        }
    }

    fn parse_func_decl(&mut self) -> Result<FuncDecl<'a, 'bump>, &'static str> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // consume`fn` keyword
        let name = self
            .eat_token(Tag::Ident)
            .map(Ident)
            .ok_or("Identifier expected")?;
        let mut params: Vec<'bump, Ident<'a>> = Vec::new_in(self.bump);
        self.eat_token(Tag::LParen).ok_or("Left paren expected")?;

        // Early return for functions without parameters
        match self.eat_token(Tag::RParen) {
            Some(_) => {
                return Ok(FuncDecl {
                    name,
                    params,
                    body: self.parse_block_stmt()?,
                })
            }
            None => {}
        };

        // Parameter parsing
        loop {
            params.0.push(
                self.eat_token(Tag::Ident)
                    .map(Ident)
                    .ok_or("Missing identifier")?,
            );

            // RParen or Comma is expected after an ident
            match self.peek() {
                Some(tok) if tok.tag == Tag::RParen => {
                    self.next();
                    break;
                }
                Some(tok) if tok.tag == Tag::Comma => {
                    self.next();
                }
                _ => return Err("Expected right paren or comma"),
            }
        }

        Ok(FuncDecl {
            name,
            params,
            body: self.parse_block_stmt()?,
        })
    }

    fn parse_var_decl(&mut self) -> Result<VarDecl<'a, 'bump>, &'static str> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // consume let keyword
        let name = self
            .eat_token(Tag::Ident)
            .map(Ident)
            .ok_or("Identifier expected")?;

        match self.peek() {
            Some(tok) if tok.tag == Tag::Equal => {
                self.next();
            }
            Some(tok) if tok.tag == Tag::Semicolon => {
                self.next();
                return Ok(VarDecl { name, value: None }); // uninitialized var decl
            }
            _ => return Err("Unexpected token"),
        };

        let value = Some(self.parse_expr()?);

        self.eat_token(Tag::Semicolon).ok_or("Expected `;`")?;

        Ok(VarDecl { name, value })
    }

    fn parse_expr_stmt(&mut self) -> Result<ExprStmt<'a, 'bump>, &'static str> {
        let tok = self.peek().expect("self.tokens shouldn't be consumed");

        let expr = match tok.tag {
            Tag::If => {
                let exp = Expr::If(Box::new_in(self.bump, self.parse_if_expr()?));
                self.lazy_eat(Tag::Semicolon);
                exp
            }
            Tag::While => {
                let exp = Expr::While(Box::new_in(self.bump, self.parse_while_expr()?));
                self.lazy_eat(Tag::Semicolon);
                exp
            }
            Tag::Return => {
                let exp = Expr::Return(Box::new_in(self.bump, self.parse_return_expr()?));
                self.lazy_eat(Tag::Semicolon);
                exp
            }
            _ => {
                let exp = self.parse_expr()?;
                self.lazy_eat(Tag::Semicolon);
                exp
            }
        };
        Ok(ExprStmt { expr })
    }

    fn parse_block_stmt(&mut self) -> Result<BlockStmt<'a, 'bump>, &'static str> {
        self.parse_block_expr()
            .map(|expr| BlockStmt { body: expr.body })
    }

    fn parse_block_expr(&mut self) -> Result<BlockExpr<'a, 'bump>, &'static str> {
        self.eat_token(Tag::LBrace).ok_or("Expected `{`")?;

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
                None => return Err("RBrace not found"),
            }
        }

        Ok(BlockExpr { body: stmts })
    }

    fn parse_expr(&mut self) -> Result<Expr<'a, 'bump>, &'static str> {
        self.parse_expr_bp(0)
    }

    fn parse_if_expr(&mut self) -> Result<IfExpr<'a, 'bump>, &'static str> {
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
            _ => return Err("Abort!!! brace missing - ifs lost"),
        };

        Ok(IfExpr {
            condition,
            consequence,
            alternate,
        })
    }

    fn parse_while_expr(&mut self) -> Result<WhileExpr<'a, 'bump>, &'static str> {
        let _ = self.next().expect("self.tokens shouldn't be consumed");
        let condition = self.parse_expr()?;
        let consequence = self.parse_block_expr()?;

        Ok(WhileExpr {
            condition,
            consequence,
        })
    }

    fn parse_return_expr(&mut self) -> Result<ReturnExpr<'a, 'bump>, &'static str> {
        let _ = self.next().expect("self.tokens shouldn't be consumed"); // consume return keyword

        match self.peek() {
            Some(tok) if matches!(tok.tag, Tag::Semicolon | Tag::RBrace) => {
                Ok(ReturnExpr { value: None })
            }
            Some(tok) => Ok(ReturnExpr {
                value: Some(self.parse_expr()?),
            }),
            None => Err("Expected `:`, `}` or an operator"),
        }
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr<'a, 'bump>, &'static str> {
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

    fn parse_prefix(&mut self) -> Result<Expr<'a, 'bump>, &'static str> {
        let tok = self.peek().expect("self.tokens should not be consumed");
        match tok.tag {
            Tag::Ident => self.next_after(Ok(Expr::Id(Ident(tok)))),
            Tag::String { .. } => self.next_after(Ok(Expr::Str(Str(tok)))),
            Tag::Bool => self.next_after(Ok(Expr::Bool(Bool(tok)))),
            Tag::Number => self.next_after(Ok(Expr::Int(Int(tok)))),
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
                self.eat_token(Tag::RParen)
                    .ok_or("Missing closing parenthesis")?;
                Ok(Expr::Group(Box::new_in(self.bump, Group(lhs))))
            }
            Tag::If => Ok(Expr::If(Box::new_in(self.bump, self.parse_if_expr()?))),
            Tag::While => Ok(Expr::While(Box::new_in(self.bump, self.parse_while_expr()?))),
            Tag::Return => Ok(Expr::Return(Box::new_in(self.bump, self.parse_return_expr()?))),
            Tag::LBrace => Ok(Expr::Block(Box::new_in(self.bump, self.parse_block_expr()?))),
            _ => {
                return Err("Expected expression")
            }
        }
    }

    pub fn lazy_eat(&mut self, tag: Tag) {
        match self.peek() {
            Some(tok) if tok.tag == tag => {
                self.next();
            }
            _ => {}
        };
    }

    fn peek(&mut self) -> Option<Token<'a>> {
        self.tokens.clone().next()
    }

    fn next_after<T>(&mut self, value: T) -> T {
        let result = value;
        self.next();
        result
    }

    fn lazy_next_after<T>(&mut self, func: impl FnOnce(&mut Self) -> T) -> T {
        let result = func(self);
        self.next();
        result
    }

    fn eat_token(&mut self, token_tag: Tag) -> Option<Token<'a>> {
        (self.peek()?.tag == token_tag).then(|| self.tokens.next().unwrap())
    }

    fn eat_token_if(&mut self, predicate: impl FnOnce(Tag) -> bool) -> Option<Token<'a>> {
        predicate(self.peek()?.tag).then(|| self.tokens.next().unwrap())
    }

    fn must_eat_token_if(
        &mut self,
        predicate: impl FnOnce(Tag) -> bool,
    ) -> Option<Option<Token<'a>>> {
        match self.peek() {
            None => None,
            Some(tok) if predicate(tok.tag) => Some(Some(self.tokens.next().unwrap())),
            _ => Some(None),
        }
    }

    fn next(&mut self) -> Option<Token<'a>> {
        self.tokens.next()
    }
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