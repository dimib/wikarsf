// Declaration of an MD content structure

// MD Content Reader trait
// Must be implemented by any struct that wants to read MD content
pub trait MdContentReader {
    fn next_char(&mut self) -> Option<char>;
    fn current_char(&mut self) -> Option<char>;
    fn remaining(&mut self) -> usize;
    fn inc_index(&mut self, by: usize);
    fn reset(&mut self);
}

#[derive(Clone)]
pub struct MdContent {
    pub content: String,
    pub content_len: usize,
    pub index: usize,
}

impl MdContentReader for MdContent {
    // Returns the next char from content.
    fn next_char(&mut self) -> Option<char> {
        if self.index < self.content_len {
            let c = self.content.as_bytes()[self.index as usize] as char;
            self.index += 1;
            Some(c)
        } else {
            None
        }
    }

    // Returns the next char from the content.
    fn current_char(&mut self) -> Option<char> {
        if self.index < self.content_len {
            Some(self.content.as_bytes()[self.index] as char)
        } else {
            None
        }
    }

    fn remaining(&mut self) -> usize {
        (self.content_len - self.index).clone()
    }


    fn inc_index(&mut self, by: usize) {
        if self.index + by >= self.content_len {
            self.index = self.content_len;
        } else {
            self.index += by;
        }
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}
