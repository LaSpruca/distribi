mod object;

use core::fmt;
use std::{collections::HashMap, convert::TryFrom, error::Error, fs::read_to_string};

pub use object::*;

pub fn parse(source: &str, file_name: &str) -> Result<Object, ParseError> {
    let space = regex::Regex::new("[ ]+").unwrap();

    let mut obj_definition = Object::new();
    let mut imported_objects = HashMap::<String, Object>::new();

    for line in source.split("\n") {
        if line.starts_with("#") {
            continue;
        }

        let fields = space.split(line).collect::<Vec<&str>>();

        if fields.len() == 2 {
            let name = fields[0];
            let field_type = fields[1];

            if name.starts_with("@") {
                let import_filename = format!("structures/{}.dst", name.replace("@", ""));
                println!("Importing {}", import_filename);
                let source = match read_to_string(import_filename.as_str()) {
                    Err(error) => {
                        return Err(ParseError {
                            file: file_name.to_string(),
                            error: error.into(),
                        })
                    }
                    Ok(val) => val,
                };
                imported_objects.insert(
                    field_type.to_string(),
                    parse(&source, import_filename.as_str()).unwrap(),
                );
                continue;
            }

            if field_type.contains("->") {
                let types = field_type.split("->").collect::<Vec<&str>>();
                obj_definition.insert(
                    name.to_string(),
                    FieldType::Map(
                        match Primitive::try_from(types[0]) {
                            Err(error) => {
                                return Err(ParseError {
                                    file: file_name.to_string(),
                                    error: error.into(),
                                })
                            }
                            Ok(val) => val,
                        },
                        match Primitive::try_from(types[1]) {
                            Err(error) => {
                                return Err(ParseError {
                                    file: file_name.to_string(),
                                    error: error.into(),
                                })
                            }
                            Ok(val) => val,
                        },
                    ),
                );
            } else if field_type.starts_with("@") {
                let name = field_type.replace("@", "");
                let object = match imported_objects.get(&name) {
                    None => {
                        return Err(ParseError {
                            file: file_name.to_string(),
                            error: InvalidObjectName { name }.into(),
                        })
                    }
                    Some(val) => val,
                };
                obj_definition.insert(name, FieldType::Object(object.to_owned()));
            } else if field_type.starts_with(";") {
                let t = field_type.replace(";", "");
                obj_definition.insert(
                    name.to_string(),
                    FieldType::Array(match Primitive::try_from(t.as_str()) {
                        Err(error) => {
                            return Err(ParseError {
                                file: file_name.to_string(),
                                error: error.into(),
                            })
                        }
                        Ok(val) => val,
                    }),
                );
            } else {
                obj_definition.insert(
                    name.to_string(),
                    FieldType::Primitive(match Primitive::try_from(field_type) {
                        Err(error) => {
                            return Err(ParseError {
                                file: file_name.to_string(),
                                error: error.into(),
                            })
                        }
                        Ok(val) => val,
                    }),
                );
            }
        }
    }

    Ok(obj_definition)
}

#[derive(Debug)]
pub struct InvalidObjectName {
    name: String,
}

impl fmt::Display for InvalidObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The name '{0}' has not been imported, please use '@[file] {0}' to import it",
            self.name
        )
    }
}

impl Error for InvalidObjectName {}

#[derive(Debug)]
pub struct ParseError {
    file: String,
    error: Box<dyn Error>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error parsing file, {}, Gave error {}",
            self.file, self.error
        )
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.error)
    }
}
