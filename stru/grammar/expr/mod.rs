use std::ops::BitOr;

use crate::ast::{tag_is_binop, tag_is_unaryop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
pub use super::Parser;

pub mod control_flow;
pub mod block;
pub mod array;
pub mod access;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn expr(&mut self) -> Result<NodeChild<'s, 'b>, ParsingError> {
        self.expr_bp(0, Restrictions::NONE)
    }

    pub(crate) fn expr_with_restrictions(&mut self, restrictions: Restrictions) -> Result<NodeChild<'s, 'b>, ParsingError> {
        self.expr_bp(0, restrictions);

        todo!()
    }

    fn expr_bp(&mut self, min_bp: u8, restrictions: Restrictions) -> Result<NodeChild<'s, 'b>, ParsingError> {
        let mut lhs = self.prefix(restrictions)?;
    
        loop {
            let op = match self.peek() {
                Some(token) if tag_is_binop(token.tag) => token,
                Some(token) => {
                    self.add_error(ExpectedOperator, Loc::from_token(token));
                    return Err(Failed)
                },
                None => {
                    self.add_error(UnexpectedEOF, self.loc(0));
                    // `Fatal` would work here too
                    return Err(Failed);
                }
            };

            let (lbp, rbp) = bp::infix(op.tag);

            if lbp < min_bp {
                break;
            }

            self.next(); // consume op

            // Parse the next expression preceding an operator
            // whose bp < rbp
            let rhs = self.expr_bp(rbp, restrictions)?;

            let mut lhs_builder = NodeBuilder::from_type(Infix, self.bump);
            lhs_builder.add(lhs);
            lhs_builder.add(op);
            lhs_builder.add(rhs);

            lhs = lhs_builder.finish(false).into();
        }

        Ok(lhs)
    }

    fn prefix(&mut self, restrictions: Restrictions) -> Result<NodeChild<'s, 'b>, ParsingError> {
        let tok = self.next().expect("caller should ensure a token is available");

        match tok.tag {
            Tag::Ident => {
                let Some(tok) = self.peek() 
                    else { self.add_error(ExpectedExpr, self.loc(0)); return Err(Failed); };
                match tok.tag {
                    Tag::LBracket => { self.next(); self.index_expr().map(NodeChild::Node) }
                    Tag::Dot => { self.next(); self.field_access_expr().map(NodeChild::Node) }
                    _ => Ok(tok.into())
                }
            },
            Tag::String | Tag::Bool | Tag::Number => Ok(tok.into()),
            tag if tag_is_unaryop(tag) => {
                let ((), rbp) = bp::prefix(tag);
                let rhs = self.expr_bp(rbp, restrictions)?;
                let mut node = NodeBuilder::from_type(Prefix, self.bump);
                node.add(tok); node.add(rhs);
                Ok(node.finish(false).into())
            }
            Tag::If => self.if_expr().map(NodeChild::Node),
            Tag::While => self.while_expr().map(NodeChild::Node),
            Tag::Return => self.return_expr().map(NodeChild::Node),
            Tag::Break => self.break_expr().map(NodeChild::Node),
            Tag::LBracket => self.array_expr().map(NodeChild::Node),
            Tag::LBrace => if restrictions.has(Restrictions::BLOCK) {
                self.add_error(BlockExprDisallowed, Loc::from_token(tok));
                return Err(Failed);
            } else {
                self.block_expr().map(NodeChild::Node)
            }
            _ => {
                // We generally don't recover to keep parsing expressions due to
                // the risk of cascading. Some common mistakes may be recovered from.
                self.add_error(ExpectedExpr, Loc::from_token(tok));
                // We task the caller with synchronization.
                return Err(Failed)
            }
        }
    }

    pub(crate) fn expr_synchronize(&mut self, closer: Tag) {
        use Tag::*;
        loop {
            let tag = self.peek().map(|tok| tok.tag); 
            match tag {
                Some(tag) if tag == closer => { self.next(); return; },
                // Leading tokens for expressions as the next statement
                // may well be an expression statement.
                Some(
                    | Let | String | Number | Bool | Minus 
                    | Bang | Break | Return | LParen | LBrace 
                    | LParen | If) => { return; }
                Some(_) => { self.next(); continue; }, 
                None => return // End of file
            }
        }
    }
}

/// Restrict the parsing of one or more alternation
/// of an expression.
#[derive(Clone, Copy)]
pub struct Restrictions(u8);

impl Restrictions {
    pub const NONE: Self = Self(0);
    /// Reject Block expression
    pub const BLOCK: Self = Self(1);

    // TODO: More restrictions like STRUCT

    #[inline]
    pub fn has(self, restriction: Self) -> bool {
        (self.0 & restriction.0) != 0
    }
}

impl BitOr for Restrictions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// The binding power module
/// 
/// Contains two simple functions that map operator to binding power.
/// 
/// ### Binding power
/// Consider the expression: `5 + 3 * 9`
/// 
/// 
/// 
mod bp {
    use super::Tag;

    pub fn infix(tag: Tag) -> (u8, u8) {
        match tag {
            Tag::Minus | Tag::Plus => (1, 2),
            Tag::Slash | Tag::Asterisk => (3, 4),
            Tag::BangEqual
            | Tag::Greater
            | Tag::GreaterEqual
            | Tag::Less
            | Tag::LessEqual
            | Tag::EqualEqual => (5, 6),
            _ => unreachable!()
        }
    }

    pub fn prefix(tag: Tag) -> ((), u8) {
        match tag {
            Tag::Minus | Tag::Bang => ((), 7),
            _ => unreachable!()
        }
    }
}