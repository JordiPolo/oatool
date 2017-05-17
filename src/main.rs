extern crate clap;
extern crate openapi;
extern crate google_discovery;

#[macro_use]
extern crate error_chain;

use clap::{Arg, App, AppSettings, SubCommand};

mod validation;
pub mod errors {
    error_chain!{
        foreign_links {
            Parse(::openapi::errors::Error);
            Convert(::google_discovery::errors::Error);
        }
    }
}
use errors::*;

fn parse_spec(path: &str) -> Result<openapi::Spec> {
    Ok(openapi::from_path(path).chain_err(|| "Unable to parse the input file.")?)
}

fn to_json(spec: &openapi::Spec) -> Result<String> {
    Ok(openapi::to_json(&spec).chain_err(|| "Unable to serialize into JSON.")?)
}

fn to_yaml(spec: &openapi::Spec) -> Result<String> {
    Ok(openapi::to_yaml(&spec).chain_err(|| "Unable to serialize into YAML.")?)
}

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

fn merge(files: Vec<&str>) -> Result<()> {
    // let specs : Vec<Result<openapi::Spec>> = files.iter().map(|a| openapi::from_path(a)).collect();
   // let file: String = files.iter().take(1).collect();
   // let spec: openapi::Spec = validation::validate_file(files.iter().take(1).collect())?;
/*    for file in files {
        print!("{:?}", file);
        let spec = openapi::from_path(file)?;
        validation::validate(&spec)?;
    }*/
    Ok(())
}

fn main() {
    let file_arg = Arg::with_name("file")
                .help("OpenAPI spec file")
                .required(true)
                .long("OpenAPI spec file") // seems to do nothing
                .index(1);
    /*
    let stdin_arg = Arg::with_name("stdin")
        .help("Reads from STDIN.")
        .long_help("Redirect files to the STDIN like  oatool < spec.yml")
        .required(false)
        .index(1);
*/
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
        // .subcommand(SubCommand::with_name("merge")
        //     .about("Merges different OpenAPI specs into one")
        //     .arg(Arg::with_name("files")
        //         .help("List of files, comma separated ")
        //         .required(true)
        //         .multiple(true)
        //         .index(1)))
        .get_matches();

    match application.subcommand() {
        ("validate", Some(arguments)) => {
            match validation::validate_file(arguments.value_of("file").unwrap()) {
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


fn convert(filename: &str, from: &str, to: &str) -> Result<String> {
       // let mut result = Ok(String::new());
        let mut openapi_spec: openapi::Spec;
        if from == "openapi" {
            openapi_spec = parse_spec(&filename)?
        } else {
            openapi_spec = google_discovery::convert(google_discovery::from_path(&filename)?)?
        }

        match to {
            "json" => to_json(&openapi_spec),
            "yaml" => to_yaml(&openapi_spec),
            _ => Ok("".to_string())
        }
}