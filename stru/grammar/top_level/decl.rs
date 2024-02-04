use std::f32::consts::E;

use crate::ast::{Node, NodeBuilder, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::Tag;
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub fn module(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume module token
        let mut node = NodeBuilder::from_type(Module, self.bump);

        // Identifiers are required but report error and keep parsing
        let ident = self.expect_token(Tag::Ident); 
        node.add(ident);
        
        let mut decls = NodeBuilder::from_type(TopDeclList, self.bump);

        self.expect_token(Tag::LBrace);

        while let Some(_) = self.peek() {
            match self.top_level_declaration() {
                Ok(decl) => decls.add(decl),
                // Fatal errors always bubble up
                err @ Err(Fatal) => return err,
                // Skip declaration if parsing fails at some point
                Err(_) => continue
            }
        }

        self.expect_token(Tag::RBrace);

        node.add(decls.finish(false));
    
        Ok(node.finish(false))
    }

    /**
     * loop
     *  match ident, if not sync and error
     *  match dot or semi, if not sync and error
     * 
     * loop {
     *  match tok.tag {
     *      Ident if needs_ident => { self.next(); needs_ident = false; }
     *      Dot if !needs_ident => { self.next(); needs_ident = true; }
     *      Semi if !needs_ident => { self.next(); break; }+
     * 
     * }
     * }
     */
    pub fn import(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume import token
        let mut import_path = NodeBuilder::from_type(ImportDecl, self.bump);

        let mut needs_ident = true;

        loop {
            let Some(tok) = self.peek() else {
                self.add_error(Expected(Tag::Ident), self.loc(0));
                return Err(Failed);
            };
            match tok.tag {
                Tag::Ident if needs_ident => { 
                    self.next(); 
                    import_path.add(tok);
                    needs_ident = false; 
                }
                Tag::Dot if !needs_ident => { self.next(); needs_ident = true; }
                Tag::Semicolon if !needs_ident => { self.next(); break; }
                _ => {
                    self.add_error(MalformedImportPath, Loc::from_token(tok));
                    self.decl_synchronize();
                    return Err(Failed);
                }
            }
        }

        // // Error when expected tag is not found
        // if self.peek_is(Tag::Ident) { 
        //     import_path.add(self.next().unwrap());
        // } else {
        //     self.add_error(MalformedImportPath, self.loc(0)); 
        //     self.decl_synchronize();
        //     return Err(Failed);
        // }

        // // There are not many errors we can return here as the syntax is very simple
        // // so return immediate errors and bail out rightafter.
        // loop {
        //     let tok = self.peek().ok_or(Failed)?; // TODO: should error cuz EOF
        //     match tok.tag {
        //         Tag::Dot => { self.next(); },
        //         Tag::Semicolon => { self.next(); break },
        //         _ => {
        //             self.add_error(MalformedImportPath, self.loc(0));
        //             self.add_error(MissingImportDelimeter, Loc::from_token(tok));
        //             self.decl_synchronize();
        //             return Err(Failed);
        //         }
        //     }
        //     let ident = self.expect_token(Tag::Ident);
        //     if ident.is_empty() { 
        //         self.decl_synchronize();
        //         return Err(Failed);
        //     } else { import_path.add(ident); }
        // }

        Ok(import_path.finish(false))
    }

    /// Skip tokens until the another declaration can likely be parsed
    /// 
    /// This method will consume several tokens until a closer(typically a semicolon) (checkpoint) 
    /// or the leading token of a declaration is found.
    /// 
    /// This is a simple implementation of Panic Mode error recovery
    pub(crate) fn decl_synchronize_generic(&mut self, closer: Tag) {
        use Tag::*;
        loop {
            let tag = self.peek().map(|tok| tok.tag); 
            // The `top_level_declaration` method must always peek
            // leading tokens so don't consume them
            match tag {
                Some(t) if t == closer => { self.next(); },
                Some(Module | Import | Enum | Fn | Struct | Type | Const) => { },
                Some(_) => { self.next(); continue; }, 
                None => return // End of file
            }
            break;
        }
    }

    fn decl_synchronize(&mut self) { self.decl_synchronize_generic(Tag::Semicolon) }

    pub(crate) fn type_alias(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume `type` token
        let mut node = NodeBuilder::from_type(TypeAlias, self.bump);

        let ident = self.expect_token(Tag::Ident);
        node.add(ident);

        self.expect_token(Tag::Equal);
        
        let type_value = match self.type_expr() {
            // Type errors would have already reported some errors
            Err(Failed) => { self.decl_synchronize(); return Err(Failed); },
            type_expr => type_expr 
        }?; // <-- Propagate other errors if any

        node.add(type_value);

        Ok(node.finish(false))
    }

    pub(crate) fn const_decl(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume `const` token
        let mut node = NodeBuilder::from_type(ConstDecl, self.bump);

        let ident = self.expect_token(Tag::Ident);
        node.add(ident);

        self.expect_token(Tag::Semicolon);
        let const_type = self.type_expr()?;

        node.add(const_type);

        self.expect_token(Tag::Equal);
        
        let type_value = match self.expr() {
            // Type errors would have already reported some errors
            Err(Failed) => { self.decl_synchronize(); return Err(Failed); },
            type_expr => type_expr 
        }?; // <-- Propagate other errors if any

        node.add(type_value);

        Ok(node.finish(false))
    }
}