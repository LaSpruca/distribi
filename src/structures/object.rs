use core::fmt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, error::Error};

/// Type the represents the layout of an object, field names mapped to filed type
pub type Object = HashMap<String, FieldType>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Repersents a basic value that can be used in a object definition
pub enum Primitive {
    Int,
    UInt,
    Float,
    Str,
    Bool,
}

impl TryFrom<&str> for Primitive {
    type Error = PrimitiveFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Int" => Ok(Primitive::Int),
            "UInt" => Ok(Primitive::UInt),
            "Float" => Ok(Primitive::Float),
            "String" => Ok(Primitive::Str),
            "Bool" => Ok(Primitive::Bool),
            _ => Err(PrimitiveFromError {
                bad_val: value.to_string(),
            }),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
// The valid types that a field can be in an object
pub enum FieldType {
    Primitive(Primitive),
    Array(Primitive),
    Map(Primitive, Primitive),
    Object(Object),
}

/// Error thrown if the program fails to convert from a string to a primitive
#[derive(Debug)]
pub struct PrimitiveFromError {
    bad_val: String,
}

impl fmt::Display for PrimitiveFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" is not a valid primitive type", self.bad_val)
    }
}

impl Error for PrimitiveFromError {}
