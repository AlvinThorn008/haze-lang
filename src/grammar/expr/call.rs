use crate::ast2::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::peek_matches;
use crate::token::{Tag, Token};
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub fn call_expr(&mut self, ident: Token<'s>) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(CallExpr, self.bump);
        let mut args = NodeBuilder::from_type(ArgList, self.bump);

        node.add(ident);
        loop {
            let arg = self.expr_with_allower(|tok| matches!(tok.tag, Tag::RParen | Tag::Comma))?;
            args.add(arg);

            match self.peek() {
                Some(tok) => match tok.tag {
                    Tag::Comma => { self.next(); continue; },
                    Tag::RParen => { self.next(); break; },
                    _ => {
                        self.add_error(Expected(Tag::RParen), Loc::from_token(tok));
                        return Err(Failed);
                    }
                }
                _ => {
                    self.add_error(Expected(Tag::RParen), self.loc(0));
                    return Err(Failed);
                }
            }
        }
        node.add(args.finish(false));
        Ok(node.finish(false))
    }
}