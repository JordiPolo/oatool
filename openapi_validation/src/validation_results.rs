use OpenAPIValidation;
use ValidationOptions;

pub type ValidationResult = Result<(), String>;

#[derive(Debug)]
pub struct ValidationResults {
    errors: Vec<String>,
    warnings: Vec<String>,
}

use std::fmt;
impl fmt::Display for ValidationResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: colors
        if !self.errors.is_empty() {
            write!(f, "\nerrors\n")?;
            for error in &self.errors {
                write!(f, "    {}\n", error)?;
            }
        }

        if !self.warnings.is_empty() {
            write!(f, "\nwarnings\n")?;
            for warning in &self.warnings {
                write!(f, "    {}\n", warning)?;
            }
        }

        write!(f, "")
    }
}


impl ValidationResults {
    pub fn new() -> ValidationResults {
        ValidationResults {
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn assert(&mut self, result: &ValidationResult) {
        result.as_ref().map_err(|e| self.append_error(e)).ok();
    }

    pub fn assert_warn(&mut self, result: &ValidationResult) {
        result.as_ref().map_err(|e| self.append_warning(e)).ok();
    }

    pub fn validate<T>(&mut self, element: &T, options: &ValidationOptions)
    where
        T: OpenAPIValidation,
    {
        let mut result = element.validate(options);
        self.errors.append(&mut result.errors);
        self.warnings.append(&mut result.warnings);
    }

    pub fn failed(&self) -> bool
    {
        !self.errors.is_empty()
    }

    fn append_error(&mut self, value: &str) {
        self.errors.push(value.to_string());
    }

    fn append_warning(&mut self, value: &str) {
        self.warnings.push(value.to_string());
    }

}