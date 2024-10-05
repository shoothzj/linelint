pub struct Issue {
    pub rule: String,
    pub filename: String,
    pub description: String,
    pub line_number: usize,
}

impl Issue {
    pub fn new(rule: &str, filename: &str, description: &str, line_number: usize) -> Self {
        Issue {
            rule: rule.to_string(),
            filename: filename.to_string(),
            description: description.to_string(),
            line_number,
        }
    }
}
