use crate::ast2::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
use crate::grammar::types::*;
pub use super::{Parser, Restrictions};

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn if_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        // ATM, this method assumes the if token is consumed
        let mut node = NodeBuilder::from_type(IfExpr, self.bump);

        let condition = self.expr_bp(0, Restrictions::BLOCK, |tok| tok.tag == Tag::LBrace)?;
        node.add(condition);

        if self.expect_token(Tag::LBrace).is_empty() {
            return Err(Failed);
        }

        let consequence = self.block_expr()?;
        node.add(consequence);

        let mut alternate = Node::null(Any);

        if self.peek_is(Tag::Else) { self.next(); }
        else { node.add(alternate); return Ok(node.finish(false)); }

        match self.peek() {
            Some(tok) if tok.tag == Tag::If => { self.next(); alternate = self.if_expr()?; },
            Some(tok) if tok.tag == Tag::LBrace => { self.next(); alternate = self.block_expr()?; },
            Some(tok) => {
                self.add_error(ExpectedIfOrBlock, Loc::from_token(tok));
                return Err(Failed);
            }
            None => {
                // self.add_error(ExpectedIfOrBlock, self.loc(0));
                self.add_error(UnexpectedEOF, self.loc(0));
                return Err(Failed);
            }
        }
        node.add(alternate);

        Ok(node.finish(false))
    }

    pub(crate) fn while_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        // ATM, this method assumes the while token is consumed
        let mut node = NodeBuilder::from_type(WhileExpr, self.bump);

        let condition = self.expr_bp(0, Restrictions::BLOCK, |tok| tok.tag == Tag::LBrace)?;
        node.add(condition);

        if self.expect_token(Tag::LBrace).is_empty() {
            return Err(Failed);
        }

        let consequence = self.block_expr()?;
        node.add(consequence);

        Ok(node.finish(false))
    }

    pub(crate) fn return_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(ReturnExpr, self.bump);
        match self.peek() {
            Some(tok) if matches!(tok.tag, Tag::Semicolon | Tag::RBrace) => {
                node.add(Node::null(Any));
                Ok(node.finish(false))
            }
            Some(tok) => { node.add(self.expr()?); Ok(node.finish(false)) }
            None => {
                self.add_error(ExpectedExprOrSemi, self.loc(0));
                return Err(Failed);
            }
        } 
    }

    pub(crate) fn break_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(BreakExpr, self.bump);
        match self.peek() {
            Some(tok) if matches!(tok.tag, Tag::Semicolon | Tag::RBrace) => {
                node.add(Node::null(Any));
                Ok(node.finish(false))
            }
            Some(tok) => { node.add(self.expr()?); Ok(node.finish(false)) }
            None => {
                self.add_error(ExpectedExprOrSemi, self.loc(0));
                return Err(Failed);
            }
        } 
    }

    pub(crate) fn continue_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> { todo!() }
}