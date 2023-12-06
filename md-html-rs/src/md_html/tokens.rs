// MD-Tokens
// A simple markdown token reader written in Rust

use std::str;

pub struct Tag {
    pub name: Box<str>,
    pub token: Box<str>,
    pub ends: Box<str>,
    pub html: Box<str>,
    pub html_ends: Box<str>,
}

pub enum Token {
    TagBegin,
    TagEnd,
    Text,
}

pub const tags: [Tag] = [
    Tag { name: "h1", token: "# ", ends: "\n", html: "<h1>", html_ends: "</h1>" },
    Tag { name: "h2", token: "## ", ends: "\n", html: "<h2>", html_ends: "</h2>" },
    Tag { name: "h3", token: "### ", ends: "\n", html: "<h3>", html_ends: "</h3>" },
    Tag { name: "h4", token: "#### ", ends: "\n", html: "<h4>", html_ends: "</h4>" },
    Tag { name: "h5", token: "##### ", ends: "\n", html: "<h5>", html_ends: "</h5>" },
    Tag { name: "h6", token: "###### ", ends: "\n", html: "<h6>", html_ends: "</h6>" },
    Tag { name: "code", token: "`", ends: "`", html: "<code>", html_ends: "</code>" },
    Tag { name: "italic", token: "*", ends: "*", html: "<i>", html_ends: "</i>" },
    Tag { name: "italic", token: "_", ends: "_", html: "<i>", html_ends: "</i>" },
    Tag { name: "bold", token: "**", ends: "**", html: "<b>", html_ends: "</b>" },
    Tag { name: "bold", token: "__", ends: "__", html: "<b>", html_ends: "</b>" },
    Tag { name: "bolditalic", token: "***", ends: "***", html: "<b><i>", html_ends: "</i></b>" },
    Tag { name: "bolditalic", token: "___", ends: "___", html: "<b><i>", html_ends: "</i></b>" },
    Tag { name: "del", token: "~~", ends: "~~", html: "<del>", html_ends: "</del>" },
];



