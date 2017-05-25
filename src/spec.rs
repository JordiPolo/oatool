// This module validates that the OpenAPI file is correct

use openapi;
//use error::Result;
use errors::*;

pub fn from_path(path: &str) -> Result<openapi::Spec> {
    Ok(openapi::from_path(path).chain_err(|| "Unable to deserialize the input file.")?)
}

pub fn to_json(spec: &openapi::Spec) -> Result<String> {
    Ok(openapi::to_json(&spec).chain_err(|| "Unable to serialize into JSON.")?)
}

pub fn to_yaml(spec: &openapi::Spec) -> Result<String> {
    Ok(openapi::to_yaml(&spec).chain_err(|| "Unable to serialize into YAML.")?)
}


pub fn validate_file(path: &str) -> Result<openapi::Spec> {
    validate(&from_path(path)?)
}

// Returns the title of the document.
pub fn validate(borrow_spec: &openapi::Spec) -> Result<openapi::Spec> {
    let spec = borrow_spec.clone(); //TODO: Avoid cloning

    let param_ids: Option<Vec<String>> = spec.parameters.map(|params| {
        params.iter().map(|(param_name, _a)| format!("#/parameters/{}", param_name)).collect()
    });

    let def_ids: Option<Vec<String>> = spec.definitions.map(|defs| {
        defs.iter().map(|(def_name, _a)| format!("#/definitions/{}", def_name)).collect()
    });

    spec.info.version.ok_or("No API version found. Must specify info.version")?;
    spec.info.description.ok_or("No API description found. Must specify info.description")?;
    spec.info
        .terms_of_service
        .ok_or("No terms of service found. Must specify info.termsOfService")?;
    spec.info.contact.ok_or("No contact information found. Must specify info.contact")?;

    let schemes = spec.schemes.ok_or("No schemes information found. Add the https scheme")?;
    if schemes != ["https"] {
        Err("Scheme is not https. Only https is a valid scheme.")?;
    }

    for (path_name, methods) in spec.paths.iter() {
        //TODO: Dry
        for method in methods.get.as_ref() {
            validate_method(&path_name, method, &param_ids, &def_ids)?;
        }
        for method in methods.post.as_ref() {
            validate_method(&path_name, method, &param_ids, &def_ids)?;
        }
    }

    Ok(borrow_spec.clone())
}


fn spec_contains(values: &Option<Vec<String>>, key: &str) -> bool {
    values.clone().unwrap().iter().any(|param| param == &key)
}

//TODO:  clean code, return type?
fn validate_method(path: &str,
                   method: &openapi::Operation,
                   all_params: &Option<Vec<String>>,
                   all_definitions: &Option<Vec<String>>)
                   -> Result<()> {
    method.description.as_ref().ok_or(format!("No description for method in {}", path));

    if method.parameters.is_some() {
        let method_params = method.clone().parameters.unwrap();
        for parameter in method_params {
            match parameter {
                openapi::ParameterOrRef::Ref { ref_path } => {
                    if !spec_contains(&all_params, &ref_path) {
                        Err(format!("The parameter {:?} used in {:?} \
                                            is missing from the spec",
                                    ref_path,
                                    path))?;
                    }
                }
                _ => (),
            }
        }
    }

    for (_, response) in method.responses.clone() {
        if response.schema.is_some() {
            let schema = response.schema.unwrap();
            if schema.ref_path.is_some() {
                let ref_path = &schema.ref_path.unwrap();
                if !spec_contains(&all_definitions, &ref_path) {
                    Err(format!("The definition {:?} used in {:?} is missing from the \
                                        spec",
                                ref_path,
                                path))?;
                }
            }
        }
    }
    Ok(())
}

