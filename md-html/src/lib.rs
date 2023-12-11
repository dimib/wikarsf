pub mod md;
pub mod html;
pub mod common;

use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn md_to_html(input_data: *const c_char) -> *mut c_char {

    let input_data = unsafe {
        assert!(!input_data.is_null());

        std::ffi::CStr::from_ptr(input_data)
    };

    let mut content = md::create_content(input_data.to_str().unwrap().to_string());
    let tokens = md::tokenizer::tokenize(&mut content);
    let html = html::generator::generate_html(tokens);

    println!("{}", html);

    let cstring = CString::new(html).unwrap();
    cstring.into_raw()
//    "" // html
}

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
