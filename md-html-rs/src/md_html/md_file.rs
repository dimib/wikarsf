// MD-FILE
// A simple markdown file reader written in Rust

use std::{fs, string};
use crate::md_html::tokens::{Token, Tag};
use crate::md_html::tokens;

// Read a file and return a MdContent struct.
pub fn read(path: &String) -> MdContent {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    MdContent {
        content: file_content.clone(),
        content_len: file_content.len(),
        index: 0,
    }
}

// MD Content Reader trait
// Must be implemented by any struct that wants to read MD content
pub trait MdContentReader {
    fn next_line(&mut self) -> String;
    fn next_char(&mut self) -> char;
    fn remaining(&mut self) -> usize;
    fn next_token(&mut self) -> Option<Token>;
}
pub struct MdContent {
    pub content: String,
    pub content_len: usize,
    pub index: usize,
}

impl MdContentReader for MdContent {

    fn next_line(&mut self) -> String {
        let mut line = String::new();
        let mut c = self.next_char();
        while c != '\n' && c != '\0' {
            line.push(c);
            c = self.next_char();
        }
        line
    }

    fn next_char(&mut self) -> char {
        if self.index < self.content_len {
            let c = self.content.as_bytes()[self.index as usize] as char;
            self.index += 1;
            c
        } else {
            '\0'
        }
    }

    fn remaining(&mut self) -> usize {
        self.content_len - self.index
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut string = String::new();

        let start_tag = find_start_tag_at_index(&self.content, self.index as usize);
        match start_tag {
            Some(tag) => {
                self.index += tag.token.len();
                return Some(Token::TagBegin(tag));
            },
            None => {
                return None;
            }
        }
    }
}

// Try to find one of the supported start tags at the given index.
fn find_start_tag_at_index(content: &String, index: usize) -> Option<&Tag> {
    let mut tag: Option<&Tag> = None;

    for t in tokens::TAGS.iter() {
        if content[index..].starts_with(&t.token) {
            tag = Some(t);
            break;
        }
    }
    tag
}

// Try to find one of the supported end tags at the given index.
fn find_end_tag_at_index(content: &String, index: usize) -> Option<&Tag> {
    let mut tag: Option<&Tag> = None;

    for t in tokens::TAGS.iter() {
        if content[index..].starts_with(&t.ends) {
            tag = Some(t);
            break;
        }
    }
    tag
}
