// HTML Generator

use std::cmp::max;
use std::collections::LinkedList;
use std::fs::File;
use std::io::Write;
use crate::common::tokens::Token;

mod styles;
#[derive(Clone)]
pub enum StyleType {
    // Include CSS from file. String = path to file
    Inline(String),

    // Just include a link to a CSS file. String = URL
    External(String),

    // Default style, inlined
    Default,
}

pub enum Pager {
    OnePage,
    MultiPage,
}

pub fn generate_html(tokens: LinkedList<Token>, style: StyleType, pager: Pager) -> String {
    let mut html = String::new();
    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<script src=\"md_html.js\"></script>\n");

    let multi_page = match pager {
        Pager::OnePage => false,
        Pager::MultiPage => true,
    };

    match style {
        StyleType::Inline(path) => {
            match styles::inline_style(path) {
                Ok(style) => {
                    html.push_str(&format!("<style>\n{}\n</style>\n", style));
                },
                Err(_) => {println!("Could not read CSS file")},
            }
        },
        StyleType::External(url) => {
            html.push_str(&format!("<link rel=\"stylesheet\" href=\"{}\">\n", url));
        },
        StyleType::Default => {
            html.push_str(&styles::default_style());
        },
    }
    html.push_str("<title>MD-HTML-RS</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    let mut page_count: i32 = 0;

    for token in tokens {
        match token {
            Token::TagBegin(tag) if tag.name == "h1" && multi_page => {
                println!("Page count: {}", page_count);
                if page_count > 0 {
                    let html_prev= format!("<a href=\"javascript:show_page('page_{}','page_{}')\">Previous</a>\n", page_count-1, max(0, page_count-2));
                    let html_next= format!("<a href=\"javascript:show_page('page_{}','page_{}')\">Next</a>\n", page_count-1, page_count);
                    html.push_str("<hr>");
                    html.push_str("\n<p><div class=\"page_nav\">\n");
                    html.push_str(&html_prev);
                    html.push_str(&html_next);
                    html.push_str("</div></p>\n");
                    html.push_str("\n</div>\n");
                }
                let display = if page_count == 0 { "block" } else { "none" };
                html.push_str(format!("<div id=\"page_{}\" style=\"display:{}\">\n\n", page_count, display).as_str());
                html.push_str(&tag.html);
                page_count += 1;
            },
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
    if multi_page {
        let html_prev= format!("<a href=\"javascript:show_page('page_{}','page_{}')\">Previous</a>", page_count-1, page_count-2);
        html.push_str("\n<p><div class=\"page_nav\">\n");
        html.push_str(&html_prev);
        html.push_str("</div></p>\n");
        html.push_str("</div>\n");
    }
    html.push_str("</body>\n");
    html.push_str("</html>\n");
    html
}

pub fn write_html(content: String, path: String) {
    let mut file = File::create(path).expect("Could not create file");
    file.write_all(content.as_bytes()).expect("Could not write to file");
}