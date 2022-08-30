#![allow(warnings)]
#![feature(test)]

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

const WHOLE_SOURCE: &str = r#"
let PI = 3.14;

if a { return 3; } else { return 4; } * 4;


"#;

fn main() {
    use bumpalo::Bump;

    let bump = Bump::new();

    let mut parser = Parser::new(WHOLE_SOURCE, &bump);

    let tree = parser.parse();

    let mut h = 3;
    let mut d = 3;

    let mut f = d = 3;

    match tree {
        Ok(ast) => {
            let json_tree = serde_json::to_string(&ast).unwrap();
            println!("{}", json_tree);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }

}

fn d(a: bool) -> i32 {
    if a {
        return 3
    } else {
        return 2
    }
}