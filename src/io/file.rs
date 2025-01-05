use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FastFileReader {
    reader: BufReader<File>,
    buffer: Vec<String>,
}

impl FastFileReader {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    pub fn next_line(&mut self) -> Option<String> {
        let mut line = String::new();
        if self.reader.read_line(&mut line).ok()? == 0 {
            None
        } else {
            Some(
                line.trim_end_matches("\r\n")
                    .trim_end_matches('\n')
                    .to_string(),
            )
        }
    }

    pub fn next_i32(&mut self) -> Option<i32> {
        self.next_word()?.parse::<i32>().ok()
    }

    pub fn next_i64(&mut self) -> Option<i64> {
        self.next_word()?.parse::<i64>().ok()
    }

    pub fn next_word(&mut self) -> Option<String> {
        while self.buffer.is_empty() {
            let mut line = String::new();
            if self.reader.read_line(&mut line).ok()? == 0 {
                return None;
            }
            self.buffer = line.split_whitespace().map(String::from).collect()
        }
        Some(self.buffer.remove(0))
    }
}
