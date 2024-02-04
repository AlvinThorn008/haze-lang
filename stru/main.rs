#![allow(warnings)]
#![feature(allocator_api)]
#![feature(test)]

use std::str::CharIndices;

use crate::{parser2::Parser};

pub mod lexer;
pub mod token;
pub mod utils;
pub mod ast;
pub mod ast;
pub mod parser;
pub mod bumping;
pub mod errors;

pub mod grammar;
pub mod typecheck;
pub mod visitor;

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

fn area_circle(radius) {
    return PI * radius * radius;
}

let radius = int(input("What is the radius"));
print(area_circle(radius));

let students = [];

while true {
    print("Enter student record");
    let name = input("Student Name: ");
    let age = int(input("Student Age: "));
    let class = input("Student's class");


    if age > 18 {
        return print("This person is too old");
    }
    if class.len() > 3 { return print("Invalid class name"); }

    students.push((name, age, class));

    if input("Exit? ") {
        return print("Exiting record system")
    }
}
"#;

fn main() {
    use bumpalo::Bump;

    type g = fn (i: i32) -> i32;

    fn h(e: i32) -> i32 { 0 }

    let j: g = h;

    let bump = Bump::new();

    let mut parser = Parser::new(r#"
fn message(id) {
}
"#, &bump);

    let mut g = 4;
    let h = g = 2;

    let j = (34);

    enum D { I() }

    mod DE {}

    let h = D::I();

    let tree = parser.parse();

    match tree {
        Ok(ast) => {
            let json_tree = serde_json::to_string(&ast).unwrap();
            println!("{}", json_tree);
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }

}
