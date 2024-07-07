use crate::schema::{
    schema_type::{ArrayType, MatchType, Type, TypeValidator},
    schema_type_options::{ArrayOptions, NumberOptions, ObjectOptions, Options, StringOptions},
    Schema, SCHEMA_TYPE,
};
use std::collections::HashMap;

pub struct SchemaValidator {
    pub k: Vec<Vec<String>>,
    pub hm: SCHEMA_TYPE,
}
impl SchemaValidator {
    pub fn new() -> Self {
        SchemaValidator {
            k: vec![],
            hm: HashMap::default(),
        }
    }
    pub fn parse(&mut self, t: Type, kk: Vec<String>) {
        let mut keys: Vec<String> = kk.clone();
        if keys.len() == 0 {
            keys = match t {
                Type::ArrayType(_) => vec!["Array".to_string()],
                Type::ObjectType(_) => vec!["Object".to_string()],
                _ => panic!("Expected Array or Object"),
            };
            self.push(keys.clone(), t.clone());
        }
        self.start_parsing(&mut t.clone(), keys, t.is_nested_required());
    }

    fn start_parsing(&mut self, t: &mut Type, keys: Vec<String>, nested_required: bool) {
        match t {
            Type::ArrayType(v) => {
                for c in v.children.iter_mut().enumerate() {
                    let mut vec_clone = keys.clone();
                    vec_clone.push("[INDEX]".to_string());
                    if nested_required {
                        self.update_nested_required(c.1);
                    }
                    self.push(vec_clone.clone(), c.1.clone());
                    let mut final_c = c.1; //.clone();
                    let nested = final_c.is_nested_required();
                    self.start_parsing(&mut final_c, vec_clone, nested_required || nested);
                }
            }
            Type::ObjectType(v) => {
                for c in v.records.clone().iter_mut().enumerate() {
                    let mut vec_clone = keys.clone();
                    vec_clone.push(c.1 .0.clone());
                    if nested_required {
                        self.update_nested_required(c.1 .1);
                    }
                    self.push(vec_clone.clone(), c.1 .1.clone());
                    let mut final_c = c.1 .1; //.clone();
                    let nested = final_c.is_nested_required();
                    self.start_parsing(&mut final_c, vec_clone, nested_required || nested);
                }
            }
            _ => {}
        }
    }

    fn update_nested_required(&mut self, t: &mut Type) {
        match t {
            Type::StringTypeOptions(v) => v.options.push(StringOptions::Required),
            Type::NumberType(v) => v.options.push(NumberOptions::Required),
            Type::BooleanType(v) => v.options.push(Options::Required),
            Type::Null(v) => v.options.push(Options::Required),
            Type::ArrayType(v) => v.options.push(ArrayOptions::Required),
            Type::ObjectType(v) => {
                v.options.push(ObjectOptions::Required);
                for r in v.records.iter_mut() {
                    self.update_nested_required(r.1);
                }
            }
            _ => {}
        }
    }

    fn push(&mut self, keys: Vec<String>, t: Type) {
        let key = keys.join(".");
        let mut cc = self.hm.clone();
        match cc.get_mut(&key) {
            None => {
                self.hm
                    .insert(key, vec![(SchemaValidator::get_type_match(&t), t)]);
            }
            Some(v) => {
                v.push((SchemaValidator::get_type_match(&t), t));
                self.hm.insert(key, v.to_vec());
            }
        }
    }

    fn get_type_match(t: &Type) -> MatchType {
        match t {
            Type::AnyType => MatchType::Any,
            Type::ArrayType(_) => MatchType::Array,
            Type::BooleanType(_) => MatchType::Boolean,
            Type::ObjectType(_) => MatchType::Object,
            Type::StringTypeOptions(_) => MatchType::String,
            Type::None => MatchType::None,
            Type::Null(_) => MatchType::Null,
            Type::NumberType(_) => MatchType::Number,
        }
    }

    pub fn get_match_from_json(jv: &json::JsonValue) -> MatchType {
        match jv {
            json::JsonValue::Array(_) => MatchType::Array,
            json::JsonValue::Boolean(_) => MatchType::Boolean,
            json::JsonValue::String(_) | json::JsonValue::Short(_) => MatchType::String,
            json::JsonValue::Null => MatchType::Null,
            json::JsonValue::Number(_) => MatchType::Number,
            json::JsonValue::Object(_) => MatchType::Object,
            _ => MatchType::None,
        }
    }
}
