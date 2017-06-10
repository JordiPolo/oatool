use openapi;
use schema::*;
use std::collections::BTreeMap;


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct OpenAPIPaths(pub BTreeMap<String, openapi::Operations>);

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct OpenAPIResponses(pub BTreeMap<String, openapi::Response>);

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct OpenAPIParams(pub Vec<openapi::ParameterOrRef>);

const DEFAULT_RESPONSES: [(&str, &str); 2] = [("404", "Resource not found."),
                                              ("500", "Fatal error in the server.")];

// Google spec:
// pub id: String,
// pub name: String,
// pub version: String,
// pub title: String,
// pub description: String,
// #[serde(rename="documentationLink")]
// pub documentation_link: String,
// pub protocol: String,
// #[serde(rename="basePath")]
// pub base_path: String,
impl<'a> From<&'a Spec> for openapi::Info {
    fn from(google_spec: &'a Spec) -> Self {
        let contact = openapi::Contact {
            name: Some("<YOUR NAME>".to_string()),
            email: Some("EMAIL@YOURDOMAIN.COM".to_string()),
            url: None,
        };
        openapi::Info {
            title: Some(google_spec.title.clone()),
            description: Some(google_spec.description.clone()),
            version: Some(google_spec.version.clone()),
            terms_of_service: Some(google_spec.documentation_link.clone()),
            license: None,
            contact: Some(contact),
        }
    }
}





impl<'a> From<&'a GoogleMethods> for openapi::Operations {
    fn from(methods: &'a GoogleMethods) -> Self {

        // &methods.0.fold(openapi::Operations { ..Default::default() }, |operations, method| {
        //     let operation: openapi::Operation = method.into(); //::from(method);

        //     //TODO write the rest, find a better way of doing this
        //     if method.http_method == "GET" {
        //         operations.get = Some(operation);
        //     } else if method.http_method == "POST" {
        //         operations.post = Some(operation);
        //     }
        //     operations
        // })
        let mut base_struct = openapi::Operations { ..Default::default() };

        for method in &methods.0 {
            let operation: openapi::Operation = method.into(); //::from(method);

            //TODO write the rest, find a better way of doing this
            if method.http_method == "GET" {
                base_struct.get = Some(operation);
            } else if method.http_method == "POST" {
                base_struct.post = Some(operation);
            } else if method.http_method == "REPORT" {
                base_struct.post = Some(operation);
            } else if method.http_method == "PUT" {
                base_struct.put = Some(operation);
            }  else if method.http_method == "PATCH" {
                base_struct.patch = Some(operation);
            } else if method.http_method == "DELETE" {
                base_struct.delete = Some(operation);
            } else if method.http_method == "OPTIONS" {
                base_struct.options = Some(operation);
            } else if method.http_method == "HEAD" {
                base_struct.head = Some(operation);
            }
        }

        base_struct
    }
}


impl<'a> From<&'a Method> for openapi::Operation {
    fn from(method: &'a Method) -> Self {
        openapi::Operation {
            description: Some(method.description.clone()),
            operation_id: Some(method.id.to_string()),
            parameters: method.clone().parameters.map(|param| OpenAPIParams::from(&param).0),
            responses: OpenAPIResponses::from(&method.response).0,
            ..Default::default()
        }
    }
}



impl<'a> From<&'a Response> for OpenAPIResponses {
    fn from(response: &'a Response) -> Self {
        let mut responses = DEFAULT_RESPONSES.iter()
            .map(|&(code, description)| {
                (code.to_string(),
                 openapi::Response {
                     description: description.to_string(),
                     schema: None,
                 })
            })
            .collect::<BTreeMap<_, _>>();

         responses.insert("200".to_string(), openapi::Response::from(response));
        OpenAPIResponses(responses)
    }
}

impl<'a> From<&'a Response> for openapi::Response {
    fn from(response: &'a Response) -> Self {
        match response.clone() {
            Response::Reference { location } => {
                openapi::Response {
                    description: "The operation was successful".to_string(),
                    schema: Some(openapi::Schema {
                        ref_path: Some(transform_ref_path(&location)),
                        ..Default::default()
                    }),
                }
            },
            Response::ResponseList { response_type, items, .. } => {
                openapi::Response {
                    description: "The operation was successful".to_string(),
                    schema: Some(openapi::Schema {
                        items: Some(Box::new(openapi::Schema{ref_path: Some(transform_ref_path(&items.location)), ..Default::default()})),
                      //  ref_path: Some(items.location),
                        schema_type: Some(response_type),
                        ..Default::default()
                    }),
                }
            }
            Response::ResponseSingle { response_type, properties, .. } => {
                openapi::Response {
                    description: "The operation was successful".to_string(),
                    schema: Some(openapi::Schema {
                        schema_type: Some(response_type),
                        properties: Some(google_schema_to_openapi_schema(&properties)),
                        ..Default::default()
                    }),
                }
            },
        }

    }
}

// schemas/Region/v1.0.0 -> "#/definitions/Region/v1.0.0"
fn transform_ref_path(google_ref: &str) -> String {
    let pieces: Vec<&str> = google_ref.split("/").collect();
    ["#/definitions", pieces[1]].join("/")
}

//TODO: make this a From implementation
fn google_schema_to_openapi_schema(properties: &BTreeMap<String, Property>)
                                   -> BTreeMap<String, openapi::Schema> {
    properties.iter()
        .map(|(property_name, property)| {
            (property_name.to_string(),
             openapi::Schema {
                 description: property.description.clone(),
                 schema_type: Some(property.property_type.clone()),
                 ..Default::default()
             })
        })
        .collect()
}




impl<'a> From<&'a GoogleParams> for OpenAPIParams {
    fn from(params: &'a GoogleParams) -> Self {
        let openapi_params = params.0
            .iter()
            .map(|(name, param)| {
                openapi::ParameterOrRef::Parameter {
                    //    openapi::Parameter {
                    name: name.to_string(),
                    location: param.location.clone().unwrap_or(param_to_param_location(&param)),
                    description: param.description.clone(),
                    required: Some(param.required.unwrap_or(true)),
                    format: None,
                    param_type: param.param_type.clone().or_else(|| Some("string".to_string())),
                    schema: None,
                    unique_items: None, // ..Default::default()
                }
            })
            .collect();
        OpenAPIParams(openapi_params)
    }
}


fn param_to_param_location(param: &Parameter) -> String {
    if param.required.unwrap_or(true) {
        "path".to_string()
    } else {
        "query".to_string()
    }
}
