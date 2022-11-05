use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(typescript_custom_section)]
const ISmsSplitter: &'static str = r#"
export namespace ISmsSplitter {
    export interface ISplitSmsResult {
      character_set: string;
      parts: { content: string, length: number, bytes: number }[];
      bytes: number;
      length: number;
      remaining_in_part: number;
    }

    export interface ISplitterOptions {
        support_shift_tables: boolean;
        summary: boolean;
    }
}
"#;

#[wasm_bindgen]
extern "C" {
    
    #[wasm_bindgen(typescript_type = "ISmsSplitter.ISplitSmsResult")]
    pub type ISplitSmsResult;

    #[wasm_bindgen(typescript_type = "ISmsSplitter.ISplitterOptions")]
    pub type ISplitterOptions;

    #[wasm_bindgen(js_namespace = JSON)]
    fn parse(text: &str) -> JsValue;

}

#[wasm_bindgen]
pub struct SmsSplitter {
    options: sms_splitter::splitter_options::SplitterOptions,
}

#[wasm_bindgen]
impl SmsSplitter {

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<ISplitterOptions>) -> SmsSplitter {
        let mut splitter_options = sms_splitter::splitter_options::SplitterOptions::default();
        if options.is_some() {
            let options: sms_splitter::splitter_options::SplitterOptions = serde_wasm_bindgen::from_value(options.unwrap().obj).unwrap();
            splitter_options = options;
        }
        SmsSplitter {
            options: splitter_options,
        }
    }

    pub fn split(self,message: &str) -> ISplitSmsResult {
        let result = sms_splitter::SplitSms::new(self.options).split(message.to_string());
        ISplitSmsResult::from(parse(result.to_string().as_str()))
    }
}