// MD-PARSER
// A simple markdown parser written in Rust

mod md_file;
mod tokens;

enum ParserState {
    WaitForBegin,
    WaitForNewLine,
}

use md_file::MdContentReader;
use md_file::read;
pub fn md_parser(path: &String) {
    let mut md_content = read(path);
    println!("File contents:\n{}", md_content.content);

    while md_content.remaining() > 0 {
        let line = md_content.next_line();
        println!("Line: {}", line);
    }
}

