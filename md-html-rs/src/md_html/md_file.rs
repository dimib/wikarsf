// MD-FILE
// A simple markdown file reader written in Rust


use std::fs;

mod tokens;

// Read a file and return a MdContent struct.
pub fn read(path: &String) -> MdContent {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    MdContent {
        content: file_content.clone(),
        content_len: file_content.len() as i64,
        index: 0,
    }
}

// MD Content Reader trait
// Must be implemented by any struct that wants to read MD content
pub trait MdContentReader {
    fn next_line(&mut self) -> String;
    fn next_char(&mut self) -> char;
    fn remaining(&mut self) -> i64;
    fn next_token(&mut self) -> Token;
}
pub struct MdContent {
    pub content: String,
    pub content_len: i64,
    pub index: i64,
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

    fn remaining(&mut self) -> i64 {
        self.content_len - self.index
    }

    fn next_token(&mut self) -> Token {
        Token::Text("".to_string())
    }
}

