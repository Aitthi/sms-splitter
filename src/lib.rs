pub mod gsm_splitter;
pub mod gsm_validator;
pub mod splitter_options;
pub mod splitter_result;
pub mod unicode_splitter;
// use
use splitter_options::SplitterOptions;
use splitter_result::{SplitterPart, SplitterResult};

#[derive(Debug)]
pub struct SplitSms {
    options: SplitterOptions,
}

impl Default for SplitSms {
    fn default() -> Self {
        SplitSms::new(SplitterOptions::default())
    }
}

impl SplitSms {
    pub fn new(options: SplitterOptions) -> SplitSms {
        SplitSms { options }
    }

    fn calculate_remaining(
        &self,
        parts: &Vec<SplitterPart>,
        single_bytes: usize,
        multi_bytes: usize,
        char_bytes: usize,
    ) -> usize {
        let mut max = multi_bytes;
        if parts.len() == 1 {
            max = single_bytes;
        }
        return (max - parts[parts.len() - 1].bytes) / char_bytes;
    }

    fn validate_message(&self, message: String) -> bool {
        if self.options.support_shift_tables {
            return gsm_validator::GsmValidator::new().validate_message_with_shift_table(message);
        }
        return gsm_validator::GsmValidator::new().validate_message(message);
    }

    pub fn split(&mut self, message: String) -> SplitSmsResult {
        let is_gsm = self.validate_message(message.clone());
        let split_result: SplitterResult;
        let single_bytes: usize;
        let multi_bytes: usize;
        let char_bytes: usize;
        let character_set: String;
        if is_gsm {
            split_result = gsm_splitter::GsmSplitter::new(SplitterOptions::new(
                self.options.support_shift_tables,
                self.options.summary,
            ))
            .split(message.clone());
            single_bytes = 160;
            multi_bytes = 153;
            char_bytes = 1;
            character_set = "GSM".to_string();
        } else {
            split_result = unicode_splitter::UnicodeSplitter::new(SplitterOptions::new(
                self.options.support_shift_tables,
                self.options.summary,
            ))
            .split(message.clone());
            single_bytes = 140;
            multi_bytes = 134;
            char_bytes = 2;
            character_set = "Unicode".to_string();
        }
        let remaining_in_part =
            self.calculate_remaining(&split_result.parts, single_bytes, multi_bytes, char_bytes);
        SplitSmsResult::new(
            character_set,
            split_result.parts,
            split_result.total_bytes,
            split_result.total_length,
            remaining_in_part,
        )
    }
}

#[derive(Debug)]
pub struct SplitSmsResult {
    pub character_set: String,
    pub parts: Vec<SplitterPart>,
    pub bytes: usize,
    pub length: usize,
    pub remaining_in_part: usize,
}

impl SplitSmsResult {
    pub fn new(
        character_set: String,
        parts: Vec<SplitterPart>,
        bytes: usize,
        length: usize,
        remaining_in_part: usize,
    ) -> SplitSmsResult {
        SplitSmsResult {
            character_set,
            parts,
            bytes,
            length,
            remaining_in_part,
        }
    }
}

impl Clone for SplitSmsResult {
    fn clone(&self) -> Self {
        SplitSmsResult {
            character_set: self.character_set.clone(),
            parts: self.parts.clone(),
            bytes: self.bytes,
            length: self.length,
            remaining_in_part: self.remaining_in_part,
        }
    }
}