extern crate openapi;

#[macro_use]
extern crate error_chain;

mod validation_results;
use validation_results::ValidationResults;


pub mod errors {
    error_chain!{
        errors {
            ValidationError(error_string: String) {
                description("OpenAPI validation error")
                display("OpenAPI validation error: '{}'", error_string)
            }
        }
    }
}
use errors::*;

pub struct ValidationOptions {
    pub support_google_spec: bool,
}

pub trait OpenAPIValidation {
    fn validate(&self, options: &ValidationOptions, results: &mut ValidationResults) {}
}



trait Assertions {
    fn assert_existence(&self, results: &mut ValidationResults) {}
    fn assert_eq(&self, value: &str, results: &mut ValidationResults) {}
    fn assert_array_value(&self, value: &str, results: &mut ValidationResults) {}
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

impl<'a> Assertions for Field<&'a String> {
    fn assert_eq(&self, value: &str, results: &mut ValidationResults) {
         if self.data != value {
            let error = format!("Expected {} to be {} but it was {:?}", self.name, value, self);
            results.append_error(error);
        }
    }
}

impl<'a> Assertions for Field<&'a Option<String>> {
    fn assert_eq(&self, value: &str, results: &mut ValidationResults) {
         if self.data.as_ref().unwrap() != value {
            let error = format!("Expected {} to be {} but it was {:?}", self.name, value, self);
            results.append_error(error);
        }
    }
    fn assert_existence(&self, results: &mut ValidationResults)  {
        if self.data.is_none() {
            let error = format!("{} does not exist in the spec. It needs to be set to a value.", self.name);
            results.append_error(error);
        }
    }

}

impl<'a> Assertions for Field<&'a Option<Vec<String>>> {
    fn assert_existence(&self, results: &mut ValidationResults)  {
        if self.data.is_none() {
            let error = format!("{} does not exist in the spec. It needs to be set to a value.", self.name);
            results.append_error(error);
        }
    }
    fn assert_array_value(&self, value: &str, results: &mut ValidationResults) {
        if self.data.as_ref().unwrap() != &vec![value.to_string()] {
            let error = format!("Only the value '{}' is accepted for {} at the top level of the spec", value, self.name);
            results.append_error(error);
        }
    }

}



// Based on the openapi crate, there are some fields we know they exist or
// deserialization itself would have failed. We do not test for those.
impl OpenAPIValidation for openapi::Spec {
    fn validate(&self, options: &ValidationOptions, mut results: &mut ValidationResults) {
      //  let mut results = ValidationResults::new();

        let swagger = Field::new(&self.swagger, "version");
        swagger.assert_eq("2.0", &mut results);

        // TODO: warning if host is set

      //  self.info.validate(options)?;

        let base_path = Field::new(&self.base_path, "basePath");
        base_path.assert_existence(&mut results);
        // TODO: warning if not version on path
        // self.base_path.with_name("basePath").assert_regexp(/\v/, &mut results)

        let schemes = Field::new(&self.schemes, "schemes");
        schemes.assert_existence(&mut results);
        schemes.assert_array_value("https", &mut results);

        let consumes = Field::new(&self.consumes, "consumes");
        consumes.assert_existence(&mut results);
        consumes.assert_array_value("application/json", &mut results);

        let produces = Field::new(&self.produces, "produces");
        produces.assert_existence(&mut results);
       // produces.assert_array_value("application/json", &mut results);

    }
}


// impl OpenAPIValidation for openapi::Info {
//     fn validate(&self, options: &ValidationOptions, &mut results: ValidationResults) {
//         //let title = Field::new(&self.title, "info.title");
//         //title.assert_existence("title", &mut results);
//         assert_existence(&self.title, "info.title")?;
//         assert_existence(&self.description, "info.description")?;
//         if options.support_google_spec {
//             assert_existence(&self.version, "info.version")?;
//         }
//         assert_existence(&self.contact, "info.contact")?;
//         assert_existence(&self.contact.as_ref().unwrap().name, "info.contact.name")?;
//         assert_existence(&self.contact.as_ref().unwrap().email, "info.contact.email")?;
//         // TODO: verify email format
//     }
// }


