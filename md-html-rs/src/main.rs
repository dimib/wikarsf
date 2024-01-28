// MD-HTML-RS
// A simple markdown to html converter written in Rust

use std::collections::LinkedList;
use std::env;
use std::process;
use md_html::md::{read_content_from_file, parse};
use md_html::html::generator::generate_html;
use md_html::html::generator::write_html;
use md_html::html::generator::{StyleType, Pager};
use md_html::common::tokens::Token;
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

    let mut content = read_content_from_file(&input_file)
        .unwrap_or_else(|| {
            println!("Problem reading file: {}", input_file);
            process::exit(1);
        });

    let tokens = parse(&mut content).unwrap_or_else(|| {
        println!("File was empty..?: {}", input_file);
        process::exit(1);
    });

    match args.pages {
        true => {
            write_pages(tokens, args.styles, args.output);
        },
        false => {
            write_one_page(tokens, args.styles, args.output);
        },
    }
}

fn write_one_page(tokens: LinkedList<Token>, style: StyleType, output_file: Option<String>) {
    let html = generate_html(tokens, style, Pager::OnePage);
    match output_file {
        Some(output_file) => { write_html(html, output_file); },
        None => { println!("{}", html) },
    }
}

fn write_pages(tokens: LinkedList<Token>, style: StyleType, output_file: Option<String>) {
    let html = generate_html(tokens, style, Pager::MultiPage);
    match output_file {
        Some(output_file) => { write_html(html, output_file); },
        None => { println!("{}", html) },
    }
}

fn show_usage() {
    println!("Usage: md-html-rs -i <input-file> [-o <output-file>] [--css-href <url>] [--css-file <css-file>] [--pages] [-v]");
}