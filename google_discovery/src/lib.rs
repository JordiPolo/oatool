#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate openapi;
extern crate inflector;

#[macro_use]
extern crate error_chain;

//#[cfg(feature="serde_yaml")]
//extern crate yaml_merge_keys;

use std::fs;
use std::path::Path;
use std::collections::BTreeMap;
use std::io::Read;


mod schema;
pub use schema::*;

pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
            Json(::serde_json::Error);
        }
    }
}
use errors::*;

mod from_google_to_openapi;
mod from_openapi_to_google;

//  pub use merge_keys::merge_keys;
//#[cfg(feature="serde_yaml")]
//pub use ::yaml_merge_keys::serde::merge_keys_serde; //serde::merge_keys_serde;

// TODO:
// responses
// SLTs

pub fn to_yaml(spec: &Spec) -> Result<String> {
    Ok(serde_yaml::to_string(spec).chain_err(|| "Unable to serialize into YAML.")?)
}


/// deserialize an google discovery spec file from a path
pub fn from_path<P>(path: P) -> Result<Spec>
    where P: AsRef<Path>
{
    from_reader(fs::File::open(path).chain_err(|| "Can't open google discovery spec file")?)
}

/// deserialize an google discovery spec from type which implements Read
pub fn from_reader<R>(read: R) -> Result<Spec>
    where R: Read
{
    // let mut bytes = Vec::new();
    // read.read_to_end(&mut bytes).map_err(errors::ErrorKind::Io)?;
    // let s = std::str::from_utf8(&bytes).unwrap();//map_err(Error::str_utf8)?;
    // let value = serde_yaml::from_str(s).unwrap();

    // let merged = ::yaml_merge_keys::merge_keys_serde(value).unwrap();

    // let merged_string: String = serde_yaml::from_value(merged).map_err(errors::ErrorKind::Yaml)?;//.unwrap();
    //         println!("{:?}", merged_string);

    // let doc = serde_yaml::from_str::<Spec>(&merged_string).chain_err(|| "YAML file is not a valid google discovery file")?;

    let doc = serde_yaml::from_reader::<R, Spec>(read).chain_err(|| "YAML file is not a valid google discovery file")?;
    Ok(doc)
    //    Ok(::yaml_merge_keys::serde::merge_keys_serde(doc))
}




impl<'a> From<&'a Spec> for openapi::Spec {
    fn from(spec: &'a Spec) -> Self {
        let google_spec = spec.clone();

        let definitions = google_spec.schemas
            .0
            .iter()
            .map(|(schema_name, google_schema)| {
                (schema_name.to_string(), schema_to_response(google_schema).schema.unwrap())
            })
            .collect();

        // resources is a BTreeMap
        // TODO Remove clones
        let spec_paths = google_spec.resources.0
            .values()
            .flat_map(|resource| resource.methods.values())
            // Fold into a  BTreeMap<String, Vec<Method>>
            .fold(BTreeMap::new(), |mut acc, method|
            {
                let path = format!("/{}", &method.path);
                acc.entry(path).or_insert(vec![]).push(method.clone());
                acc
            })
            //TODO: map on the hash and add method_path properly
            .iter().map(|(method_path, methods)| (method_path.to_string(), openapi::Operations::from(&GoogleMethods(methods.clone()))))
            .collect::<BTreeMap<_, _>>();

        openapi::Spec {
            swagger: "2.0".to_string(),
            schemes: Some(vec!["https".to_string()]),
            base_path: Some(google_spec.base_path.to_string()),
            consumes: Some(vec!["application/json".to_string()]),
            produces: Some(vec!["application/json".to_string()]),
            host: Some("PLEASE.SETHOST.com".to_string()),
            info: openapi::Info::from(&google_spec),
            definitions: Some(definitions),
            paths: spec_paths,

            parameters: None,
            responses: None,
            security_definitions: None,
            tags: None,
        }

    }
}





impl<'a> From<&'a openapi::Spec> for Spec {
    fn from(spec: &'a openapi::Spec) -> Self {
        let openapi_spec = spec.clone();
        let name = spec.info.title.clone().unwrap().to_lowercase();
        let version = openapi_spec.info.version.unwrap();

        Spec {
            id: format!("{}:{}", name, version),
            name: name.to_string(),
            version: version.clone(),
            title: openapi_spec.info.title.unwrap().clone(),
            description: openapi_spec.info.description.unwrap().clone(),
            documentation_link: openapi_spec.info.terms_of_service.clone(),
            protocol: "rest".to_string(),
            base_path: openapi_spec.base_path.unwrap().clone(),
            schemas: from_openapi_to_google::openapi_definitions_to_google_schemas(&openapi_spec.definitions.unwrap()),
            resources: from_openapi_to_google::openapi_paths_to_google_resources(&openapi_spec.paths, &openapi_spec.parameters.unwrap()),
            aliases: None, //from_openapi_to_google::openapi_parameters_to_aliases(&openapi_spec.parameters),
        }
    }
}







fn google_schema_to_openapi_schema(properties: &BTreeMap<String, Property>)
                                   -> BTreeMap<String, openapi::Schema> {
    properties.iter()
        .map(|(property_name, property)| {
            (property_name.to_string(),
             openapi::Schema {
                 description: property.description.clone(),
                 schema_type: property.property_type.clone(),
                 ..Default::default()
             })
        })
        .collect()
}



//Almost copy from  From::Response to Response because the structs are almost copies
fn schema_to_response(schema: &schema::Schema) -> openapi::Response {
    match schema.clone() {
        Schema::ResponseList { schema_type, items, .. } => {
            openapi::Response {
                description: "The operation was successful".to_string(),
                schema: Some(openapi::Schema {
                    ref_path: Some(items.location),
                    schema_type: Some(schema_type),
                    ..Default::default()
                }),
            }
        }
        Schema::ResponseSingle { schema_type, properties, .. } => {
            openapi::Response {
                description: "The operation was successful".to_string(),
                schema: Some(openapi::Schema {
                    schema_type: Some(schema_type),
                    properties: Some(google_schema_to_openapi_schema(&properties)),
                    ..Default::default()
                }),
            }
        }
    }
}
