extern crate openapi;
extern crate regex;
extern crate inflector;

use regex::Regex;
use std::collections::BTreeMap;
use inflector::Inflector;

mod validation_results;
use validation_results::ValidationResults;

mod field_assert;
use field_assert::{Field, Assert};

#[derive(Default)]
pub struct ValidationOptions {
    pub support_google_spec: bool
}

pub trait OpenAPIValidation {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults;
}

// Based on the openapi crate, there are some fields we know they exist or
// deserialization itself would have failed. We do not test for those.
impl OpenAPIValidation for openapi::Spec {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();

        let swagger     = Field::new(&self.swagger, "version");
        let host        = Field::new(&self.host, "host");
        let base_path   = Field::new(&self.base_path, "basePath");
        let schemes     = Field::new(&self.schemes, "schemes");
        let consumes    = Field::new(&self.consumes, "consumes");
        let produces    = Field::new(&self.produces, "produces");
//        let info        = Field::new(&self.info, "Info block");
        let definitions = Field::new(&self.definitions, "definitions block");
//        let paths       = Field::new(&self.paths, "Paths block");
        let parameters  = Field::new(&self.parameters, "parameters block");


        r.assert(&swagger.eq("2.0"));

        r.assert_warn(&host.not_exist());

        r.assert(&base_path.exist());
        r.assert(&base_path.is_match(&Regex::new(r"^/\w*(/\w+)*$").unwrap()));

        r.assert(&schemes.exist());
        r.assert(&schemes.eq(["https"]));

        r.assert(&schemes.exist());
        r.assert(&schemes.eq(["https"]));

        r.assert(&consumes.exist());
        r.assert(&consumes.eq(["application/json"]));

        r.assert(&produces.exist());
        r.assert(&produces.eq(["application/json"]));

        r.validate(&self.info, options);

        r.validate(&self.paths, options);

        // In theory this may not exist but maybe should be strait assertions
        r.assert_warn(&definitions.exist());
        r.assert_warn(&parameters.exist());

        r
    }
}


impl OpenAPIValidation for openapi::Info {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let title            = Field::new(&self.title, "info.title");
        let description      = Field::new(&self.description, "info.description");
        let version          = Field::new(&self.version, "info.version");
        let contact          = Field::new(&self.contact, "info.contact");
        let terms_of_service = Field::new(&self.terms_of_service, "info.terms_of_service");
        let license          = Field::new(&self.license, "info.license");

        r.assert(&title.exist());

        r.assert(&description.exist());

        if options.support_google_spec {
            r.assert(&version.exist());
        }

        r.assert_warn(&terms_of_service.not_exist());
        r.assert_warn(&license.not_exist());

        r.assert(&contact.exist());

        self.contact.as_ref().map(|contact| {
            r.validate(contact, options)
        });

        r
    }
}


impl OpenAPIValidation for openapi::Contact {
    fn validate(&self, _: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let contact_name  = Field::new(&self.name, "info.contact.name");
        let contact_email = Field::new(&self.email, "info.contact.email");

        r.assert(&contact_name.exist());
        r.assert(&contact_email.exist());

        r
    }
}

struct PathOperation<'a> {
    path: &'a str,
    operation: &'a Option<openapi::Operation>,
}

// Paths
impl OpenAPIValidation for BTreeMap<String, openapi::Operations> { //openapi::Operations {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();

        for (the_path, the_oper) in self {
            let path = the_path.to_string(); // TODO
            let operations = the_oper.clone();
            r.assert(&Field::new(&path, "path name").eq(&path.to_snake_case()));

            r.validate(&PathOperation{ path:&path, operation: &operations.get }, options);
            r.validate(&PathOperation{ path:&path, operation: &operations.post }, options);
            r.validate(&PathOperation{ path:&path, operation: &operations.put }, options);
            r.validate(&PathOperation{ path:&path, operation: &operations.patch }, options);
            r.validate(&PathOperation{ path:&path, operation: &operations.delete }, options);
            r.validate(&PathOperation{ path:&path, operation: &operations.options }, options);
            r.validate(&PathOperation{ path:&path, operation: &operations.head }, options);
        }
        r
    }
}


// struct PathResponses<'a> {
//     path: &'a str,
//     results: &'a BTreeMap<String, openapi::Response>,
// }


impl<'a> OpenAPIValidation for PathOperation<'a> {
    fn validate(&self, options: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let path = self.path;
        let operation = self.operation.clone();
        // TODO: avoid clone
        match operation {
            None => r,
            Some(operation) => {
                let summary      = Field::new(&operation.summary, &format!("summary in '{}'", path));
                let description  = Field::new(&operation.description, &format!("description in '{}'", path));
                let schemes      = Field::new(&operation.schemes, &format!("schemes in '{}'", path));
                let consumes     = Field::new(&operation.consumes, &format!("consumes in '{}'", path)); //&output(path, "consumes"));
                let produces     = Field::new(&operation.produces, &format!("produces in '{}'", path));
                let operation_id = Field::new(&operation.operation_id, &format!("operation_id in '{}'", path));

                r.assert(&summary.exist());
                r.assert(&summary.length_less_than(120));
                // if operation.summary.is_none() {
                //     r.assert(&description.exist());
                // }
                r.assert_warn(&schemes.not_exist());
                r.assert_warn(&consumes.not_exist());
                r.assert_warn(&produces.not_exist());
                if options.support_google_spec {
                    r.assert(&operation_id.exist());
                } else {
                    r.assert_warn(&operation_id.not_exist());
                }

                // responses
                // parameters

                r
            },
        }
    }
}


// impl<'a> OpenAPIValidation for PathResponses<'a> {
//     fn validate(&self, options: &ValidationOptions) -> ValidationResults {
//         let mut r = ValidationResults::new();
//         let path = self.path;
//         let results = self.results.clone();



//         r
//     }
// }


impl OpenAPIValidation for openapi::Parameter {
    fn validate(&self, _: &ValidationOptions) -> ValidationResults {
        let mut r = ValidationResults::new();
        let name  = Field::new(&self.name, &format!("parameter.name {}", &self.name));
        let location = Field::new(&self.location, &format!("parameter.in {}", &self.name));
       // let required = Field::new(&self.required, &format!("parameter.required {}", &self.name));
        let param_type = Field::new(&self.param_type, &format!("parameter.param_type {}", &self.name));
        let description = Field::new(&self.description, &format!("parameter.description {}", &self.name));
        //  let format =
        // TODO : default (openapi library does not support?)
        if location.data().is_some() && location.data().unwrap() != "header" {
            r.assert(&name.eq(&self.name.to_snake_case()));
        }
       // r.assert(&location.included_in(["header", "body", "query", "path"]));
        r.assert(&param_type.exist());
        r.assert(&description.exist());

        r
    }
}

//extern crate term;
//use std::io::prelude::*;


// fn output(location: &str, key_name: &str) -> String {
//     let mut t = String::new();

//     t.fg(term::color::BLUE).unwrap();
//     write!(t, "{}", location).unwrap();

//     t.fg(term::color::GREEN).unwrap();
//     writeln!(t, " :{}  ", key_name).unwrap();

//     t.reset().unwrap();
//     t
//     //format!("{} {}", key_name, location)
// }