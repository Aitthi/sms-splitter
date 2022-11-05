use crate::{
    gsm_validator::GsmValidator,
    splitter_options::SplitterOptions,
    splitter_result::{SplitterPart, SplitterResult},
};

// Debug
#[derive(Debug,Default)]
pub struct GsmSplitter {
    options: SplitterOptions,
}

impl GsmSplitter {
    pub fn new(options: SplitterOptions) -> GsmSplitter {
        GsmSplitter { options }
    }

    fn is_high_surrogate(&self, code: u16) -> bool {
        (55296..=56319).contains(&code)
    }

    // SplitterResult
    pub fn split(&self, message: String) -> SplitterResult {
        let message = message.encode_utf16().collect::<Vec<u16>>();
        if message.is_empty() {
            return SplitterResult::empty();
        }
        let mut messages: Vec<SplitterPart> = Vec::new();
        let mut length = 0;
        let mut bytes = 0;
        let mut total_bytes = 0;
        let mut total_length = 0;
        let mut message_part = String::new();
        let bank = |bytes: &mut usize,
                    length: &mut usize,
                    total_bytes: &mut usize,
                    total_length: &mut usize,
                    message_part: &mut String,
                    messages: &mut Vec<SplitterPart>| {
            let mut msg_part = SplitterPart::new("".to_string(), 0, 0);
            if !self.options.summary {
                msg_part = SplitterPart::new(message_part.clone(), *length, *bytes);
            }
            messages.push(msg_part);
            *total_length += *length;
            *length = 0;
            *total_bytes += *bytes;
            *bytes = 0;
            *message_part = "".to_string();
        };
        let count = message.len();
        // println!("count: {}", count);
        let mut i = 0;
        while i < count {
            let space = *'\u{0020}'
                .to_string()
                .encode_utf16()
                .collect::<Vec<u16>>().first()
                .unwrap();
            let mut code = message.get(i).unwrap_or(&space);
            // println!("bytes: {} {}", bytes, code);
            if !self.validate_character(*code) {
                if self.is_high_surrogate(*code) {
                    i += 1;
                }
                code = &space;
            } else if self.validate_extended_character(*code) {
                // println!("extended character bytes {}", bytes);
                if bytes == 152 {
                    bank(
                        &mut bytes,
                        &mut length,
                        &mut total_bytes,
                        &mut total_length,
                        &mut message_part,
                        &mut messages,
                    );
                }
                bytes += 1;
            }
            bytes += 1;
            length += 1;
            if !self.options.summary {
                message_part.push(std::char::from_u32(*code as u32).unwrap());
            }
            if bytes == 153 {
                bank(
                    &mut bytes,
                    &mut length,
                    &mut total_bytes,
                    &mut total_length,
                    &mut message_part,
                    &mut messages,
                );
            }
            i += 1;
        }

        if bytes > 0 {
            bank(
                &mut bytes,
                &mut length,
                &mut total_bytes,
                &mut total_length,
                &mut message_part,
                &mut messages,
            );
        }
        if messages.get(1).is_some() && total_bytes <= 160 {
            let mut parts = Vec::new();
            let content: String = String::from("");
            // options.summary ? undefined : messages[0].content + messages[1].content
            if self.options.summary {
                parts.push(SplitterPart::new(content, total_length, total_bytes));
            } else {
                parts.push(SplitterPart::new(
                    messages[0].content.clone() + &messages[1].content,
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
        SplitterResult {
            parts: messages,
            total_length,
            total_bytes,
        }
    }

    fn validate_character(&self, character_code: u16) -> bool {
        if self.options.support_shift_tables {
            return GsmValidator::new().validate_character_with_shift_table(character_code);
        }
        GsmValidator::new().validate_character(character_code)
    }

    fn validate_extended_character(&self, character_code: u16) -> bool {
        if self.options.support_shift_tables {
            return GsmValidator::new()
                .validate_extended_character_with_shift_table(character_code);
        }
        GsmValidator::new().validate_extended_character(character_code)
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
        let splitter = GsmSplitter::default();
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
