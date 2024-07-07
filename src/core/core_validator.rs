use std::collections::{HashMap, HashSet};
static INDEX: &'static str = "[INDEX]";
use json::JsonValue;

use crate::{
    error::{ErrorController, ValidateError},
    schema::{
        schema_type::{ArrayType, BooleanType, MatchType, MatchTypeString, Type, TypeValidator},
        schema_type_options::ObjectOptions,
        SCHEMA_TYPE,
    },
};

use super::{
    array_validator, common_validators, schema_validator::SchemaValidator, string_validator::validate_string
};

pub struct CoreValidator {
    schema: SCHEMA_TYPE,
    first_type_val: Vec<String>,
    pub error_controller: ErrorController,
}
impl CoreValidator {
    pub fn new(first_type: Type, schema: HashMap<String, Vec<(MatchType, Type)>>) -> Self {
        let first_type_val = match first_type {
            Type::ArrayType(_) => vec!["Array".to_string()],
            Type::ObjectType(_) => vec!["Object".to_string()],
            _ => panic!("Expected Array or Object"),
        };
        CoreValidator {
            schema,
            first_type_val,
            error_controller: ErrorController::new(),
        }
    }

    pub fn start(&mut self, json: json::JsonValue) {
        self.continue_validate(
            json,
            self.first_type_val.clone(),
            self.first_type_val.clone(),
        );
    }

