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
    tags.push(Tag { name: S!("h1"), token: S!("# "), ends: S!("\n"), html: S!("<h1>"), html_ends: S!("</h1>") });
    tags.push(Tag { name: S!("h2"), token: S!("## "), ends: S!("\n"), html: S!("<h2>"), html_ends: S!("</h2>") });
    tags.push(Tag { name: S!("h3"), token: S!("### "), ends: S!("\n"), html: S!("<h3>"), html_ends: S!("</h3>") });
    tags.push(Tag { name: S!("h4"), token: S!("#### "), ends: S!("\n"), html: S!("<h4>"), html_ends: S!("</h4>") });
    tags.push(Tag { name: S!("h5"), token: S!("##### "), ends: S!("\n"), html: S!("<h5>"), html_ends: S!("</h5>") });
    tags.push(Tag { name: S!("h6"), token: S!("###### "), ends: S!("\n"), html: S!("<h6>"), html_ends: S!("</h6>") });
    tags.push(Tag { name: S!("code"), token: S!("`"), ends: S!("`"), html: S!("<code>"), html_ends: S!("</code>") });
    tags.push(Tag { name: S!("italic"), token: S!("*"), ends: S!("*"), html: S!("<i>"), html_ends: S!("</i>") });
    tags.push(Tag { name: S!("italic"), token: S!("_"), ends: S!("_"), html: S!("<i>"), html_ends: S!("</i>") });
    tags.push(Tag { name: S!("bold"), token: S!("**"), ends: S!("**"), html: S!("<b>"), html_ends: S!("</b>") });
    tags.push(Tag { name: S!("bold"), token: S!("__"), ends: S!("__"), html: S!("<b>"), html_ends: S!("</b>") });
    tags.push(Tag { name: S!("bolditalic"), token: S!("***"), ends: S!("***"), html: S!("<b><i>"), html_ends: S!("</i></b>") });
    tags.push(Tag { name: S!("bolditalic"), token: S!("___"), ends: S!("___"), html: S!("<b><i>"), html_ends: S!("</i></b>") });
    tags.push(Tag { name: S!("del"), token: S!("~~"), ends: S!("~~"), html: S!("<del>"), html_ends: S!("</del>") });
    tags
});

/*
pub fn makeTags() -> Vec<Tag> {
    println!("Make tags");
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
*/

