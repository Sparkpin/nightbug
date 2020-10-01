//! Nightbug is a Lisp-like programming language.
//! Please note that it is a very early work in progress,
//! so the language is likely highly unstable and probably
//! inefficient. Please be careful!

#![feature(once_cell)]
#![feature(or_patterns)]
#![feature(type_alias_impl_trait)]

pub mod errors;
pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let code = "(add 2 (second 3 4))";
    println!("Code: {:?}", code);
    println!();

    let tokens = match lexer::lex(code) {
        Ok(res) => res,
        Err(_) => return
    };
    println!("Tokens: {:?}", tokens);
    let expressions = parser::parse(tokens);
    println!("Expressions: {:?}", expressions);
    println!();

    println!(
        "Result: {:?}",
        interpreter::interpret(expressions.into_iter())
    );
}
