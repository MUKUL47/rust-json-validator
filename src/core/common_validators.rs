use crate::schema::{
    schema_type::{MatchType, Type},
    SCHEMA_TYPE,
};

pub fn get_type(schema: &SCHEMA_TYPE, hash_key: &String, target_type: MatchType) -> Option<Type> {
    match schema.get(hash_key) {
        None => {}
        Some(v) => {
            for i in v.into_iter() {
                if target_type == i.0 {
                    return Some(i.1.clone());
                }
            }
        }
    }
    return None;
}

pub fn has_any(schema: &SCHEMA_TYPE, key: &String) -> bool {
    match schema.get(key) {
        Some(v) => {
            for i in v.into_iter() {
                if i.0 == MatchType::Any {
                    return true;
                }
            }
            return false;
        }
        None => false,
    }
}
