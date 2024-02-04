use crate::ast2::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::peek_matches;
use crate::token::Tag;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub fn array_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut array = NodeBuilder::from_type(ArrayExpr, self.bump);
        loop {
            let expr = self.expr()?;
            array.add(expr);

            match self.peek() {
                Some(tok) => match tok.tag {
                    Tag::Comma => { self.next(); continue; },
                    Tag::RBracket => { self.next(); break; },
                    _ => {
                        self.add_error(ExpectedArrayDelimeter, Loc::from_token(tok));
                        return Err(Failed);
                    }
                }
                _ => {
                    self.add_error(ExpectedArrayDelimeter, self.loc(0));
                    return Err(Failed);
                }
            }
        }
        Ok(array.finish(false))
    }
}