    fn continue_validate(
        &mut self,
        json: json::JsonValue,
        keys: Vec<String>,
        original_keys: Vec<String>,
    ) {
        let mut k_clone = keys.clone();
        let hash_key = CoreValidator::get_hash(&k_clone);
        let original_hash_key = CoreValidator::get_hash(&original_keys);
        if self.is_missing_type(&hash_key, &original_hash_key) {
            return;
        }
        match json {
            JsonValue::Array(v) => {
                let mut unknown_allowed: bool = false;
                match self.check_has_type(&hash_key, MatchType::Array, &original_hash_key) {
                    Some(t) => {
                        unknown_allowed = t.allow_unknown();
                        match array_validator::validate_array(&t, &v, &original_hash_key) {
                            Some(e) => self.throw_error(e),
                            _ => {}
                        }
                    }
                    _ => {}
                }
                k_clone.push(INDEX.to_string());
                let arr_key = CoreValidator::get_hash(&k_clone);
                if common_validators::has_any(&self.schema, &arr_key) {
                    return;
                }
                let mut existing_types = HashSet::new();
                for (idx, s) in v.into_iter().enumerate() {
                    let j_type = SchemaValidator::get_match_from_json(&s);
                    let j_key = j_type.to_string();
                    let mut new_original_keys = original_keys.clone();
                    new_original_keys.push(idx.to_string());
                    existing_types.insert(j_key);
                    if !unknown_allowed {
                        self.continue_validate(s, k_clone.clone(), new_original_keys);
                    } else {
                        match common_validators::get_type(&self.schema, &arr_key, j_type) {
                            Some(_) => {
                                self.continue_validate(s, k_clone.clone(), new_original_keys)
                            }
                            _ => {}
                        }
                    }
                }
                self.check_missing_types(&arr_key, &existing_types, &original_hash_key);
            }
            JsonValue::Boolean(_) => {
                self.check_has_type(&hash_key, MatchType::Boolean, &original_hash_key);
            }
            JsonValue::String(_) | JsonValue::Short(_) => {
                match self.check_has_type(&hash_key, MatchType::String, &original_hash_key) {
                    Some(_) => {
                        match validate_string(&self.schema, &json, &hash_key, &original_hash_key) {
                            Some(e) => self.throw_error(e),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            JsonValue::Null => {
                self.check_has_type(&hash_key, MatchType::Null, &original_hash_key);
            }
            JsonValue::Number(_) => {
                self.check_has_type(&hash_key, MatchType::Number, &original_hash_key);
            }
            JsonValue::Object(records) => {
                let mut unknown_allowed = false;
                let mut object_keys: HashSet<String> = HashSet::new();
                let mut forbidden_keys: Vec<String> = vec![];
                let mut forbidden_keys_set: HashSet<String> = HashSet::new();
                match common_validators::get_type(&self.schema, &hash_key, MatchType::Object) {
                    Some(v) => {
                        unknown_allowed = v.allow_unknown();
                        match v {
                            Type::ObjectType(r) => {
                                for record in r.records.iter() {
                                    if record.1.is_required() {
                                        object_keys.insert(record.0.to_string());
                                    }
                                }
                                for o in r.options.into_iter() {
                                    match o {
                                        ObjectOptions::Forbidden(k) => {
                                            for s in k.into_iter() {
                                                forbidden_keys_set.insert(s.to_string());
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                for (k, v) in records.iter() {
                    let mut cc = k_clone.clone();
                    let value_obj_type = SchemaValidator::get_match_from_json(&v);
                    cc.push(k.to_string());
                    let mut new_original_keys = original_keys.clone();
                    new_original_keys.push(k.to_string());
                    let cc_key = CoreValidator::get_hash(&cc);
                    if forbidden_keys_set.contains(&k.to_string()) {
                        forbidden_keys.push(CoreValidator::get_hash(&new_original_keys).to_string());
                        continue;
                    }
                    object_keys.remove(&k.to_string());
                    if !unknown_allowed {
                        self.continue_validate(v.clone(), cc.clone(), new_original_keys);
                    } else {
                        match common_validators::get_type(&self.schema, &cc_key, value_obj_type) {
                            Some(next_type) if next_type != Type::AnyType => {
                                self.continue_validate(v.clone(), cc.clone(), new_original_keys)
                            }
                            _ => {}
                        }
                    }
                }
                if forbidden_keys.len() > 0 {
                    self.throw_error(ValidateError::ForbiddenObjectKey(forbidden_keys.clone()));
                }
                if object_keys.len() > 0 {
                    self.throw_error(ValidateError::ObjectMissingKeys(
                        original_hash_key.clone(),
                        object_keys.into_iter().collect(),
                    ))
                }
            }
        }
    }

    pub fn get_hash(keys: &Vec<String>) -> String {
        return keys.join(".");
    }

    fn check_has_type(
        &mut self,
        hash_key: &String,
        target_type: MatchType,
        original_hash_key: &String,
    ) -> Option<Type> {
        let mut match_types: Vec<MatchType> = vec![];
        let mut t: Option<Type> = None;
        match self.schema.get(hash_key) {
            None => {}
            Some(v) => {
                for i in v.into_iter() {
                    if target_type == i.0 {
                        t = Some(i.1.clone());
                    }
                    match_types.push(i.0.clone());
                }
            }
        }
        match t {
            None => {
                self.error_controller.throw_error(ValidateError::Expected(
                    original_hash_key.to_string(),
                    target_type,
                    match_types,
                ));
            }
            _ => {}
        }
        return t;
    }

    fn check_missing_types(&mut self, key: &String, curren_types: &HashSet<String>, original_key: &String){
        let mut missing_types: Vec<MatchType> = Vec::new();
        match self.schema.get(key) {
            Some(v) => {
                for i in v.into_iter() {
                    if !curren_types.contains(&i.0.to_string()) && i.1.is_required() {
                        missing_types.push(i.0.clone());
                    }
                }
            }
            None => {}
        }
        if missing_types.len() > 0 {
            self.throw_error(ValidateError::MissingTypes(original_key.clone(), missing_types))
        }
    }

    fn is_missing_type(&mut self, key: &String, original_key: &String) -> bool {
        match self.schema.get(key) {
            None => {
                self.throw_error(ValidateError::UnexpectedTypeFound(original_key.to_string()));
                return true;
            }
            Some(v) => {
                if v.len() == 0 {
                    self.throw_error(ValidateError::UnexpectedTypeFound(original_key.to_string()));
                    return true;
                }
                return false;
            }
        }
    }

    fn throw_error(&mut self, error: ValidateError) {
        self.error_controller.throw_error(error);
    }
}
