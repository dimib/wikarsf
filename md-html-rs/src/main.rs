// MD-HTML-RS
// A simple markdown to html converter written in Rust

use std::env;
mod md_html;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Parse file: {:?}", args[1]);
    md_html::md_parser(&args[1].clone());
}
