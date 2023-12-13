// MD-PARSER
// A simple markdown parser written in Rust

pub mod content;
pub mod file;
pub mod tokenizer;

use std::collections::LinkedList;
use content::MdContent;
use tokenizer::tokenize;
use super::common::tokens::Token;

use file::read;

pub fn read_content_from_file(path: &String) -> Option::<MdContent> {
    let md_content = read(path);
    md_content
}

pub fn parse(content: &mut MdContent) -> Option<LinkedList<Token>> {
    let token = tokenize(content);
    Some(token)
}

pub fn create_content(content: String) -> MdContent {
    MdContent::new(content)
}

