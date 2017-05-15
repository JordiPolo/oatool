extern crate clap;
extern crate openapi;
extern crate serde;


use clap::{Arg, App, AppSettings, SubCommand};

pub mod error;
use error::Result;
use error::OpenApiError;


fn validate_stdin() -> Result<String> {
    let spec = openapi::from_reader(std::io::stdin())?;
    validate(&spec)
}

//TODO: validate definitions, clean code, return type?
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
                    if !all_params.clone().unwrap().iter().any(|param| param == &ref_path) {
                        None.ok_or(format!("The definition of the parameter {:?} used in {:?} \
                                            is missing in the spec",
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
                if !all_definitions.clone()
                    .unwrap()
                    .iter()
                    .any(|definition| definition == ref_path) {
                    None.ok_or(format!("The definition of {:?} used in {:?} is missing in the spec",
                                    ref_path,
                                    path))?;
                }
            }
        }
    }
    Ok(())
}

//TODO
// check all definitions are there
// check all params are there
// Returns the title of the document.
fn validate(borrow_spec: &openapi::Spec) -> Result<String> {
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

    Ok(spec.info.title.to_owned().unwrap())
}

fn to_json() -> Result<String> {
    let spec = openapi::from_reader(std::io::stdin())?;
    Ok(openapi::to_json(&spec)?)
}

fn to_yaml() -> Result<String> {
    let spec = openapi::from_reader(std::io::stdin())?;
    Ok(openapi::to_yaml(&spec)?)
}

fn exit_with_error(error: OpenApiError, extra_error_message: &str) {
    use std::io::Write;
    writeln!(&mut std::io::stderr(), "{}", error.to_string()).unwrap();
    writeln!(&mut std::io::stderr(), "{}", extra_error_message).unwrap();
    std::process::exit(-1);
}

fn merge(files: Vec<&str>) -> Result<()> {
    // let specs : Vec<Result<openapi::Spec>> = files.iter().map(|a| openapi::from_path(a)).collect();
    for file in files {
        print!("{:?}", file);
        let spec = openapi::from_path(file)?;
        validate(&spec)?;
    }
    Ok(())
}

fn main() {
    // TODO: Support this
    /*
    let file_arg = Arg::with_name("file")
                .help("OpenAPI spec file")
                .required(false)
                .long("OpenAPI spec file") // seems to do nothing
                .index(1);
    */



    let stdin_arg = Arg::with_name("stdin")
        .help("Reads from STDIN.")
        .long_help("Redirect files to the STDIN like  oatool < spec.yml")
        .required(false)
        .index(1);

    let application = App::new("oatool")
        .version("0.1.0")
        .about("A tool to manage OpenAPI files")
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(SubCommand::with_name("validate")
            .about("Validates the OpenAPI input following opionated rules")
            .arg(&stdin_arg))
        .subcommand(SubCommand::with_name("to_json")
            .about("Translates OpenAPI input to JSON")
            .arg(&stdin_arg))
        .subcommand(SubCommand::with_name("to_yaml")
            .about("Translates OpenAPI input to YAML")
            .arg(&stdin_arg))
        .subcommand(SubCommand::with_name("merge")
            .about("Merges different OpenAPI specs into one")
            .arg(Arg::with_name("files")
                .help("List of files, comma separated ")
                .required(true)
                .multiple(true)
                .index(1)))
        .get_matches();

    match application.subcommand() {
        ("validate", Some(_arguments)) => {
            match validate_stdin() {
                Ok(title) => println!("Validation of {} successful!", title),
                Err(e) => exit_with_error(e, "Validation failed"),
            }
        }
        ("to_json", Some(_arguments)) => {
            match to_json() {
                Ok(text) => println!("{}", text),
                Err(e) => exit_with_error(e, "Translation to JSON failed"),
            }
        }
        ("to_yaml", Some(_arguments)) => {
            match to_yaml() {
                Ok(text) => println!("{}", text),
                Err(e) => exit_with_error(e, "Translation to YAML failed"),
            }
        }
        ("merge", Some(arguments)) => {
            match merge(arguments.values_of("files").unwrap().collect()) {
                Ok(_) => println!("Files merged into one.yml"),
                Err(e) => exit_with_error(e, "Merging failed."),
            }
        }
        _ => println!("{}", application.usage()),
    }

    std::process::exit(0);
}
