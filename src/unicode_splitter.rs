use crate::{
    splitter_options::SplitterOptions,
    splitter_result::{SplitterPart, SplitterResult},
};

pub struct UnicodeSplitter {
    options: SplitterOptions,
}

impl UnicodeSplitter {
    pub fn new(options: SplitterOptions) -> UnicodeSplitter {
        UnicodeSplitter { options }
    }

    fn is_high_surrogate(&self, code: u16) -> bool {
        (55296..=56319).contains(&code)
    }

    pub fn split(&self, message: String) -> SplitterResult {
        let original_message = message.clone();
        let message = message.encode_utf16().collect::<Vec<u16>>();
        if message.is_empty() {
            return SplitterResult::empty();
        }
        let mut messages: Vec<SplitterPart> = Vec::new();
        let mut length = 0;
        let mut bytes = 0;
        let mut total_bytes = 0;
        let mut total_length = 0;
        let mut part_start = 0;
        let bank = |part_start: &mut usize,
                    part_end: usize,
                    bytes: &mut usize,
                    length: &mut usize,
                    total_bytes: &mut usize,
                    total_length: &mut usize,
                    messages: &mut Vec<SplitterPart>| {
            let mut content: Vec<u16> = Vec::new();
            if !self.options.summary {
                if part_end > 0 {
                    for i in *part_start..part_end + 1 {
                        content.push(message[i]);
                    }
                } else {
                    for i in *part_start..message.len() {
                        content.push(message[i]);
                    }
                }
            }
            let content = String::from_utf16(&content).unwrap();
            let msg = SplitterPart::new(content, *length, *bytes);
            messages.push(msg);
            *part_start = part_end + 1;
            *total_length += *length;
            *length = 0;
            *total_bytes += *bytes;
            *bytes = 0;
        };
        let count = message.len();
        let mut i = 0;
        while i < count {
            let space = *'\u{0020}'
                .to_string()
                .encode_utf16()
                .collect::<Vec<u16>>().first()
                .unwrap();
            let code = message.get(i).unwrap_or(&space);
            let high_surrogate = self.is_high_surrogate(*code);
            // println!("bytes: {} {} {}", bytes, code, high_surrogate);
            if high_surrogate {
                if bytes == 132 {
                    bank(
                        &mut part_start,
                        i - 1,
                        &mut bytes,
                        &mut length,
                        &mut total_bytes,
                        &mut total_length,
                        &mut messages,
                    );
                }
                bytes += 2;
                i += 1;
            }
            bytes += 2;
            length += 1;
            if bytes == 134 {
                bank(
                    &mut part_start,
                    i,
                    &mut bytes,
                    &mut length,
                    &mut total_bytes,
                    &mut total_length,
                    &mut messages,
                );
            }
            i += 1;
        }

        if bytes > 0 {
            bank(
                &mut part_start,
                0,
                &mut bytes,
                &mut length,
                &mut total_bytes,
                &mut total_length,
                &mut messages,
            );
        }

        match messages.get(1) {
            Some(_) => {
                if total_bytes <= 140 {
                    let mut parts = Vec::new();
                    let content: String = String::from("");
                    if self.options.summary {
                        parts.push(SplitterPart::new(content, total_length, total_bytes));
                    } else {
                        parts.push(SplitterPart::new(
                            original_message,
                            total_length,
                            total_bytes,
                        ));
                    }
                    return SplitterResult {
                        parts: parts.clone(),
                        total_length,
                        total_bytes,
                    };
                }
            }
            None => {}
        }
        SplitterResult {
            parts: messages,
            total_length,
            total_bytes,
        }
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
        let splitter = UnicodeSplitter::default();
        for test in test_data.as_array().unwrap() {
            println!(
                "TEST: {:?}",
                test["description"].to_string().replace('\"', "")
            );
            let message = test["message"].as_str().unwrap().to_string();
            let expected_parts = test["parts"].as_array().unwrap();
            let expected_total_length = test["totalLength"].as_u64().unwrap() as usize;
            let expected_total_bytes = test["totalBytes"].as_u64().unwrap() as usize;
            let result = splitter.split(message);
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
