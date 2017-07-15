extern crate clap;
extern crate openapi;
extern crate convert_google_spec;
extern crate google_discovery_spec;
extern crate openapi_validation;

#[macro_use]
extern crate error_chain;

use clap::{Arg, App, AppSettings, SubCommand};
use std::io::Write;

mod spec;

pub mod errors {
    error_chain!{
        foreign_links {
            Parse(::openapi::errors::Error);
            GoogleSpec(::google_discovery_spec::errors::Error);
       //     Validation(::openapi_validation::errors::Error);
        }
    }
}
use errors::*;

use openapi_validation::{OpenAPIValidation, ValidationOptions};


fn exit_with_error(error: &Error, extra_error_message: &str) {
    writeln!(&mut std::io::stderr(), "{}", extra_error_message).unwrap();
    writeln!(&mut std::io::stderr(), "↳ {}", error.to_string()).unwrap();
    for (i, e) in error.iter().enumerate().skip(1) {
        let ident = " ".repeat(i);
        writeln!(&mut std::io::stderr(), "{}↳ {}", ident, e).unwrap();
    }
    std::process::exit(-1);
}

fn exit_on_validation_error(openapi_spec: &openapi::Spec, options: &ValidationOptions) {
    let results = openapi_spec.validate(options);
    if results.failed() {
        writeln!(&mut std::io::stderr(), "Validation results: {}", results).unwrap();
        std::process::exit(-1);
    }
}


fn main() {
    let file_arg = Arg::with_name("file")
                .help("OpenAPI spec file")
                .required(true)
                .long("OpenAPI spec file") // seems to do nothing
                .index(1);

    let application = App::new("oatool")
        .version("0.7.0")
        .about("A tool to manage OpenAPI files")
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(SubCommand::with_name("validate")
            .about("Validates an OpenAPI file.")
            .arg(&file_arg)
            .arg(Arg::with_name("support_google")
                .long("support_google")
                .takes_value(false)
                .required(false)
                .help("Validates an openapi file which can be converted to google (or not)."))
        )
        .subcommand(SubCommand::with_name("convert")
            .about("Translates an API spec file to other format.")
            .arg(&file_arg)
            .arg(Arg::with_name("from")
                .long("from")
                .takes_value(true)
                .require_equals(true)
                .required(true)
                .possible_values(&["openapi", "google"])
                .help("Sets the format to convert the file from."))
            .arg(Arg::with_name("to")
                .long("to")
                .takes_value(true)
                .require_equals(true)
                .required(true)
                .possible_values(&["openapi_yaml", "openapi_json", "google"])
                .help("Sets the format to convert the file to.")))
        .get_matches();

    match application.subcommand() {
        ("validate", Some(arguments)) => {
            let filename = arguments.value_of("file").unwrap();
            let openapi_spec = spec::from_path(filename).unwrap();
            let options = ValidationOptions{ support_google_spec: arguments.is_present("support_google"), ..Default::default() };
            exit_on_validation_error(&openapi_spec, &options);
            println!("Your file passed the validation. Congrats!");
        }
        ("convert", Some(arguments)) => {
            let filename = arguments.value_of("file").unwrap();
            let from = arguments.value_of("from").unwrap();
            let to = arguments.value_of("to").unwrap();

            match convert(filename, from, to) {
                Ok(text) => println!("{}", text),
                Err(e) => exit_with_error(&e, &format!("Convertion from {} to {} failed", &from, &to)),
            }
        }
        _ => println!("{}", application.usage()),
    }

    //TODO: Isn't this the default?
    std::process::exit(0);
}


fn convert(filename: &str, from: &str, to: &str) -> Result<String> {
        let openapi_spec = if from == "openapi" {
            spec::from_path(filename)?
        } else {
            convert_google_spec::google_to_openapi::google_spec_to_openapi(&google_discovery_spec::from_path(filename)?)
           // openapi::Spec::from(&google_discovery::from_path(filename)?)
        };

        if to == "openapi_json" {
            spec::to_json(&openapi_spec)
        } else if to == "openapi_yaml" {
            spec::to_yaml(&openapi_spec)
        } else { // to google
            exit_on_validation_error(&openapi_spec, &ValidationOptions{ support_google_spec: true, ..Default::default() });
            // TODO: should not need thsi chain_err here
            google_discovery_spec::to_yaml(&convert_google_spec::openapi_to_google::openapi_spec_to_google(openapi_spec)).chain_err(|| "Unable to serialize into YAML.")
        }
}
