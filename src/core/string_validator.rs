use std::collections::HashSet;

use crate::{
    error::{ValidateError},
    schema::{
        schema_type::{Type},
        schema_type_options::StringOptions,
        SCHEMA_TYPE,
    },
};

pub fn validate_string(
    schema: &SCHEMA_TYPE,
    value: &json::JsonValue,
    key: &String,
    original_key: &String,
) -> Option<ValidateError> {
    if let Some(v) = schema.get(key) {
        for i in v.iter() {
            if let Type::StringTypeOptions(c) = &i.1 {
                let available_strs: HashSet<_> = c
                    .options
                    .iter()
                    .filter_map(|option| {
                        if let StringOptions::ShouldMatch(v) = option {
                            Some(v.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                if available_strs.len() > 0 && !available_strs.contains(&value.to_string()) {
                    return Some(ValidateError::StringMisMatch(
                        original_key.to_string(),
                        value.to_string(),
                        available_strs.into_iter().collect(),
                    ));
                }
            }
        }
    }
    None
}
