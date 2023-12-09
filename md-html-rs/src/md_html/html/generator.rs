// HTML Generator

use std::collections::LinkedList;
use crate::md_html::common::tokens::Token;

pub fn generate_html(tokens: LinkedList<Token>) -> String {
    let mut html = String::new();
    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<link rel=\"stylesheet\" href=\"md_html.css\">\n");
    html.push_str("<title>MD-HTML-RS</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    for token in tokens {
        match token {
            Token::TagBegin(tag) => {
                html.push_str(&tag.html);
            },
            Token::TagEnd(tag) => {
                html.push_str(&tag.html_ends);
            },
            Token::Text(text) => {
                html.push_str(&text);
            },
            Token::Image(alt, src) => {
                html.push_str(&format!("<img src=\"{}\" alt=\"{}\" />", src, alt));
            },
            Token::Link(text, href) => {
                html.push_str(&format!("<a href=\"{}\" target=\"_\">{}</a>", href, text));
            },
        }
    }

    html.push_str("</body>\n");
    html.push_str("</html>\n");
    html
}

pub fn write_html(content: String, path: String) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path).expect("Could not create file");
    file.write_all(content.as_bytes()).expect("Could not write to file");
}