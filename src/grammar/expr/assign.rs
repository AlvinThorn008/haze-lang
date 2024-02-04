use crate::{ast2::{tag_is_binop, tag_is_unaryop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::{*, self}}, token::Token};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn assign_expr(&mut self, ident: Token<'s>) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(AssignExpr, self.bump);
        node.add(ident);
        let value = self.expr()?;
        node.add(value);

        Ok(node.finish(false))
    }
}