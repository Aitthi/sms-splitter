use crate::{
    gsm_validator::GsmValidator,
    splitter_options::SplitterOptions,
    splitter_result::{SplitterPart, SplitterResult},
};

// Debug
#[derive(Debug)]
pub struct GsmSplitter {
    options: SplitterOptions,
    messages: Vec<SplitterPart>,
    length: usize,
    bytes: usize,
    total_bytes: usize,
    total_length: usize,
    message_part: String,
}

impl GsmSplitter {
    pub fn new(options: SplitterOptions) -> GsmSplitter {
        GsmSplitter {
            options,
            messages: Vec::new(),
            length: 0,
            bytes: 0,
            total_bytes: 0,
            total_length: 0,
            message_part: String::new(),
        }
    }

    fn is_high_surrogate(&self, code: u16) -> bool {
        code >= 55296 && code <= 56319
    }

    // SplitterResult
    pub fn split(&mut self, message: String) -> SplitterResult {
        if message.is_empty() {
            return SplitterResult::empty();
        }

        let mut c = 0;
        let count = message.chars().count();
        // println!("count: {}", count);
        while c < count {
            let mut code = message.chars().nth(c).unwrap_or('\u{0020}');
            let c_len = code.len_utf16();
            let mut utf_16 = [0u16; 10];
            let utf_16 = code.encode_utf16(&mut utf_16[0..c_len]);
            let mut i = 0;
            while i < c_len {
                if !self.validate_character(code) {
                    if self.is_high_surrogate(utf_16[i]) {
                        i += 1;
                    }
                    code = '\u{0020}';
                } else if self.validate_extended_character(code) {
                    if self.bytes == 152 {
                        self.bank();
                    }
                    self.bytes += 1;
                }
                self.bytes += 1;
                self.length += 1;
                if !self.options.summary {
                    self.message_part.push(code);
                }
                if self.bytes == 153 {
                    self.bank();
                }
                i += 1;
            }
            c += 1;
        }

        if self.bytes > 0 {
            self.bank();
        }
        match self.messages.get(1) {
            Some(_) => {
                if self.total_bytes <= 160 {
                    let mut parts = Vec::new();
                    let content: String = String::from("");
                    // options.summary ? undefined : messages[0].content + messages[1].content
                    if self.options.summary {
                        parts.push(SplitterPart::new(
                            content,
                            self.total_length,
                            self.total_bytes,
                        ));
                    } else {
                        parts.push(SplitterPart::new(
                            self.messages[0].content.clone() + &self.messages[1].content,
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

    fn bank(&mut self) {
        let mut msg_part = SplitterPart::new("".to_string(), 0, 0);
        if !self.options.summary {
            msg_part = SplitterPart::new(self.message_part.clone(), self.length, self.bytes);
        }
        self.messages.push(msg_part);
        self.total_length += self.length;
        self.length = 0;
        self.total_bytes += self.bytes;
        self.bytes = 0;
        self.message_part = "".to_string();
    }

    fn validate_character(&self, character: char) -> bool {
        if self.options.support_shift_tables {
            return GsmValidator::new().validate_character_with_shift_table(character);
        }
        return GsmValidator::new().validate_character(character);
    }

    fn validate_extended_character(&self, character: char) -> bool {
        if self.options.support_shift_tables {
            return GsmValidator::new().validate_extended_character_with_shift_table(character);
        }
        return GsmValidator::new().validate_extended_character(character);
    }
}

// GsmSplitter default options
impl Default for GsmSplitter {
    fn default() -> Self {
        GsmSplitter {
            options: SplitterOptions::default(),
            messages: Vec::new(),
            length: 0,
            bytes: 0,
            total_bytes: 0,
            total_length: 0,
            message_part: String::new(),
        }
    }
}

impl Clone for GsmSplitter {
    fn clone(&self) -> Self {
        GsmSplitter {
            options: self.options.clone(),
            messages: self.messages.clone(),
            length: self.length,
            bytes: self.bytes,
            total_bytes: self.total_bytes,
            total_length: self.total_length,
            message_part: self.message_part.clone(),
        }
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
        let mut file = File::open(Path::new("test/gsm.test.json")).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let test_data: serde_json::Value = serde_json::from_str(&data).unwrap();
        for test in test_data.as_array().unwrap() {
            // println!("TEST: {:?}", test["description"].to_string().replace("\"", ""));
            let message = test["message"].as_str().unwrap().to_string();
            let expected_parts = test["parts"].as_array().unwrap();
            let expected_total_length = test["totalLength"].as_u64().unwrap() as usize;
            let expected_total_bytes = test["totalBytes"].as_u64().unwrap() as usize;
            let result = GsmSplitter::default().split(message);
            // println!("RESULT: {:#?}", result);
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
