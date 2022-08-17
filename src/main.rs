#![allow(warnings)]

use std::str::CharIndices;

use crate::{parser::Parser};

pub mod lexer;
pub mod token;
pub mod utils;
pub mod ast;
pub mod parser;
pub mod bumping;

/* fn main() {
    let mut tokens = lexer::Lexer::from(r#"京""910 + 56 * ("hello楽")"
    I'm on the second line yoooo
    Lets go guys
    Fifth line gang
&    "#);
    let mut token_list = Vec::new();
    while let Some(tok) = tokens.next() {
        println!("{:?}", tok);
        token_list.push(tok);
    }

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        //println!("{:?}", input);
        if input.eq("end") { break; }
        let mut idxs_iter = input
            .split("..")
            .map(|str| {
                str.parse::<usize>().unwrap()
            });
        let idxs = idxs_iter.next().unwrap()..idxs_iter.next().unwrap();

        println!("{}", &tokens.src[idxs]);
    }
}

 */

fn main() {
    use bumpalo::Bump;

    // println!("{}", std::mem::size_of::<crate::ast::Node<'_, '_>>());

    // return;

    let bump = Bump::new();

    let mut parser = Parser::new("
    fn drake(name, hate, bit) {
        let h = \"Hi world!\";

        {
            let g = 45;
            g + 5;
        }
    }
    ", &bump);

    let tree = parser.parse();

    match tree {
        Ok(ast) => {
            let json_tree = serde_json::to_string(&ast).unwrap();
            println!("{}", json_tree);
        }
        Err(err) => {
            return ()
        }
    }


}