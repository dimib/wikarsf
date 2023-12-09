// This is the main tokenizer for the markdown parser.

use std::collections::LinkedList;
use super::MdContent;
use super::Token;

use crate::md_parser::content::MdContentReader;

use crate::md_parser::tokens::TAGS;
use crate::md_parser::tokens::Tag;

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

    let end = false;

    // Determine first token
    while md_content.remaining() > 0 {

        // Check, if we have a token to be closed
        if let Some(current_tag) = tag_stack.back().cloned() {
            if is_end_tag_at_current_index(md_content, &current_tag) {
                if !buffer.is_empty() {
                    tokens.push_back(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                tokens.push_back(Token::TagEnd(tag_from_reference(&current_tag)));
                tag_stack.pop_back();
                md_content.inc_index(current_tag.ends.len());
                continue;
            }
        }

        if let Some(tag) = find_start_tag(&md_content) {
            if !buffer.is_empty() {
                tokens.push_back(Token::Text(buffer.clone()));
                buffer.clear();
            }

            tokens.push_back(Token::TagBegin(tag_from_reference(tag)));
            tag_stack.push_back(tag_from_reference(tag));

            md_content.inc_index(tag.token.len());
        } else {
            let c = md_content.next_char().expect("Did not find the end of content");
            buffer.push(c);
        }
    }

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
        if index + tag.ends.len() >= content.content_len {
            return true
        }
        let endindex = index+tag.ends.len();

        if content.content.as_bytes()[endindex].is_ascii_whitespace() {
            return true
        }
    }

    false
}

fn tag_from_reference(tag: &Tag) -> Tag {
    let new_tag = Tag {
        name: tag.name.clone(),
        token: tag.token.clone(),
        ends: tag.ends.clone(),
        html: tag.html.clone(),
        html_ends: tag.html_ends.clone(),
        ignore_tags: tag.ignore_tags.clone(),
    };
    new_tag
}
