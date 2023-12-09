// MD-HTML-RS
// A simple markdown to html converter written in Rust

mod md_html;

use std::env;
use crate::md_html::md::{read_content_from_file, parse};

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
