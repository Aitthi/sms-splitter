//! # SMS Splitter
//!
//! [![Documentation](https://img.shields.io/badge/docs-0.1.9-4d76ae?style=for-the-badge)](https://docs.rs/sms_splitter)
//! [![Version](https://img.shields.io/crates/v/sms_splitter?style=for-the-badge)](https://crates.io/crates/sms_splitter)
//! [![License](https://img.shields.io/crates/l/sms_splitter?style=for-the-badge)](https://crates.io/crates/sms_splitter)
//!
//! An SMS message splitter with support for both GSM and Unicode written in Rust.
//! GSM support is limited to GSM 03.38 with the extension table (see the [Wikipedia article](https://en.wikipedia.org/wiki/GSM_03.38#GSM_7_bit_default_alphabet_and_extension_table_of_3GPP_TS_23.038_.2F_GSM_03.38))
//!
//! ## Installation
//!
//! ```bash
//! cargo add sms_splitter
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use sms_splitter::SplitSms;
//!
//! fn main(){
//!     let info = SplitSms::default().split("Hello World!".to_string());
//!     println!("{:#?}", info);
//! }
//! ```
//! <!-- out put -->
//! ```text
//! SplitSmsResult {
//!     character_set: "GSM",
//!     parts: [
//!         SplitterPart {
//!             content: "Hello World!",
//!             length: 12,
//!             bytes: 12,
//!         },
//!     ],
//!     bytes: 12,
//!     length: 12,
//!     remaining_in_part: 148,
//! }
//! ```
//!
//! # Credits
//!
//! A lot of the code in this package was based on Codesleuth [`split-sms`](https://github.com/Codesleuth/split-sms).
//!
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
        (max - parts[parts.len() - 1].bytes) / char_bytes
    }

    fn validate_message(&self, message: String) -> bool {
        if self.options.support_shift_tables {
            return gsm_validator::GsmValidator::new().validate_message_with_shift_table(message);
        }
        gsm_validator::GsmValidator::new().validate_message(message)
    }

    pub fn split(&self, message: String) -> SplitSmsResult {
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
            .split(message);
            single_bytes = 160;
            multi_bytes = 153;
            char_bytes = 1;
            character_set = "GSM".to_string();
        } else {
            split_result = unicode_splitter::UnicodeSplitter::new(SplitterOptions::new(
                self.options.support_shift_tables,
                self.options.summary,
            ))
            .split(message);
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
