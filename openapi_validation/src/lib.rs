extern crate openapi;
/*
pub struct Spec {
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemes: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub produces: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// HTTP path to Operations mappings
    pub paths: BTreeMap<String, Operations>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub definitions: Option<BTreeMap<String, Schema>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<BTreeMap<String, Parameter>>,
    /// mappings to http response codes or "default"
    #[serde(skip_serializing_if="Option::is_none")]
    pub responses: Option<BTreeMap<String, Response>>,
}
*/
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
    fn validate(&self, options: &ValidationOptions) -> Result<()> {
        Ok(())
    }
}





trait Assertions {
    fn assert_existence(&self, field_name: &str, results: &mut ValidationResults)  {}
    fn assert_eq(&self, value: &str, field_name: &str, results: &mut ValidationResults) {}
}

impl Assertions for String {
    fn assert_eq(&self, value: &str, field_name: &str, results: &mut ValidationResults) {
        if self != value {
            let error = format!("Expected {} to be {} but it was {}", field_name, value, self);
            results.append_error(error);
        }
    }

}

impl Assertions for Option<String> {
    fn assert_existence(&self, field_name: &str, results: &mut ValidationResults) {
        if self.is_none() {
            let error = format!("{} does not exist in the spec. It needs to be set to a value.", field_name);
            results.append_error(error);
        }
    }
    fn assert_eq(&self, value: &str, field_name: &str, results: &mut ValidationResults) {
       // if self.as_ref.unwrap() != value
    }

}

impl Assertions for Option<Vec<String>> {
    fn assert_existence(&self, field_name: &str, results: &mut ValidationResults) {
        if self.is_none() {
            let error = format!("{} does not exist in the spec. It needs to be set to a value.", field_name);
            results.append_error(error);
        }
    }
}

// #[derive(Debug)]
// struct Field<T> {
//     data: T,
//     name: String,
// }

// impl<T> Field<T> {
//     fn new(data: T, name: &str) -> Field<T> {
//         Field {
//             data: data,
//             name: name.to_string(),
//         }
//     }
// }

// impl<T> Assertions for Field<T> {
//     fn assert_eq(&self, value: &str, field_name: &str, results: &mut ValidationResults) {
//          if self != value {
//             let error = format!("Expected {} to be {} but it was {:?}", field_name, value, self);
//             results.append_error(error);
//         }
//     }
//     fn assert_existence(&self, field_name: &str, results: &mut ValidationResults)  {}

// }

// trait NamedField {
//     fn with_name<T>(&self, name: &str) -> Field<T> {
//         Field {data: T, name: "".to_string()}
//     }
// }

// impl NamedField for Option<String> {
//     fn with_name<String>(&self, name: &str) {
//         Field<String> {
//             data: self.clone(),
//             name: name.to_string(),
//         }
//     }
// }



// Based on the openapi crate, there are some fields we know they exist or
// deserialization itself would have failed. We do not test for those.
impl OpenAPIValidation for openapi::Spec {
    fn validate(&self, options: &ValidationOptions) -> Result<()>{
        let mut results = ValidationResults::new();


        // results.assert_eq(Field::new(self.swagger, "version"), "2.0");
        // results.assert_exist(Field::new(self.base_path, "basePath"));
        // Field::new(self.base_path, "basePath").assert_exist(&mut results);
        // Field::new(self.swagger, "version").assert_eq("2.0", &mut results);
       // (self.base_path, "basePath").assert_exist(&mut results);

        // self.swagger.with_name("version").assert_exist(&mut results);
        // self.swagger.with_name("version").assert_eq("2.0", &mut results);
   //     let swagger = Field::new(&self.swagger, "version");
     //   swagger.assert_eq("2.0", "version", &mut results);
        // swagger.assert...

        // TODO: warning if host is set

        self.info.validate(options)?;

     //   let base_path = Field::new(&self.base_path, "basePath");
        self.base_path.assert_existence("basePath", &mut results);
        // TODO: warning if not version on path
        // self.base_path.with_name("basePath").assert_regexp(/\v/, &mut results)

        self.schemes.assert_existence("schemes", &mut results);
        assert_array_value(&self.schemes, "schemes", "https")?;


        self.consumes.assert_existence("consumes", &mut results);
        assert_array_value(&self.consumes, "consumes", "application/json")?;

        self.produces.assert_existence("produces", &mut results);
        assert_array_value(&self.produces, "produces", "application/json")?;

        Ok(())

    }
}


impl OpenAPIValidation for openapi::Info {
    fn validate(&self, options: &ValidationOptions) -> Result<()>{
        assert_existence(&self.title, "info.title")?;
        assert_existence(&self.description, "info.description")?;
        if options.support_google_spec {
            assert_existence(&self.version, "info.version")?;
        }
        assert_existence(&self.contact, "info.contact")?;
        assert_existence(&self.contact.as_ref().unwrap().name, "info.contact.name")?;
        assert_existence(&self.contact.as_ref().unwrap().email, "info.contact.email")?;
        // TODO: verify email format
        Ok(())
    }
}




fn assert_array_value(field: &Option<Vec<String>>, field_name: &str, value: &str) -> Result<()>
{
    if field.as_ref().unwrap() != &vec!["application/json".to_string()] {
        bail!("Only the value '{}' is accepted for {} at the top level of the spec", value, field_name);
    }
    Ok(())
}

fn print_warning(warning: &str) {
    // TODO: colors
    println!("WARNING: {}", warning);
}

fn assert_existence<T>(field: &Option<T>, field_name: &str) -> Result<()>
where T: Sized {
    if field.is_none() {
        bail!("{} does not exist in the spec. It needs to be set to a value.", field_name)
    }
    Ok(())
}


