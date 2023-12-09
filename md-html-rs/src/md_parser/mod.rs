// MD-PARSER
// A simple markdown parser written in Rust

pub mod content;
pub mod file;
pub mod tokens;
mod tokenizer;

use std::collections::LinkedList;
use content::MdContent;
use tokenizer::tokenize;
use tokens::Token;

use file::read;

pub fn read_content_from_file(path: &String) -> MdContent {
    let md_content = read(path);
    println!("File contents:\n{}", md_content.content);
    md_content
}

pub fn parse(content: &mut MdContent) -> Option<LinkedList<Token>> {
    let mut md_content = &content;
    let token = tokenize(content);
    Some(token)
}

