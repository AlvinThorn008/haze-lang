use crate::ast::{tag_is_binop, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
use crate::grammar::types::*;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn statement(&mut self) -> Result<NodeChild<'s, 'b>, ParsingError> {
        let tok = self.peek().unwrap();

        match tok.tag {
            Tag::Let => self.var_decl().map(NodeChild::Node),
            Tag::Semicolon => {
                self.next();
                Ok(tok.into())
            }
            _ => self.expr_stmt().map(NodeChild::Node)
        }
    }

    fn expr_stmt(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut node = NodeBuilder::from_type(ExprStmt, self.bump);
        let expr = match self.expr() {
            Ok(expr) => expr,
            Err(Failed) => { self.stmt_synchronize(); return Err(Failed); },
            Err(err) => return Err(err)
        };

        match &expr {
            NodeChild::Node(node) => match node.kind.0 {
                IfExpr | WhileExpr | BlockExpr => self.lazy_eat(Tag::Semicolon),
                _ => { self.expect_token(Tag::Semicolon); }
            }
            _ => { self.expect_token(Tag::Semicolon); }
        }

        node.add(expr);
        Ok(node.finish(false))
    }

    fn var_decl(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume `let`` token
        
        let mut node = NodeBuilder::from_type(VarDecl, self.bump);

        let ident = self.expect_token(Tag::Ident);
        node.add(ident);

        if !self.expect_token(Tag::Colon).is_empty() { 
            self.next();
            node.add(self.type_expr()?);
        } else { node.add(Node::null(Any)); }

        if !self.expect_token(Tag::Equal).is_empty() {
            self.next();
            node.add(self.expr()?);
        } else { node.add(Node::null(Any)); }

        if self.peek_is(Tag::Semicolon) { self.next(); return Ok(node.finish(false)) }

        match self.stmt_synchronize() {
            SyncStatus::FoundSemi => { self.next(); Err(Failed) }
            SyncStatus::FoundLeading => {
                self.add_error(ExpectedSemi, Loc::from_token(self.peek().unwrap()));
                Err(Failed)
            }
            SyncStatus::EOF => {
                self.add_error(UnexpectedEOF, self.loc(0));
                Err(Failed)
            }
        }
    }

    fn stmt_synchronize(&mut self) -> SyncStatus {
        use Tag::*;
        use SyncStatus::*;
        loop {
            let tag = self.peek().map(|tok| tok.tag); 
            match tag {
                Some(Semicolon) => { self.next(); return FoundSemi; },
                // Leading tokens for expressions as the next statement
                // may well be an expression statement.
                Some(
                    | Let | String | Number | Bool | Minus 
                    | Bang | Break | Return | LParen | LBrace 
                    | LParen | If) => { return FoundLeading; }
                Some(_) => { self.next(); continue; }, 
                None => return EOF // End of file
            }
        }
    }
}

enum SyncStatus {
    FoundSemi,
    FoundLeading,
    EOF
}