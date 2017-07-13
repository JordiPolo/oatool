pub struct ValidationResults {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl ValidationResults {
    pub fn new() -> ValidationResults {
        ValidationResults {
            errors: vec![],
            warnings: vec![],
        }
    }
    pub fn append_error(&mut self, value: String) {
        self.errors.push(value);
    }
}