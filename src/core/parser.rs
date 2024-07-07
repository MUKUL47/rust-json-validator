use std::collections::{HashMap, HashSet};
static INDEX: &'static str = "[INDEX]";
use json::JsonValue;

use crate::{
    error::{ErrorController, ValidateError},
    schema::{
        schema_type::{ArrayType, BooleanType, MatchType, MatchTypeString, Type, TypeValidator},
        SCHEMA_TYPE,
    },
};

use super::{common_validators, schema_parser::SchemaParser, string_validator::validate_string};

pub struct Parser {
    first_type: Type,
    schema: SCHEMA_TYPE,
    first_type_val: Vec<String>,
    pub error_controller: ErrorController,
}
impl Parser {
    pub fn new(first_type: Type, schema: HashMap<String, Vec<(MatchType, Type)>>) -> Self {
        let first_type_val = match first_type {
            Type::ArrayTypeOptions(_) => vec!["Array".to_string()],
            Type::ObjectType(_) => vec!["Object".to_string()],
            _ => panic!("Expected Array or Object"),
        };
        Parser {
            first_type,
            schema,
            first_type_val,
            error_controller: ErrorController::new(),
        }
    }

    pub fn start(&mut self, json: json::JsonValue) {
        self.continue_validate(json, self.first_type_val.clone());
    }

    fn continue_validate(&mut self, json: json::JsonValue, keys: Vec<String>) {
        let mut k_clone = keys.clone();
        let hash_key = Parser::get_hash(&k_clone);
        if self.is_missing_type(&hash_key) {
            return;
        }
        match json {
            JsonValue::Array(v) => {
                let mut unknown_allowed: bool = false;
                match self.check_has_type(&hash_key, MatchType::Array) {
                    Some(t) => unknown_allowed = t.allow_unknown(),
                    _ => {}
                }
                k_clone.push(INDEX.to_string());
                let arr_key = Parser::get_hash(&k_clone);
                if common_validators::has_any(&self.schema, &arr_key) {
                    return;
                }
                let mut existing_types = HashSet::new();
                for s in v.into_iter() {
                    let j_type = SchemaParser::get_match_from_json(&s);
                    let j_key = j_type.to_string();
                    existing_types.insert(j_key);
                    if !unknown_allowed {
                        self.continue_validate(s, k_clone.clone());
                    } else {
                        match common_validators::get_type(&self.schema, &arr_key, j_type) {
                            Some(_) => self.continue_validate(s, k_clone.clone()),
                            _ => {}
                        }
                    }
                }
                self.check_missing_types(&arr_key, &existing_types);
            }
            JsonValue::Boolean(_) => {
                self.check_has_type(&hash_key, MatchType::Boolean);
            }
            JsonValue::String(_) | JsonValue::Short(_) => {
                match self.check_has_type(&hash_key, MatchType::String) {
                    Some(_) => match validate_string(&self.schema, &json, &hash_key) {
                        Some(e) => self.throw_error(e),
                        _ => {}
                    },
                    _ => {}
                }
            }
            JsonValue::Null => {
                self.check_has_type(&hash_key, MatchType::Null);
            }
            JsonValue::Number(_) => {
                self.check_has_type(&hash_key, MatchType::Number);
            }
            JsonValue::Object(records) => {
                self.check_has_type(&hash_key, MatchType::Object);
                for (k, v) in records.iter() {
                    let mut cc = k_clone.clone();
                    cc.push(k.to_string());
                    let cc_key = Parser::get_hash(&cc);
                    if self.is_missing_type(&cc_key)
                        || common_validators::has_any(&self.schema, &cc_key)
                    {
                        return;
                    }
                    self.check_has_type(&cc_key, SchemaParser::get_match_from_json(v));
                    self.continue_validate(v.clone(), cc.clone());
                }
            }
        }
    }

    pub fn get_hash(keys: &Vec<String>) -> String {
        return keys.join(".");
    }

    fn check_has_type(&mut self, hash_key: &String, target_type: MatchType) -> Option<Type> {
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
                    hash_key.to_string(),
                    target_type,
                    match_types,
                ));
            }
            _ => {}
        }
        return t;
    }

    fn check_missing_types(&mut self, key: &String, curren_types: &HashSet<String>) {
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
            self.throw_error(ValidateError::MissingTypes(key.clone(), missing_types))
        }
    }

    fn is_missing_type(&mut self, key: &String) -> bool {
        match self.schema.get(key) {
            None => {
                self.throw_error(ValidateError::UnexpectedTypes(key.to_string()));
                return true;
            }
            Some(v) => {
                if v.len() == 0 {
                    self.throw_error(ValidateError::UnexpectedTypes(key.to_string()));
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
