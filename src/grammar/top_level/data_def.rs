use crate::ast2::{Node, NodeBuilder, NodeKind, NodeType::*};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::{Tag, Token};
pub use super::Parser;
use crate::peek_matches;

impl<'s, 'b> Parser<'s, 'b> {
    pub fn struct_decl(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume struct token

        let mut node = NodeBuilder::from_type(StructDecl, self.bump);
        
        // Identifiers are required but report error and keep parsing
        let struct_name = self.expect_token(Tag::Ident); 
        node.add(struct_name);

        self.expect_token(Tag::LBrace);

        let mut fields = NodeBuilder::from_type(FieldList, self.bump);

        // True if an error we don't want to recover from occurs
        let mut failed = false;

        loop {
            match self.struct_field() {
                Ok(node) => fields.add(node),
                Err(Terminal) => { 
                    self.decl_synchronize_generic(Tag::RBrace);
                    return Err(Failed); 
                },
                Err(CouldRecover) => {}
                err => return err
            }
            let Some(tok) = self.peek() else { 
                self.add_error(UnexpectedEOF, self.loc(0)); return Err(Failed); 
            };
            match tok.tag {
                Tag::Comma => { 
                    self.next(); 
                    if self.peek_is(Tag::RBrace) { self.next(); break; } 
                    else { continue; }
                },
                Tag::RBrace => { self.next(); break; },
                _ => {
                    self.add_error(MissingFieldDelimeter, Loc::from_token(tok));
                    self.decl_synchronize_generic(Tag::RBrace);
                    return Err(Failed);
                }
            }
        }

        node.add(fields.finish(false));

        return Ok(node.finish(false));
    }

    fn struct_field(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut field = NodeBuilder::from_type(Field, self.bump);

        let field_name = self.expect_token(Tag::Ident);
        if field_name.is_empty() || 
            self.expect_token(Tag::Colon).is_empty() { return Err(Terminal); }

        let field_type = match self.type_expr() {
            Ok(expr) => expr,
            // `Parser::type_expr` would have already reported some errors
            Err(Failed) => if peek_matches!(self, Tag::Comma | Tag::RBrace) { return Err(CouldRecover); }
                else { return Err(Terminal); }
            Err(err) => return Err(err)
        };

        field.add(field_name); field.add(field_type);
        Ok(field.finish(false))
    }
}

#[cfg(test)]
mod tests {
    use bumpalo::Bump;

    use super::*;
    use crate::ast2::{StructDecl, AstNode, AstToken, Field, Ident};

    #[test]
    fn test_struct_decl_valid() {
        let bump = Bump::new();
        let mut parser = Parser::new("
struct Flags { 
    visible: bool,
    ready: bool,
}
", &bump);

        let result = parser.struct_decl();
        assert!(result.is_ok());
        let node = bump.alloc(result.unwrap());

        let result = StructDecl::cast(&node);
        
        assert!(result.name().token().value == "Flags");
        let mut fields = result.fields()
            .items()
            .map(|field| (field.name().token().value, field.field_type().token().value));
        assert!(fields.eq([("visible", "bool"), ("ready", "bool")]));
    }

    #[test]
    fn test_struct_parse_error() {
        let bump = Bump::new();
        let mut parser = Parser::new("struct Flags { : bool, ready: bool }", &bump);

        let result = parser.struct_decl();
        parser.errors.iter().for_each(|err| println!("{:?}", err));
        
    }
}