use crate::ast2::{Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
pub use crate::parser3::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn type_expr(&mut self) -> Result<NodeChild<'s, 'b>, ParsingError> {
        match self.peek() {
            Some(tok) => match tok.tag {
                Tag::LBracket => { self.next(); self.array_type().map(NodeChild::Node) }
                Tag::LParen => { 
                    self.next(); 
                    let mut node = NodeBuilder::from_type(GroupType, self.bump);
                    let inner = self.type_expr()?;
                    node.add(inner);
                    if self.expect_token(Tag::RParen).is_empty() { 
                        return Err(Failed);
                    } else { Ok(node.finish(false).into()) }

                }
                Tag::Ident => { self.next(); Ok(NodeChild::Token(tok)) }
                _ => {
                    self.add_error(ExpectedType, self.loc(0));
                    return Err(Failed);
                }

            }
            None => {
                self.add_error(ExpectedType, self.loc(0));
                return Err(Failed);
            }
        }
    }

    pub(crate) fn array_type(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(ArrayType, self.bump);

        node.add(self.type_expr()?);
        self.expect_token(Tag::Semicolon);
        node.add(self.expect_token(Tag::Number));

        self.expect_token(Tag::RBracket);

        return Ok(node.finish(false));
    }
}