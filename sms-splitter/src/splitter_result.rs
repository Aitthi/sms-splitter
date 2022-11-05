#[derive(Debug)]
pub struct SplitterResult {
    pub parts: Vec<SplitterPart>,
    pub total_length: usize,
    pub total_bytes: usize,
}

impl SplitterResult {
    pub fn empty() -> Self {
        let parts = vec![SplitterPart::new(String::new(), 0, 0)];
        SplitterResult {
            parts,
            total_length: 0,
            total_bytes: 0,
        }
    }
}

impl Clone for SplitterResult {
    fn clone(&self) -> Self {
        SplitterResult {
            parts: self.parts.clone(),
            total_length: self.total_length,
            total_bytes: self.total_bytes,
        }
    }
}

#[derive(Debug)]
pub struct SplitterPart {
    pub content: String,
    pub length: usize,
    pub bytes: usize,
}
impl SplitterPart {
    pub fn new(content: String, length: usize, bytes: usize) -> Self {
        SplitterPart {
            content,
            length,
            bytes,
        }
    }
}

impl Clone for SplitterPart {
    fn clone(&self) -> Self {
        SplitterPart {
            content: self.content.clone(),
            length: self.length,
            bytes: self.bytes,
        }
    }
}
