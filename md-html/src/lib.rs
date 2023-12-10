pub mod md;
pub mod html;
pub mod common;

#[cfg(test)]
pub mod tests {
    use super::*;
    use md::create_content;
    use md::tokenizer::tokenize;

    #[test]
    pub fn test_tokenize() {
        let mut md_content = create_content(String::from("# Headline\nHello, world!"));
        let tokens = tokenize(&mut md_content);
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    pub fn test_tag() {
        let tag = common::tokens::break_tag();
        assert_eq!(tag.name, String::from("break"));
    }
}
