//#![feature(custom_attribute)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;


use std::fs;
use std::path::Path;
use std::error::Error;

mod schema;
pub use schema::*;
use std::collections::BTreeMap;




// TODO:
// <<*
// $ref
// result
/*

/// deserialize an google discovery spec file from a path
pub fn from_path<P>(path: P) -> errors::Result<Spec>
where
    P: AsRef<Path>,
{
    from_reader(fs::File::open(path)?)
}

/// deserialize an google discovery spec from type which implements Read
pub fn from_reader<R>(read: R) -> errors::Result<Spec>
where
    R: Read,
{
    Ok(serde_yaml::from_reader::<R, Spec>(read)?)
}

*/
/*

fn read_spec_from_file<P: AsRef<Path>>(path: P) -> Result<Spec, Box<Error>> {
    let file = fs::File::open(path)?;
    let u = serde_yaml::from_reader(file)?;
    Ok(u)
}

fn schema_properties_to_schema(properties: &BTreeMap<String, Property>)
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


fn convert(path: &str) -> Result<Spec, Box<Error>>{
    let google_spec = read_spec_from_file(path).unwrap();

    let contact = openapi::Contact {
        name: Some("Development team".to_string()),
        email: Some("team10@mdsol.com".to_string()),
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
                 properties: Some(schema_properties_to_schema(&schema.properties)),
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

    let openapi_text = openapi::to_yaml(&openapi_spec).unwrap();

    //  let text = serde_yaml::to_string(&google_spec).unwrap();
    println!("{}", openapi_text);
}


*/