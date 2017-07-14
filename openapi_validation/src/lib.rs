extern crate openapi;
extern crate regex;

use std::fmt::Debug;
use regex::Regex;

pub struct ValidationOptions {
    pub support_google_spec: bool,
}

pub trait OpenAPIValidation {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults;
}


type ValidationResult = Result<(), String>;

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
    pub fn append_error(&mut self, value: &str) {
        self.errors.push(value.to_string());
    }

    pub fn append_warning(&mut self, value: &str) {
        self.warnings.push(value.to_string());
    }

    pub fn assert(&mut self, result: &ValidationResult) {
        result.as_ref().map_err(|e| self.append_error(e));
    }

    pub fn assert_warn(&mut self, result: &ValidationResult) {
        result.as_ref().map_err(|e| self.append_warning(e));
    }

    pub fn validate<T>(&mut self, element: &T, options: &ValidationOptions)
    where
        T: OpenAPIValidation,
    {
        let mut errors = element.validate(options).errors;
        self.errors.append(&mut errors);
    }
}


trait Assert {
    type Data: Debug;

    fn data(&self) -> Option<Self::Data>;

    fn name(&self) -> &str;

    fn eq<T>(&self, value: T) -> ValidationResult
    where
        Self::Data: PartialEq<T>,
        T: Debug,
    {
        match self.data() {
            Some(ref a) if a == &value => Ok(()),
            _ => Err(format!(
                "Expected {} to be {:?} but it was {:?}",
                self.name(),
                value,
                self.data()
            )),
        }
    }
}


#[derive(Debug)]
struct Field<T> {
    data: T,
    name: String,
}

impl<T> Field<T> {
    fn new(data: T, name: &str) -> Field<T> {
        Field {
            data: data,
            name: name.to_string(),
        }
    }
}


impl<'a> Assert for Field<&'a String> {
    type Data = &'a String;

    fn data(&self) -> Option<Self::Data> {
        Some(self.data)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}


impl<'a> Assert for Field<&'a Option<String>> {
    type Data = &'a String;

    fn data(&self) -> Option<Self::Data> {
        self.data.as_ref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}
impl<'a> Field<&'a Option<String>> {
    fn is_match(&self, regex: &Regex) -> ValidationResult {
        match self.data() {
            Some(string) => {
                if !regex.is_match(string) {
                    Err(format!(
                        "The value {} for {} does not follow the proper format of the guidelines",
                        string,
                        self.name()
                    ))
                } else {
                    Ok(())
                }
            }
            None => Ok(()), // Really not ok but some other test should already complain about this.
        }
    }
}



impl<'a> Assert for Field<&'a Option<Vec<String>>> {
    type Data = &'a [String];

    fn data(&self) -> Option<Self::Data> {
        self.data.as_ref().map(|x| &x[..])
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}


//Existence or lack of it is just a property of Option.
impl<'a, T> Field<&'a Option<T>> {
    fn exist(&self) -> ValidationResult {
        self.data.as_ref().map(|_| ()).ok_or(format!(
            "{:?} does not exist in the spec. It needs to be set to a value.",
            self.name
        ))
    }
    fn not_exist(&self) -> ValidationResult {
        if self.data.as_ref().is_some() {
            Err(format!(
                "{:?} exists in the spec but it should not.",
                self.name
            ))
        } else {
            Ok(())
        }
    }
}



// Based on the openapi crate, there are some fields we know they exist or
// deserialization itself would have failed. We do not test for those.
impl OpenAPIValidation for openapi::Spec {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let base_path_regex = Regex::new(r"^/\w*(/\w+)*$").unwrap();

        let swagger = Field::new(&self.swagger, "version");
        let host = Field::new(&self.host, "host");
        let base_path = Field::new(&self.base_path, "basePath");
        let schemes = Field::new(&self.schemes, "schemes");
        let consumes = Field::new(&self.consumes, "consumes");
        let produces = Field::new(&self.produces, "produces");

        r.assert(&swagger.eq("2.0"));

        r.assert_warn(&host.not_exist());

        r.assert(&base_path.exist());
        r.assert(&base_path.is_match(&base_path_regex));

        r.assert(&schemes.exist());
        r.assert(&schemes.eq(["https"]));

        r.assert(&schemes.exist());
        r.assert(&schemes.eq(["https"]));

        r.assert(&consumes.exist());
        r.assert(&consumes.eq(["application/json"]));

        r.assert(&produces.exist());
        r.assert(&produces.eq(["application/json"]));

        r.validate(&self.info, options);

        r
    }
}


impl OpenAPIValidation for openapi::Contact {
    fn validate(&self, _: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let contact_name = Field::new(&self.name, "info.contact.name");
        let contact_email = Field::new(&self.email, "info.contact.email");

        r.assert(&contact_name.exist());
        r.assert(&contact_email.exist());

        r
    }
}

impl OpenAPIValidation for openapi::Info {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let title = Field::new(&self.title, "info.title");
        let description = Field::new(&self.description, "info.description");
        let version = Field::new(&self.version, "info.version");
        let contact = Field::new(&self.contact, "info.contact");
        let terms_of_service = Field::new(&self.terms_of_service, "info.terms_of_service");
        let license = Field::new(&self.license, "info.license");

        r.assert(&title.exist());

        r.assert(&description.exist());

        if options.support_google_spec {
            r.assert(&version.exist())
        }

        r.assert_warn(&terms_of_service.not_exist());
        r.assert_warn(&license.not_exist());

        r.assert(&contact.exist());

        contact.data.as_ref().and_then(|contact| {
            Some(r.validate(contact, options))
        });

        r
    }
}
