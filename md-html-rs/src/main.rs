// MD-HTML-RS
// A simple markdown to html converter written in Rust

mod md_html;

use std::env;
use crate::md_html::md::{read_content_from_file, parse};
use crate::md_html::html::generator::generate_html;
use crate::md_html::html::generator::write_html;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Parse file: {:?}", args[1]);
    let input_file = args[1].clone();
    let output_file = args[2].clone();
    let mut content = read_content_from_file(&input_file);
    let tokens = parse(&mut content);

    match tokens {
        Some(tokens) => {
            // tokens.iter().for_each(|t| println!("{:?}", t))
            let html = generate_html(tokens);
            println!("{}", html);

            write_html(html, output_file);
        },
        None => {},
    }
}
