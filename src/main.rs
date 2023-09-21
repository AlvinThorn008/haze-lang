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
pub mod errors;

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

    let bump = Bump::new();

    let mut parser = Parser::new(r#"
fn print(message) {
    let result = stdout.write(message);

    if result { error("Core dumped"); }
}
"#, &bump);

    let tree = parser.parse();

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
