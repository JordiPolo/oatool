#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_merge_keys;

#[macro_use]
extern crate error_chain;


extern crate yaml_rust;

//#[cfg(feature="serde_yaml")]
//extern crate yaml_merge_keys;

use std::fs;
use std::path::Path;
use std::io::Read;


pub mod schema;
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

//  pub use merge_keys::merge_keys;
//#[cfg(feature="serde_yaml")]
//pub use ::yaml_merge_keys::serde::merge_keys_serde; //serde::merge_keys_serde;

// TODO:
// responses
// SLTs

pub fn to_yaml(spec: &Spec) -> Result<String> {
    Ok(serde_yaml::to_string(spec).chain_err(|| "Unable to serialize into YAML.")?)
}


/// deserialize an google discovery spec file from a path
pub fn from_path<P>(path: P) -> Result<Spec>
    where P: AsRef<Path>
{
    from_reader(fs::File::open(path).chain_err(|| "Can't open google discovery spec file")?)
}

/// deserialize an google discovery spec from type which implements Read
pub fn from_reader<R>(mut read: R) -> Result<Spec>
    where R: Read
{
    //Read from reader into string . TODO: It must be easier than this
    let mut bytes = Vec::new();
    read.read_to_end(&mut bytes).map_err(errors::ErrorKind::Io)?;
    let s = std::str::from_utf8(&bytes).unwrap();

    let docs = yaml_rust::YamlLoader::load_from_str(s).unwrap();

    //TODO: avoid clone , this merges and expands all references into the doc
    let merged = ::yaml_merge_keys::merge_keys(docs[0].clone()).unwrap();

    // Back to a string
    let mut out_str = String::new();
    {
    let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
    emitter.dump(&merged).unwrap();
    }
    //print!("{}", out_str);

    let doc = serde_yaml::from_str::<Spec>(&out_str).chain_err(|| "YAML file is not a valid google discovery file")?;
    Ok(doc)
}


