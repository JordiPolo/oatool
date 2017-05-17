//#![feature(custom_attribute)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate openapi;

#[macro_use]
extern crate error_chain;


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

// TODO:
// <<*
// $ref
// result


/// deserialize an google discovery spec file from a path
pub fn from_path<P>(path: P) -> Result<Spec>
where P: AsRef<Path>,
{
    from_reader(fs::File::open(path).chain_err(|| "Can't open google discovery spec file")?)
}

/// deserialize an google discovery spec from type which implements Read
pub fn from_reader<R>(read: R) -> Result<Spec>
where R: Read,
{
    Ok(serde_yaml::from_reader::<R, Spec>(read).chain_err(|| "YAML file is not a valid google discovery file")?)
}


pub fn convert(google_spec: Spec) -> Result<openapi::Spec>{

    let contact = openapi::Contact {
        name: Some("<YOUR NAME>".to_string()),
        email: Some("<YOUR EMAIL>".to_string()),
        url: None,
    };

    let info = openapi::Info {
        title: Some(google_spec.title.clone()),
        description: Some(google_spec.description),
        version: Some(google_spec.version.clone()),
        terms_of_service: Some(google_spec.documentation_link),
        license: None,
        contact: Some(contact),
    };


    let definitions = google_spec.schemas
        .iter()
        .map(|(schema_name, schema)| {
            (schema_name.to_string(),
             openapi::Schema {
                 schema_type: Some(schema.schema_type.to_string()),
                 properties: Some(google_schema_to_openapi_schema(&schema.properties)),
                 ..Default::default()
             })
        })
        .collect();

    //array of BTreemaps
    let methods: Vec<Method> = google_spec
        .resources
        .values()
        .flat_map(|resource| resource.methods.values())
        .cloned()
        .collect();


    let paths: Vec<String> = methods.iter().map(|method| &method.path).cloned().collect();
    let operations: Vec<openapi::Operations> = methods.iter().map(|method| method_to_operations(method)).collect();
    println!("{:?}", &operations);
    let mut spec_paths: BTreeMap<String, openapi::Operations> = BTreeMap::new();
    for (key, value) in paths.iter().zip(operations) {
        spec_paths.insert(key.to_string(), value);
    }


  //  println!("{:?}", operations);

    let openapi_spec = openapi::Spec {
        swagger: "2.0".to_string(),
        schemes: Some(vec!["https".to_string()]),
        base_path: Some(google_spec.base_path.to_string()),
        consumes: Some(vec!["application/json".to_string()]),
        produces: Some(vec!["application/json".to_string()]),
        host: None,
        info: info,
        definitions: Some(definitions),
        paths: spec_paths,

        parameters: None,
        responses: None,
        security_definitions: None,
        tags: None,
    };

    Ok(openapi_spec)
  //  let openapi_text = openapi::to_yaml(&openapi_spec).unwrap();

    //  let text = serde_yaml::to_string(&google_spec).unwrap();
   // println!("{}", openapi_text);
}



fn google_schema_to_openapi_schema(properties: &BTreeMap<String, Property>)
                               -> BTreeMap<String, openapi::Schema> {
    properties.iter()
        .map(|(property_name, property)| {
            (property_name.to_string(),
             openapi::Schema {
                 description: Some(property.description.clone()),
                 schema_type: Some(property.property_type.clone()),
                 ..Default::default()
             })
        })
        .collect()
}


fn google_params_to_openapi_params(params: &BTreeMap<String, schema::Parameter>) -> Vec<openapi::ParameterOrRef> {

    params.iter().map(|(name, param)|
    openapi::ParameterOrRef::Parameter{
    //    openapi::Parameter {
            name: name.to_string(),
            location: param.location.clone(),
            description: param.description.clone(),
            required: param.required,
            format: None,
            param_type: param.param_type.clone().or_else(|| Some("integer".to_string())),
            schema: None,
            unique_items: None
           // ..Default::default()
        }
    ).collect()

}


fn method_to_operations(method: &schema::Method) -> openapi::Operations {
    let mut base_struct = openapi::Operations {
        .. Default::default()
    };
    let operation = openapi::Operation {
        description: Some(method.description.clone()),
        operation_id: Some(method.id.to_string()),
        parameters: Some(google_params_to_openapi_params(&method.parameters)),
        ..Default::default()
       // method.response
    };
    if method.http_method == "GET" {
        base_struct.get = Some(operation);
    } else if  method.http_method == "POST" {
        base_struct.post = Some(operation);
    }

    base_struct
}
