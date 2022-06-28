#![allow(warnings)]

mod lexer;
mod token;
mod utils;

fn main() {
    let mut tokens = lexer::Lexer::from(r#"京""910 + 56 * ("hello楽")"
    I'm on the second line yoooo
    Boiiiiiii
    Lets go guys
    Fifth line gang
    Sixty one pilots
    "#);
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

