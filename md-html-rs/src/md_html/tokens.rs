// MD-Tokens
// A simple markdown token reader written in Rust

pub struct Tag {
    pub name: String,
    pub token: String,
    pub ends: String,
    pub html: String,
    pub html_ends: String,
}

pub enum Token<'a> {
    TagBegin(&'a Tag),
    TagEnd(&'a Tag),
    Text(String),
}

macro_rules! S {
    ($expression:expr) => {
        String::from($expression)
    };
}

pub static TAGS: Vec<Tag> = Vec::new();

pub fn makeTags() -> Vec<Tag> {
    return vec![
        Tag { name: S!("h1"), token: S!("# "), ends: S!("\n"), html: S!("<h1>"), html_ends: S!("</h1>") },
        Tag { name: S!("h2"), token: S!("## "), ends: S!("\n"), html: S!("<h2>"), html_ends: S!("</h2>") },
        Tag { name: S!("h3"), token: S!("### "), ends: S!("\n"), html: S!("<h3>"), html_ends: S!("</h3>") },
        Tag { name: S!("h4"), token: S!("#### "), ends: S!("\n"), html: S!("<h4>"), html_ends: S!("</h4>") },
        Tag { name: S!("h5"), token: S!("##### "), ends: S!("\n"), html: S!("<h5>"), html_ends: S!("</h5>") },
        Tag { name: S!("h6"), token: S!("###### "), ends: S!("\n"), html: S!("<h6>"), html_ends: S!("</h6>") },
        Tag { name: S!("code"), token: S!("`"), ends: S!("`"), html: S!("<code>"), html_ends: S!("</code>") },
        Tag { name: S!("italic"), token: S!("*"), ends: S!("*"), html: S!("<i>"), html_ends: S!("</i>") },
        Tag { name: S!("italic"), token: S!("_"), ends: S!("_"), html: S!("<i>"), html_ends: S!("</i>") },
        Tag { name: S!("bold"), token: S!("**"), ends: S!("**"), html: S!("<b>"), html_ends: S!("</b>") },
        Tag { name: S!("bold"), token: S!("__"), ends: S!("__"), html: S!("<b>"), html_ends: S!("</b>") },
        Tag { name: S!("bolditalic"), token: S!("***"), ends: S!("***"), html: S!("<b><i>"), html_ends: S!("</i></b>") },
        Tag { name: S!("bolditalic"), token: S!("___"), ends: S!("___"), html: S!("<b><i>"), html_ends: S!("</i></b>") },
        Tag { name: S!("del"), token: S!("~~"), ends: S!("~~"), html: S!("<del>"), html_ends: S!("</del>") },
    ];
}


