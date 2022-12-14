# SMS Splitter

[![Documentation](https://img.shields.io/badge/docs-0.1.9-4d76ae?style=for-the-badge)](https://docs.rs/sms_splitter)
[![Version](https://img.shields.io/crates/v/sms_splitter?style=for-the-badge)](https://crates.io/crates/sms_splitter)
[![NPM version](https://img.shields.io/npm/v/sms-splitter-wasm.svg?style=for-the-badge)](https://www.npmjs.com/package/sms-splitter-wasm)
[![License](https://img.shields.io/crates/l/sms_splitter?style=for-the-badge)](https://crates.io/crates/sms_splitter)

An SMS message splitter with support for both GSM and Unicode written in Rust.
GSM support is limited to GSM 03.38 with the extension table (see the [Wikipedia article](https://en.wikipedia.org/wiki/GSM_03.38#GSM_7_bit_default_alphabet_and_extension_table_of_3GPP_TS_23.038_.2F_GSM_03.38))

## Installation and Usage in Rust

```bash
cargo add sms_splitter
```

```rust
use sms_splitter::SplitSms;

fn main(){
    let info = SplitSms::default().split("Hello World!".to_string());
    println!("{:#?}", info);
}
```
<!-- out put -->
```text
SplitSmsResult {
    character_set: "GSM",
    parts: [
        SplitterPart {
            content: "Hello World!",
            length: 12,
            bytes: 12,
        },
    ],
    bytes: 12,
    length: 12,
    remaining_in_part: 148,
}
```

## Installation and Usage in NodeJs

```bash
npm install sms-splitter-wasm
```
or
```bash
yarn add sms-splitter-wasm
```

```ts
import { SmsSplitter } from "sms-splitter-wasm";

const message = "Hello World!";
const splitter = new SmsSplitter();
console.log(splitter.split(message));
```
```json
{
  "character_set": "GSM",
  "parts": [ { "content": "Hello World!", "length": 12, "bytes": 12 } ],
  "bytes": 12,
  "length": 12,
  "remaining_in_part": 148
}
```

# Credits

A lot of the code in this package was based on Codesleuth [`split-sms`](https://github.com/Codesleuth/split-sms).