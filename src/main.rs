extern crate clap;
extern crate openapi;
extern crate google_discovery;

#[macro_use]
extern crate error_chain;

use clap::{Arg, App, AppSettings, SubCommand};

mod spec;

pub mod errors {
    error_chain!{
        foreign_links {
            Parse(::openapi::errors::Error);
            Convert(::google_discovery::errors::Error);
        }
    }
}
use errors::*;

fn exit_with_error(error: Error, extra_error_message: &str) {
    use std::io::Write;

    writeln!(&mut std::io::stderr(), "{}", extra_error_message).unwrap();
    writeln!(&mut std::io::stderr(), "↳ {}", error.to_string()).unwrap();
    for (i, e) in error.iter().enumerate().skip(1) {
        let ident = " ".repeat(i);
        writeln!(&mut std::io::stderr(), "{}↳ {}", ident, e).unwrap();
    }
    std::process::exit(-1);
}


fn main() {
    let file_arg = Arg::with_name("file")
                .help("OpenAPI spec file")
                .required(true)
                .long("OpenAPI spec file") // seems to do nothing
                .index(1);

    let application = App::new("oatool")
        .version("0.1.0")
        .about("A tool to manage OpenAPI files")
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(SubCommand::with_name("validate")
            .about("Validates an OpenAPI spec file following opionated rules")
            .arg(&file_arg))
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
                .possible_values(&["openapi_yaml", "openapi_json"])
                .help("Sets the format to convert the file to.")))
        .get_matches();

    match application.subcommand() {
        ("validate", Some(arguments)) => {
            match spec::validate_file(arguments.value_of("file").unwrap()) {
                Ok(spec) => println!("Validation of {} successful!", spec.info.title.unwrap()),
                Err(e) => exit_with_error(e, "Validation failed"),
            }
        }
        ("convert", Some(arguments)) => {
            let filename = arguments.value_of("file").unwrap();
            let from = arguments.value_of("from").unwrap();
            let to = arguments.value_of("to").unwrap();

            match convert(filename, from, to) {
                Ok(text) => println!("{}", text),
                Err(e) => exit_with_error(e, &format!("Convertion from {} to {} failed", &from, &to)),
            }
        }
        _ => println!("{}", application.usage()),
    }

    //TODO: Isn't this the default?
    std::process::exit(0);
}


fn convert(filename: &str, from: &str, to: &str) -> Result<String> {
        let openapi_spec = if from == "openapi" {
            spec::from_path(&filename)?
        } else {
            google_discovery::google_to_openapi(google_discovery::from_path(&filename)?)?
        };

        if to == "openapi_json" {
            spec::to_json(&openapi_spec)
        } else {
            spec::to_yaml(&openapi_spec)
        }
}