use openapi;
use schema::*;
use std::collections::BTreeMap;

use inflector::Inflector;

//TODO:
// add defaults
pub fn openapi_definitions_to_google_schemas(definitions: &BTreeMap<String, openapi::Schema>)
                                             -> GoogleSchemas {
    GoogleSchemas(definitions.clone()
        .into_iter()
        .map(|(name, definition)| {
            (name.clone(),
             Schema::ResponseSingle {
                 id: format!("schemas/{}", name.clone()),
                 resource: name.clone().to_snake_case().to_plural(),
                 schema_type: definition.schema_type.unwrap().clone(),
                 properties: openapi_schema_to_google_properties(&definition.properties
                     .unwrap()
                     .clone()),
             })
        })
        .collect())
}


// TODO:  Not need to pass parameters all over the place
pub fn openapi_paths_to_google_resources(paths: &BTreeMap<String, openapi::Operations>, parameters: &BTreeMap<String, openapi::Parameter>)
                                         -> GoogleResources {
    // resource (user)=> [path /user/list, verb GET, operation]
    let mut resources: BTreeMap<String, Vec<(String, String, openapi::Operation)>> = BTreeMap::new();
    let names = vec!["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD"];

    // In Openapi, each path is unique even if the paths are related to the same resource
    // In Google spec, each resource has a number of methods on it, each method will have each own path.
    // This loop transfroms from one to the other.
    for (path, operations) in paths {
        let path_name = path[1..].to_string();
        let operation_methods = vec![operations.clone().get, operations.clone().post, operations.clone().put,
                                    operations.clone().patch, operations.clone().delete, operations.clone().head];
        let names_operation_methods = names.iter().zip(operation_methods);
        for (verb_name, operation) in names_operation_methods {
            if operation.is_some() {
                let oper = operation.clone().unwrap();
                let mut path_group = resources.entry(operation_to_operation_group(&oper)).or_insert(Vec::new());
                path_group.push((path_name.clone(), verb_name.to_string(), oper));
            }
        }
    }

    GoogleResources(
        resources.iter()
        .map(|(resource_name, path_operation_hash)| (resource_name.to_string(), to_google_resource(path_operation_hash, parameters)))
        .collect()
    )
 }

fn to_google_resource(path_operation_hash: &Vec<(String, String, openapi::Operation)>, parameters: &BTreeMap<String, openapi::Parameter>) -> Resource {
    let methods = path_operation_hash.iter().map(|tuple_with_data|
    {
        // TODO: I do not know what I'm doing
        let (ref path, ref method_name, ref operation) = *tuple_with_data;

        let operation_name = operation_to_operation_name(&operation);

        let new_method_name = if operation_name == "report" { "REPORT" } else { &method_name };
        (operation_to_operation_name(&operation),
                        to_google_method(path, operation.clone(), new_method_name, parameters))

    }).collect();

    Resource { methods: methods }
}

fn operation_to_operation_name(operation: &openapi::Operation) -> String {
    let operation_id = operation.clone().operation_id.expect("An operation ID is lacking.");
    let operation_name = operation_id.split(".").last().unwrap().clone();
    operation_name.to_string()
}

fn operation_to_operation_group(operation: &openapi::Operation) -> String {
    let operation_id = operation.clone().operation_id.expect("An operation ID is lacking.");
    let operation_id= operation_id.split(".").nth(0).unwrap().clone();
    operation_id.to_string()
}

fn to_google_method(path: &str, operation: openapi::Operation, method_name: &str, parameters: &BTreeMap<String, openapi::Parameter>) -> Method {
    // TODO: support multiple 2xx here
    Method {
        // We will need to have the operation ids to be added to the spec to do this.
        id: operation.operation_id.unwrap(),
        path: path.to_string(),
        http_method: method_name.to_string(),
        description: operation.description.unwrap(),
        parameters: operation.parameters.clone().map(|param| openapi_param_to_google_param(param, parameters)),
        response: get_successful_response(&operation.responses).map(|r| openapi_response_to_google_response(&r)),
        slt: None,
    }
}

