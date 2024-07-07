
use crate::{
    error::ValidateError,
    schema::{schema_type::Type, schema_type_options::ArrayOptions},
};

pub fn validate_array(
    t: &Type,
    json: &Vec<json::JsonValue>,
    key: &String,
) -> Option<ValidateError> {
    match t {
        Type::ArrayType(c) => {
            for o in c.options.iter() {
                match o {
                    ArrayOptions::MaxRange(range) => {
                        if json.len() > *range {
                            return Some(ValidateError::ArrayMaxRange(
                                key.clone(),
                                json.len(),
                                range.clone(),
                            ));
                        }
                    }
                    ArrayOptions::MinRange(range) => {
                        if json.len() < *range {
                            return Some(ValidateError::ArrayMinRange(
                                key.clone(),
                                json.len(),
                                range.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    None
}
