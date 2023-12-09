// This is the main tokenizer for the markdown parser.

use std::collections::LinkedList;
use super::MdContent;
use crate::md_html::md::content::MdContentReader;
use crate::md_html::common::tokens::TAGS;
use crate::md_html::common::tokens::Token;
use crate::md_html::common::tokens::Tag;
use crate::md_html::common::tokens::break_tag;

macro_rules! push_text {
    ($tokens:expr, $buffer:expr) => {
        if !$buffer.is_empty() {
            $tokens.push_back(Token::Text($buffer.clone()));
            $buffer.clear();
        }
    };
}

// Tokenize content in the specified content.
pub fn tokenize(md_content: &mut MdContent) -> LinkedList<Token> {
    let mut tokens: LinkedList<Token> = LinkedList::new();
    let mut buffer = String::new();
    let mut tag_stack: LinkedList<Tag> = LinkedList::new();

    md_content.reset();

    // Skip whitespaces and newlines at the beginning
    while md_content.current_char().expect("Did not find the start of content").is_whitespace() {
        // Just skip some unused chars
        md_content.inc_index(1);
    }

    // Determine first token
    while md_content.remaining() > 0 {

        // Check, if we have a token to be closed
        if let Some(current_tag) = tag_stack.back().cloned() {
            if is_end_tag_at_current_index(md_content, &current_tag) {
                push_text!(tokens, buffer);
                tokens.push_back(Token::TagEnd(tag_from_reference(&current_tag)));
                tag_stack.pop_back();
                md_content.inc_index(current_tag.ends.len());
                continue;
            }

            if current_tag.block {
                // This is a block content. We will not check for other tags and translate new lines to <br>.s
                let current_char = md_content.current_char().expect("End of content");
                buffer.push(current_char);
                md_content.inc_index(1);
                continue;
            }
        }

        if is_empty_line(&md_content) {
            push_text!(tokens, buffer);
            tokens.push_back(Token::TagBegin(break_tag()));
            md_content.inc_index(2);
            continue;
        }

        if let Some(tag) = find_start_tag(&md_content) {
            push_text!(tokens, buffer);

            if tag.name == "image" {        
                tokens.push_back(tokenize_image(md_content));
                continue;
            }

            if tag.name == "link" {
                tokens.push_back(tokenize_link(md_content));
                continue;
            }

            if tag.name == "https" || tag.name == "http" {
                tokens.push_back(tokenize_plain_link(md_content));
                continue;
            }

            tokens.push_back(Token::TagBegin(tag_from_reference(tag)));
            tag_stack.push_back(tag_from_reference(tag));

            md_content.inc_index(tag.token.len());
        } else {
            let c = md_content.next_char().expect("Did not find the end of content");
            buffer.push(c);
        }
    }
    // Push the last text token
    push_text!(tokens, buffer);

    tokens
}

// --- Private ---

// Try to find one of the supported start tags at the given index.
fn find_start_tag(content: &MdContent) -> Option<&Tag> {
    let index: usize = content.index;
    let mut tag: Option<&Tag> = None;

    for t in TAGS.iter() {
        if content.content[index..].starts_with(&t.token) {
            tag = Some(t);
            break;
        }
    }
    tag
}

fn is_end_tag_at_current_index(content: &MdContent, tag: &Tag) -> bool {
    let index = content.index;

    if content.content[index..].starts_with(&tag.ends) {
        return true
    }
    false
}

fn is_empty_line(content: &MdContent) -> bool {
    let index = content.index;

    if content.content[index..].starts_with("\n\n") {
        return true
    }
    false
}

fn is_begin_of_line(content: &MdContent) -> bool {
    let index = content.index;
    let before_index = index - 1;

    if index == 0 || content.content[before_index..].starts_with("\n") {
        return true
    }
    false
}

fn tokenize_image(content: &mut MdContent) -> Token {
    let mut alt = String::new();
    let mut src = String::new();

    // The image token looks like this: ![Alternative Text](image-source.jpg)

    // Skip the first two chars (![)
    content.inc_index(2);

    while content.current_char().expect("End of content") != ']' {
        alt.push(content.current_char().expect("End of content"));
        content.inc_index(1);
    }

    // Skip the next two chars ])
    content.inc_index(2);
    while content.current_char().expect("End of content") != ')' {
        src.push(content.current_char().expect("End of content"));
        content.inc_index(1);
    }
    content.inc_index(1);
    Token::Image(alt, src)
}

fn tokenize_plain_link(content: &mut MdContent) -> Token {
    let mut href = String::new();
    let mut text = String::new();

    // Just tokenzie the link.
    while !content.current_char().expect("End of content").is_whitespace() {
        let c = content.current_char().expect("End of content");
        if c == ')' { break; }
        href.push(c);
        text.push(c);
        content.inc_index(1);
    }
    
    Token::Link(text, href)
}

fn tokenize_link(content: &mut MdContent) -> Token {
    let mut text = String::new();
    let mut href = String::new();

    // The image token looks like this: ![Alternative Text](image-source.jpg)

    // Skip the first two chars ([)
    content.inc_index(1);

    while content.current_char().expect("End of content") != ']' {
        text.push(content.current_char().expect("End of content"));
        content.inc_index(1);
    }

    // Skip the next two chars ])
    content.inc_index(2);
    while content.current_char().expect("End of content") != ')' {
        href.push(content.current_char().expect("End of content"));
        content.inc_index(1);
    }
    content.inc_index(1);
    Token::Link(text, href)

}

fn tag_from_reference(tag: &Tag) -> Tag {
    let new_tag = Tag {
        name: tag.name.clone(),
        token: tag.token.clone(),
        ends: tag.ends.clone(),
        html: tag.html.clone(),
        html_ends: tag.html_ends.clone(),
        block: tag.block.clone(),
    };
    new_tag
}
