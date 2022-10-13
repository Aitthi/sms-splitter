#[derive(Debug)]
pub struct SplitterOptions {
    pub support_shift_tables: bool,
    pub summary: bool,
}

impl SplitterOptions {
    pub fn new(support_shift_tables: bool, summary: bool) -> SplitterOptions {
        SplitterOptions {
            support_shift_tables,
            summary,
        }
    }
}

// default options
impl Default for SplitterOptions {
    fn default() -> Self {
        SplitterOptions {
            support_shift_tables: false,
            summary: false,
        }
    }
}

impl Clone for SplitterOptions {
    fn clone(&self) -> Self {
        SplitterOptions {
            support_shift_tables: self.support_shift_tables,
            summary: self.summary,
        }
    }
}
