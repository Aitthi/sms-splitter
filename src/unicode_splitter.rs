use crate::{
    splitter_options::SplitterOptions,
    splitter_result::{SplitterPart, SplitterResult},
};

pub struct UnicodeSplitter {
    options: SplitterOptions,
    messages: Vec<SplitterPart>,
    length: usize,
    bytes: usize,
    total_bytes: usize,
    total_length: usize,
    part_start: usize,
    message: String,
}

impl UnicodeSplitter {
    pub fn new(options: SplitterOptions) -> UnicodeSplitter {
        UnicodeSplitter {
            options,
            messages: Vec::new(),
            length: 0,
            bytes: 0,
            total_bytes: 0,
            total_length: 0,
            part_start: 0,
            message: String::from(""),
        }
    }

    fn is_high_surrogate(&self, code: u16) -> bool {
        code >= 55296 && code <= 56319
    }

    fn bank(&mut self, part_end: usize) {
        let mut content = String::from("");
        if !self.options.summary {
            if part_end > 0 {
                content = self
                    .message
                    .chars()
                    .skip(self.part_start)
                    .take(part_end + 1)
                    .collect();
            } else {
                content = self.message.chars().skip(self.part_start).collect();
            }
        }
        let msg = SplitterPart::new(content, self.length, self.bytes);
        self.messages.push(msg);
        self.part_start = part_end + 1;
        self.total_length += self.length;
        self.length = 0;
        self.total_bytes += self.bytes;
        self.bytes = 0;
    }

    pub fn split(&mut self, message: String) -> SplitterResult {
        self.message = message;
        if self.message.is_empty() {
            return SplitterResult::empty();
        }

        let mut c = 0;
        let count = self.message.chars().count();
        // println!("count: {}", count);
        while c < count {
            let code = self.message.chars().nth(c).unwrap();
            let c_len = code.len_utf16();
            let mut utf_16 = [0u16; 10];
            let utf_16 = code.encode_utf16(&mut utf_16[0..c_len]);
            let mut i = 0;
            while i < c_len {
                let high_surrogate = self.is_high_surrogate(utf_16[i]);
                if high_surrogate {
                    if self.bytes == 132 {
                        self.bank(c - 1);
                    }
                    self.bytes += 2;
                    i += 1;
                }
                self.bytes += 2;
                self.length += 1;
                if self.bytes == 134 {
                    self.bank(c);
                }
                i += 1;
            }
            c += 1;
        }

        if self.bytes > 0 {
            self.bank(0);
        }

        match self.messages.get(1) {
            Some(_) => {
                if self.total_bytes <= 140 {
                    let mut parts = Vec::new();
                    let content: String = String::from("");
                    if self.options.summary {
                        parts.push(SplitterPart::new(
                            content,
                            self.total_length,
                            self.total_bytes,
                        ));
                    } else {
                        parts.push(SplitterPart::new(
                            self.message.clone(),
                            self.total_length,
                            self.total_bytes,
                        ));
                    }
                    return SplitterResult {
                        parts: parts.clone(),
                        total_length: self.total_length,
                        total_bytes: self.total_bytes,
                    };
                }
            }
            None => {}
        }
        return SplitterResult {
            parts: self.messages.clone(),
            total_length: self.total_length,
            total_bytes: self.total_bytes,
        };
    }
}

impl Default for UnicodeSplitter {
    fn default() -> Self {
        UnicodeSplitter::new(SplitterOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // import json test data from  test folder
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn gsm_split() {
        let mut file = File::open(Path::new("test/unicode.test.json")).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let test_data: serde_json::Value = serde_json::from_str(&data).unwrap();
        for test in test_data.as_array().unwrap() {
            println!(
                "TEST: {:?}",
                test["description"].to_string().replace("\"", "")
            );
            let message = test["message"].as_str().unwrap().to_string();
            let expected_parts = test["parts"].as_array().unwrap();
            let expected_total_length = test["totalLength"].as_u64().unwrap() as usize;
            let expected_total_bytes = test["totalBytes"].as_u64().unwrap() as usize;
            let result = UnicodeSplitter::default().split(message);
            println!("RESULT: {:#?}", result);
            // println!("result.length: {} == {}", result.total_length, expected_total_length);
            assert_eq!(result.total_length, expected_total_length);
            // println!("result.bytes: {} == {}", result.total_bytes, expected_total_bytes);
            assert_eq!(result.total_bytes, expected_total_bytes);
            for (i, part) in result.parts.iter().enumerate() {
                let expected_part = expected_parts[i].as_object().unwrap();
                assert_eq!(
                    part.bytes,
                    expected_part["bytes"].as_u64().unwrap() as usize
                );
                assert_eq!(
                    part.length,
                    expected_part["length"].as_u64().unwrap() as usize
                );
                assert_eq!(part.content, expected_part["content"].as_str().unwrap());
            }
            // println!("--------------------------------------------");
        }
    }
}
