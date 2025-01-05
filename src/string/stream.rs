pub struct StringStream {
    data: String,
    pointer: usize,
}

impl StringStream {
    pub fn new(s: String) -> Self {
        Self {
            data: s,
            pointer: 0,
        }
    }

    fn find(&self, c: &str) -> Option<usize> {
        self.data[self.pointer..].find(c).map(|i| i + self.pointer)
    }

    pub fn until(&mut self, c: &str) -> Option<String> {
        self.find(c).map(|i| {
            let result = self.data[self.pointer..i].to_string();
            self.pointer = i;
            result
        })
    }

    pub fn peek(&self) -> Option<char> {
        self.data[self.pointer..].chars().next()
    }

    pub fn skip(&mut self, n: usize) {
        self.pointer += n;
    }

    pub fn has_next(&self) -> bool {
        self.pointer < self.data.len()
    }

    pub fn next(&mut self) -> Option<char> {
        self.data[self.pointer..].chars().next().map(|c| {
            self.pointer += c.len_utf8();
            c
        })
    }

    pub fn accept(&mut self, s: &str) -> bool {
        if self.data[self.pointer..].starts_with(s) {
            self.skip(s.len());
            true
        } else {
            false
        }
    }
}
