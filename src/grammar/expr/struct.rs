use crate::ast2::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::peek_matches;
use crate::token::Tag;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub fn struct_expr(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(StructExpr, self.bump);
        
        // Identifiers are required but report error and keep parsing
        let struct_name = self.expect_token(Tag::Ident); 
        node.add(struct_name);

        self.expect_token(Tag::LBrace);

        let mut fields = NodeBuilder::from_type(FieldInitList, self.bump);

        // True if an error we don't want to recover from occurs
        let mut failed = false;

        loop {
            match self.field_init() {
                Ok(node) => fields.add(node),
                Err(Terminal) => { 
                    self.decl_synchronize_generic(Tag::RBrace);
                    return Err(Failed); 
                },
                err => return err
            }
            let Some(tok) = self.peek() else { 
                self.add_error(UnexpectedEOF, self.loc(0)); return Err(Failed); 
            };
            match tok.tag {
                Tag::Comma => { self.next(); continue; },
                Tag::RBrace => { self.next(); break; },
                _ => {
                    self.add_error(MissingFieldDelimeter, Loc::from_token(tok));
                    return Err(Failed);
                }
            }
        }

        node.add(fields.finish(false));

        return Ok(node.finish(false));
    }

    fn field_init(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut field = NodeBuilder::from_type(FieldInit, self.bump);

        let field_name = self.expect_token(Tag::Ident);
        if field_name.is_empty() || 
            self.expect_token(Tag::Colon).is_empty() { return Err(Terminal); }

        let field_expr = match self.expr_with_allower(|tok| matches!(tok.tag, Tag::Comma | Tag::RBrace)) {
            Ok(expr) => expr,
            Err(Failed) => return Err(Terminal),
            Err(err) => return Err(err)
        };

        field.add(field_name); field.add(field_expr);
        Ok(field.finish(false))
    }
}