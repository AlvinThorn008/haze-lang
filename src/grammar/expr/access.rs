use crate::ast2::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::peek_matches;
use crate::token::{Tag, Token};
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub fn index_expr(&mut self, ident: Token<'s>) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(IndexExpr, self.bump);
        node.add(ident);
        let index = self.expr_with_allower(|tok| tok.tag == Tag::RBracket)?;
        node.add(index);

        if self.expect_token(Tag::RBracket).is_empty() {
            self.expr_synchronize(Tag::RBracket);
            return Err(Failed)
        } else {
            return Ok(node.finish(false));
        }
    }

    pub fn field_access_expr(&mut self, ident: Token<'s>) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(FieldAccessExpr, self.bump);
        node.add(ident);
        let field_name = self.expect_token(Tag::Ident);
        if field_name.is_empty() { return Err(Failed); }

        node.add(field_name);
        Ok(node.finish(false))
    }
}