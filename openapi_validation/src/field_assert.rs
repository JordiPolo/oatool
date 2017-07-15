use std::fmt::Debug;
use regex::Regex;

use validation_results::ValidationResult;

pub trait Assert {
    type Data: Debug;

    fn data(&self) -> Option<Self::Data>;

    fn name(&self) -> &str;

    fn eq<T>(&self, value: T) -> ValidationResult
    where
        Self::Data: PartialEq<T>,
        T: Debug,
    {
        match self.data() {
            Some(ref a) if a == &value => Ok(()),
            _ => Err(format!(
                "Expected {} to be {:?} but it was {:?}",
                self.name(),
                value,
                self.data()
            )),
        }
    }
}


#[derive(Debug)]
pub struct Field<T> {
    data: T,
    name: String,
}

impl<T> Field<T> {
    pub fn new(data: T, name: &str) -> Field<T> {
        Field {
            data: data,
            name: name.to_string(),
        }
    }
}


impl<'a> Assert for Field<&'a String> {
    type Data = &'a String;

    fn data(&self) -> Option<Self::Data> {
        Some(self.data)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}


impl<'a> Assert for Field<&'a Option<String>> {
    type Data = &'a String;

    fn data(&self) -> Option<Self::Data> {
        self.data.as_ref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}
impl<'a> Field<&'a Option<String>> {

    pub fn length_less_than(&self, size: usize) -> ValidationResult {
        match self.data() {
            None => Ok(()),
            Some(string) => {
                if string.len() > size {
                    Err(format!("The value of {} is longer than {}, for {}", self.name(), size, string))
                } else { Ok(()) }
            }
        }

    }
    pub fn is_match(&self, regex: &Regex) -> ValidationResult {
        match self.data() {
            Some(string) => {
                if !regex.is_match(string) {
                    Err(format!(
                        "The value {} for {} does not follow the proper format of the guidelines",
                        string,
                        self.name()
                    ))
                } else {
                    Ok(())
                }
            }
            None => Ok(()), // Really not ok but some other test should already complain about this.
        }
    }
}



impl<'a> Assert for Field<&'a Option<Vec<String>>> {
    type Data = &'a [String];

    fn data(&self) -> Option<Self::Data> {
        self.data.as_ref().map(|x| &x[..])
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}


//Existence or lack of it is just a property of Option.
impl<'a, T> Field<&'a Option<T>> {
    pub fn exist(&self) -> ValidationResult {
        self.data.as_ref().map(|_| ()).ok_or(format!(
            "{:?} does not exist in the spec. It needs to be set to a value.",
            self.name
        ))
    }
    pub fn not_exist(&self) -> ValidationResult {
        if self.data.as_ref().is_some() {
            Err(format!(
                "{:?} exists in the spec but it should not.",
                self.name
            ))
        } else {
            Ok(())
        }
    }
}
