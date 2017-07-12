use openapi;
mod google_to_openapi;
use std::collections::BTreeMap;
use self::google_to_openapi::*;

use google_discovery_spec::*;
use google_discovery_spec::schema::*;


pub fn google_spec_to_openapi(spec: &Spec) -> openapi::Spec {
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
        .iter().map(|(method_path, methods)| (method_path.to_string(), google_to_openapi::methods_to_operations(&GoogleMethods(methods.clone()))))
        .collect::<BTreeMap<_, _>>();

    openapi::Spec {
        swagger: "2.0".to_string(),
        schemes: Some(vec!["https".to_string()]),
        base_path: Some(google_spec.base_path.to_string()),
        consumes: Some(vec!["application/json".to_string()]),
        produces: Some(vec!["application/json".to_string()]),
        host: Some("PLEASE.SETHOST.com".to_string()),
        info: google_to_openapi::from_spec_to_openapi_info(&google_spec),
        definitions: Some(definitions),
        paths: spec_paths,

        parameters: None,
        responses: None,
        security_definitions: None,
        tags: None,
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
fn schema_to_response(schema: &Schema) -> openapi::Response {
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
