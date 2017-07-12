#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate error_chain;

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
pub fn from_reader<R>(read: R) -> Result<Spec>
    where R: Read
{
    // let mut bytes = Vec::new();
    // read.read_to_end(&mut bytes).map_err(errors::ErrorKind::Io)?;
    // let s = std::str::from_utf8(&bytes).unwrap();//map_err(Error::str_utf8)?;
    // let value = serde_yaml::from_str(s).unwrap();

    // let merged = ::yaml_merge_keys::merge_keys_serde(value).unwrap();

    // let merged_string: String = serde_yaml::from_value(merged).map_err(errors::ErrorKind::Yaml)?;//.unwrap();
    //         println!("{:?}", merged_string);

    // let doc = serde_yaml::from_str::<Spec>(&merged_string).chain_err(|| "YAML file is not a valid google discovery file")?;

    let doc = serde_yaml::from_reader::<R, Spec>(read).chain_err(|| "YAML file is not a valid google discovery file")?;
    Ok(doc)
    //    Ok(::yaml_merge_keys::serde::merge_keys_serde(doc))
}


