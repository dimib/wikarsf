use once_cell::sync::Lazy;

// MD-Tokens
// A simple markdown token reader written in Rust
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub token: String,
    pub ends: String,
    pub html: String,
    pub html_ends: String,
    pub ignore_tags: bool
}

#[derive(Debug)]
pub enum Token {
    TagBegin(Tag),
    TagEnd(Tag),
    Text(String),
}

macro_rules! S {
    ($expression:expr) => {
        String::from($expression)
    };
}

pub static TAGS: Lazy<Vec<Tag>> = Lazy::new(|| {
    let mut tags: Vec<Tag> = Vec::new();
    tags.push(Tag { name: S!("h1"), token: S!("# "), ends: S!("\n"), html: S!("<h1>"), html_ends: S!("</h1>"), ignore_tags: false });
    tags.push(Tag { name: S!("h2"), token: S!("## "), ends: S!("\n"), html: S!("<h2>"), html_ends: S!("</h2>"), ignore_tags: false });
    tags.push(Tag { name: S!("h3"), token: S!("### "), ends: S!("\n"), html: S!("<h3>"), html_ends: S!("</h3>"), ignore_tags: false });
    tags.push(Tag { name: S!("h4"), token: S!("#### "), ends: S!("\n"), html: S!("<h4>"), html_ends: S!("</h4>"), ignore_tags: false });
    tags.push(Tag { name: S!("h5"), token: S!("##### "), ends: S!("\n"), html: S!("<h5>"), html_ends: S!("</h5>"), ignore_tags: false });
    tags.push(Tag { name: S!("h6"), token: S!("###### "), ends: S!("\n"), html: S!("<h6>"), html_ends: S!("</h6>"), ignore_tags: false });
    tags.push(Tag { name: S!("codeblock"), token: S!("```"), ends: S!("```"), html: S!("<code>\n"), html_ends: S!("\n</code>"), ignore_tags: true });
    tags.push(Tag { name: S!("code"), token: S!("`"), ends: S!("`"), html: S!("<code>"), html_ends: S!("</code>"),ignore_tags: true });
    tags.push(Tag { name: S!("bolditalic"), token: S!("***"), ends: S!("***"), html: S!("<b><i>"), html_ends: S!("</i></b>"), ignore_tags: false });
    tags.push(Tag { name: S!("bolditalic"), token: S!("___"), ends: S!("___"), html: S!("<b><i>"), html_ends: S!("</i></b>"), ignore_tags: false });
    tags.push(Tag { name: S!("bold"), token: S!("**"), ends: S!("**"), html: S!("<b>"), html_ends: S!("</b>"), ignore_tags: false });
    tags.push(Tag { name: S!("bold"), token: S!("__"), ends: S!("__"), html: S!("<b>"), html_ends: S!("</b>"), ignore_tags: false });
    tags.push(Tag { name: S!("italic"), token: S!("*"), ends: S!("*"), html: S!("<i>"), html_ends: S!("</i>"), ignore_tags: false });
    tags.push(Tag { name: S!("italic"), token: S!("_"), ends: S!("_"), html: S!("<i>"), html_ends: S!("</i>"), ignore_tags: false });
    tags.push(Tag { name: S!("del"), token: S!("~~"), ends: S!("~~"), html: S!("<del>"), html_ends: S!("</del>"), ignore_tags: false });
    tags
});
