extern crate openapi;

#[macro_use]
extern crate error_chain;

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

/*
pub struct Spec {
    pub info: Info,
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
// Based on the openapi crate, there are some fields we know they exist or
// deserialization itself would have failed. We do not test for those.
impl OpenAPIValidation for openapi::Spec {
    fn validate(&self, options: &ValidationOptions) -> Result<()>{
        if self.swagger != "2.0" {
            bail!("Version needs to be 2.0.");
        }
        if self.host.is_none() {
            bail!("host needs to be set to a value in the top level of the spec.");
        }
        if self.base_path.is_none() {
            bail!("basePath needs to be set to a value in the top level of the spec.");
        }
        self.info.validate(options)?;

        Ok(())

    }
}

impl OpenAPIValidation for openapi::Info {
    fn validate(&self, options: &ValidationOptions) -> Result<()>{
        if self.title.is_none() {
            bail!("title needs to be set to some value inside the info block.");
        }
        if self.description.is_none() {
            bail!("description needs to be set to some value inside the info block.");
        }
        if self.version.is_none() && options.support_google_spec {
            bail!("description needs to be set to some value inside the info block.");
        }
        if self.contact.is_none() {
            bail!("contact needs to be set to some value inside the info block.");
        }
         if self.contact.is_none() {
            bail!("contact needs to be set to some value inside the info block.");
        }
        if self.contact.as_ref().unwrap().name.is_none() {
            bail!("contact.name needs to be set to some value inside the info block.");
        }
        if self.contact.as_ref().unwrap().email.is_none() {
            bail!("contact.email needs to be set to some value inside the info block.");
        }
        Ok(())
    }
}

