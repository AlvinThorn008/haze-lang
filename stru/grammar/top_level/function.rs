use crate::ast::{Node, NodeBuilder, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn function_def(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume fn token
        let mut node = NodeBuilder::from_type(FnDef, self.bump);

        let ident = self.expect_token(Tag::Ident);

        let parameters;

        match self.peek() {
            Some(tok) => match tok.tag {
                Tag::LParen => { self.next(); parameters = self.function_params()? },
                _ => {
                    self.add_error(ExpectedFunctionParameters, Loc::from_token(tok));
                    return Err(Failed);
                }
            }
            None => {
                self.add_error(ExpectedFunctionParameters, self.loc(0));
                self.add_error(UnexpectedEOF, self.loc(0));
                return Err(Failed); 
            }
        }

        let return_expr;
        if self.peek_is(Tag::Arrow) { self.next(); }
        return_expr = match self.type_expr() {
            Ok(node) => node,
            Err(Failed) => Node::null(Any).into(),
            Err(err) => return Err(err)
        }; 

        // Note to self: Look at the order of fields defined in 
        // `language_nodes copy.txt`. Don't modify otherwise
        let body = self.block_expr()?;

        node.add(body); node.add(return_expr);

        Ok(node.finish(false))
    }

    fn function_params(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut params = NodeBuilder::from_type(ParamList, self.bump);

        if self.peek_is(Tag::RParen) { return Ok(params.finish(false)); }

        loop {
            let mut param = NodeBuilder::from_type(Param, self.bump);
            let ident = self.expect_token(Tag::Ident);
            param.add(ident);

            let tok = self.peek().ok_or(Failed)?;
            match tok.tag {
                Tag::Colon => { 
                    self.next();
                    let param_type = self.type_expr()?;
                    param.add(param_type);
                }
                _ => {
                    self.add_error(ParamIncomplete, Loc::from_token(tok));
                    self.add_error(ExpectedColon, Loc::from_token(tok));
                    use SyncStatus::*;
                    match self.param_synchronize() {
                        FoundComma => continue,
                        FoundClosingParen => return Ok(params.finish(false)),
                        EOF => return Err(Failed)
                    }
                }
            }

            params.add(param.finish(false));

            if self.peek_is(Tag::Comma) { continue; }
            if self.peek_is(Tag::RParen) { break; }
        }

        Ok(params.finish(false))
    }

    fn param_synchronize(&mut self) -> SyncStatus {
        use Tag::*;
        use SyncStatus::*;
        loop {
            let tag = self.peek().map(|tok| tok.tag); 
            match tag {
                Some(Comma) => { return FoundComma; },
                Some(RParen) => { return FoundClosingParen; }
                Some(_) => { self.next(); continue; }, 
                None => return EOF // End of file
            }
        }
    }
}

enum SyncStatus {
    FoundComma,
    FoundClosingParen,
    EOF
}