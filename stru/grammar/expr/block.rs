use crate::ast::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn block_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(BlockExpr, self.bump);
        let mut stmts = NodeBuilder::from_type(StmtList, self.bump);

        loop {
            match self.peek() {
                Some(tok) if tok.tag == Tag::RBrace => {
                    self.next();
                    break;
                }
                Some(_) => {
                    let statement = self.statement();
                    match statement { 
                        Ok(stmt) => stmts.add(stmt),
                        Err(Failed) => continue,
                        Err(err) => return Err(err)
                    }
                }
                None => {
                    self.add_error(ExpectedRBrace, self.loc(0));
                    self.add_error(UnexpectedEOF, self.loc(0));
                    break;
                }
            }
        }
        node.add(stmts.finish(false));

        Ok(node.finish(false))
    }
}