fn get_successful_response(responses: &BTreeMap<String, openapi::Response>) -> Option<openapi::Response> {
    // TODO: support multiple 2xx here
    let mut final_response = None;
    for (status, response) in responses {
        if status.starts_with('2') && response.schema.is_some() {
            final_response = Some(response.to_owned());
        }
    }
    final_response.to_owned()
}


fn openapi_param_to_google_param(params: Vec<openapi::ParameterOrRef>, parameters: &BTreeMap<String, openapi::Parameter>) -> GoogleParams {
    GoogleParams(params.iter()
        .map(|param_or_ref| {
            //println!("{:?}", param_or_ref);
            match param_or_ref.clone() {
                openapi::ParameterOrRef::Parameter { name,
                                                     location,
                                                     description,
                                                     required,
                                                     param_type,
                                                     schema,
                                                     .. } => {
                    (name.to_string(), {
                        let mut base_param = Property {
                            location: Some(location.clone()),
                            description: description.clone(),
                            required: required,
                            property_type: param_type.clone(),
                            ..Default::default()
                        };
                         if schema.is_some() {
                            let the_schema = schema.unwrap();
                            if the_schema.items.is_some() {
                                let the_items = the_schema.items.unwrap();
                                base_param.items = Some(TypeOrReference::Reference { location: transform_ref_path(&the_items.as_ref().clone().ref_path.unwrap())});
                            }
                        }
                        base_param
                     })
                }
                openapi::ParameterOrRef::Ref { ref_path } => {
                    let param_name = ref_path.split("/").last().unwrap();
                    let parameter = parameters.get(param_name).unwrap();
                    (param_name.to_string(),
                     Property {
                         location: Some(parameter.location.clone()),
                         description: parameter.description.clone(),
                         required: parameter.required,
                         property_type: parameter.param_type.clone(),
                         ..Default::default()

                     })
                 //   panic!("CANT understand params ref");
                }
            }
        })
        .collect::<BTreeMap<_, _>>())
}

fn openapi_schema_to_google_properties(schemas: &BTreeMap<String, openapi::Schema>)
                                       -> BTreeMap<String, Property> {

    schemas.iter()
        .map(|(schema_name, schema)| {
           // println!("{}", schema_name);
            (schema_name.to_string(),
             Property {
                 property_type: schema.schema_type.clone(),
                 description: schema.description.clone(),
                 format: None,
                 items: None,
                 ..Default::default()
             })
        })
        .collect()

}


fn openapi_response_to_google_response(response: &openapi::Response) -> Response {
    let schema = response.clone().schema.unwrap();

    if schema.ref_path.is_some() {
        Response::Reference { location: transform_ref_path(&schema.ref_path.unwrap()) }
        // TODO: all the rest
    } else if schema.items.is_some() {
        Response::ResponseList {
            id: None, //Some("NOTSETLIST".to_string()),
            resource: None, //Some("NOTSETLIST".to_string()),
            response_type: schema.schema_type.clone().unwrap(),
            items: Reference { location: transform_ref_path(&schema.items.unwrap().as_ref().clone().ref_path.unwrap()) },
        }
    } else if schema.properties.is_some() {
        Response::ResponseSingle {
            id: "NOTSET".to_string(),
            resource: "NOTSET".to_string(),
            response_type: schema.schema_type.unwrap(),
            properties: openapi_schema_to_google_schema(&schema.properties.unwrap()),
        }
    } else {
        panic!("Unknown response")
    }
}

// "#/definitions/Region" -> schemas/Region
fn transform_ref_path(openapi_ref: &str) -> String {
    let pieces: Vec<&str> = openapi_ref.split('/').collect();
    ["schemas", pieces.last().unwrap()].join("/")
}

fn openapi_schema_to_google_schema(schemas: &BTreeMap<String, openapi::Schema>)
                                   -> BTreeMap<String, Property> {
    schemas.iter()
        .map(|(schema_name, schema)| {
            (schema_name.to_string(),
             Property {
                 description: schema.description.clone(),
                 property_type: schema.schema_type.clone(),
                 format: None,
                 items: None,
                ..Default::default()
             })
        })
        .collect()
}
