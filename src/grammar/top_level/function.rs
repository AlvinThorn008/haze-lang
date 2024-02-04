use crate::ast2::{Node, NodeBuilder, NodeKind, NodeType::*, NodeChild};
use crate::errors::Loc;
use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
use crate::token::{Tag, Token};
pub use super::Parser;

impl<'s, 'b> Parser<'s, 'b> {
    pub(crate) fn function_def(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        self.next(); // consume fn token
        let mut node = NodeBuilder::from_type(FnDef, self.bump);

        let ident = self.expect_token(Tag::Ident);
        node.add(ident);

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

        node.add(parameters);

        let mut return_expr = NodeChild::Node(Node::null(Any));
        if self.peek_is(Tag::Arrow) { 
            self.next(); 
            return_expr = self.type_expr()?;
        }
    
        let mut body = NodeChild::Node(Node::null(Any));
        if self.peek_is(Tag::LBrace) { self.next(); body = self.block_expr()?.into(); }
        // Note to self: Look at the order of fields defined in 
        // `language_nodes copy.txt`. Don't modify otherwise

        node.add(body); node.add(return_expr);

        Ok(node.finish(false))
    }

    fn function_params(&mut self) -> Result<Node<'s, 'b>, ParsingError> {
        let mut params = NodeBuilder::from_type(ParamList, self.bump);

        if self.peek_is(Tag::RParen) { self.next(); return Ok(params.finish(false)); }
 
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
                        FoundComma => { self.next(); continue; },
                        FoundClosingParen => { self.next(); return Ok(params.finish(false)) },
                        EOF => return Err(Failed)
                    }
                }
            }

            params.add(param.finish(false));

            if self.peek_is(Tag::Comma) { self.next(); continue; }
            if self.peek_is(Tag::RParen) { self.next(); break; }
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

#[cfg(test)]
mod tests {
    use bumpalo::Bump;

    use super::*;
    use crate::ast2::{FnDef, AstNode, AstToken, Ident};

    #[test]
    fn test_fn() {
        let bump = Bump::new();
        let mut parser = Parser::new(r#"
fn do_something(num: int, num2: int) -> int {
    let name = "Help me";
    let j: int = 1 + bring(3);

    let k = .Foo { bar: (3), foo: 2342 - 24843 };

    if j > 6 {
        name = "Help you";
    } else { name = "uoypleh"; }

    let j = (if k.bar > 2 { return 45; } else { return 54; })
        + bring(3) - k.foo;

    j = 34;
    return num + 2;
}

fn bring(num: int) {
    return num - 3;
}

"#, &bump);

        let result = parser.function_def();
        println!("{:#}", if let Ok(node) = result { node } else { Node::null(Any) });
        parser.errors.iter().for_each(|err| println!("{:?}", err));
        
    }

    #[test]
    fn test_fn_set() {
        let bump = Bump::new();
        let mut parser = Parser::new(r#"
fn binary_search(list: [int; 20], target: int) -> int {
    let low = 0;
    let high = 20 - 1;
    let mid = (low + high) / 2;

    while low < high + 1 {
        let mid_val = list[mid];
        if mid_val == target { return mid; } 
        else if mid_val < target { low = mid + 1; } 
        else { high = mid - 1; }

        mid = (low + high) / 2;
    }
    return -1;
}

fn log_message(msg: string, severity: uint) {

    let output = "";
    if severity > 2 {
        output = append("Error: ", msg);
    } else {
        output = append("Warning: ", msg);
    }
    println(output);
}

fn main() {
    println("Hello, world!");
}

fn (bytes: [u8; 10], num: int) -> bool {}

fn (:[u8; 10], num: int) -> bool {}

fn foo(a, b, c, d) {
    println(a + b + c + d);
}

"#, &bump);

        let result = parser.parse();
        println!("{:#}", result);

        parser.debug_errors();
    }
}

/*
fn binary_search(list: [int; 20], target: int) -> int {
    let low = 0;
    let high = 20 - 1;
    let mid = (low + high) / 2;

    while low < high + 1 {
        let mid_val = list[mid];
        if mid_val == target { return mid; } 
        else if mid_val < target { low = mid + 1; } 
        else { high = mid - 1; }

        mid = (low + high) / 2;
    }
    return -1;
}

fn log_message(msg: string, severity: uint) {

    let output;
    if severity > 2 {
        output = append("Error: ", msg);
    } else {
        output = append("Warning: ", msg);
    }
    println(output);
}

fn main() {
    println("Hello, world!");
}

fn (bytes: [u8; 10], num: int) -> bool {}

fn (:[u8, 10], num: int) -> bool {}

fn foo(a, b, c, d) {
    println(a + b + c + d);
} */