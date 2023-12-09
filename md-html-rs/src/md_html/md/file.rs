// MD-FILE
// A simple markdown file reader written in Rust

use std::fs;
use super::content::MdContent;

// Read a file and return a MdContent struct.
pub fn read(path: &String) -> MdContent {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    MdContent {
        content: file_content.clone(),
        content_len: file_content.len(),
        index: 0,
    }
}
