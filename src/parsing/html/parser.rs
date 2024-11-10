use crate::parsing::Parse;

pub struct Parser {
    cursor: usize,
    input: String,
}

impl Parse for Parser {
    fn input(&self) -> &str {
        &self.input
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }
}