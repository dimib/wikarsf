// MD-HTML-RS
// A simple markdown to html converter written in Rust

use std::env;
use std::process;
use md_html::md::{read_content_from_file, parse};
use md_html::html::generator::generate_html;
use md_html::html::generator::write_html;

use crate::args::MdHtmlArgs;

mod args;

fn main() {

    let env_args: Vec<String> = env::args().collect();

    let args = MdHtmlArgs::new(&env_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        show_usage();
        process::exit(1);
    });

    let input_file = args.input.unwrap_or_else(|| {
        process::exit(1);
    });

    let mut content = read_content_from_file(&input_file);
    let tokens = parse(&mut content);

    match tokens {
        Some(tokens) => {
            let html = generate_html(tokens, args.styles);
            match args.output {
                Some(output_file) => { write_html(html, output_file); },
                None => { println!("{}", html) },
            }
        },
        None => {},
    }
}

fn show_usage() {
    println!("Usage: md-html-rs -i <input-file> [-o <output-file>] [--css-href <url>] [--css-file <css-file>] [-v]");
}