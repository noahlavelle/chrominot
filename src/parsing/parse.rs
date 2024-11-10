pub trait Parse {
    fn input(&self) -> &str;
    fn cursor(&self) -> usize;
    fn set_cursor(&mut self, cursor: usize);

    fn shift_cursor(&mut self, distance: Option<usize>) {
        self.set_cursor(self.cursor() + distance.unwrap_or(1));
    }

    fn next_char(&self) -> char {
        self.input()[self.cursor() ..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input()[self.cursor() ..].starts_with(s)
    }

    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.shift_cursor(Some(s.len()));
        } else {
            panic!("Expected {:?} at byte {} but it was not found", s, self.cursor());
        }
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.shift_cursor(Some(c.len_utf8()));
        c
    }

    fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool
    {
        let mut result = String::new();
        while !self.eof() && f(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(char::is_alphanumeric)
    }

    fn eof(&self) -> bool {
        self.cursor() >= self.input().len()
    }
}