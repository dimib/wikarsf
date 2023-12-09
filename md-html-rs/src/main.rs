// MD-HTML-RS
// A simple markdown to html converter written in Rust

use std::env;
mod md_parser;

use crate::md_parser::{read_content_from_file, parse, tokens};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Parse file: {:?}", args[1]);
    let mut content = read_content_from_file(&args[1].clone());
    let tokens = parse(&mut content);

    match tokens {
        Some(tokens) => {
            tokens.iter().for_each(|t| println!("{:?}", t))
        },
        None => {},
    }
}
