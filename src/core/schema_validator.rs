use crate::schema::{
    schema_type::{MatchType, Type, TypeValidator},
    schema_type_options::{ArrayOptions, NumberOptions, ObjectOptions, Options, StringOptions},
    ARRAY_INDEX, SCHEMA_TYPE,
};
use std::collections::HashMap;

pub struct SchemaValidator {
    pub schema_map: SCHEMA_TYPE,
}
impl SchemaValidator {
    pub fn new() -> Self {
        SchemaValidator {
            schema_map: HashMap::default(),
        }
    }
    pub fn parse(&mut self, current_type: &mut Type) {
        let keys: Vec<String> = vec![current_type.to_string()];
        self.push(&keys, current_type);
        self.start_parsing(current_type, keys, current_type.is_nested_required());
    }

    fn start_parsing(&mut self, t: &mut Type, keys: Vec<String>, nested_required: bool) {
        match t {
            Type::ArrayType(v) => {
                for c in v.children.iter_mut().enumerate() {
                    let mut vec_clone = keys.clone();
                    vec_clone.push(ARRAY_INDEX.to_string());
                    if nested_required {
                        self.update_nested_required(c.1);
                    }
                    self.push(&vec_clone, c.1);
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
                    self.push(&vec_clone, c.1 .1);
                    let mut final_c = c.1 .1;
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

    fn push(&mut self, keys: &Vec<String>, t: &mut Type) {
        let hash_key = keys.join(".");
        match self.schema_map.get(&hash_key) {
            None => {
                self.schema_map.insert(
                    hash_key,
                    vec![(SchemaValidator::get_type_match(&t), t.clone())],
                );
            }
            Some(v) => {
                let mut v_clone = v.clone();
                v_clone.push((SchemaValidator::get_type_match(&t), t.clone()));
                self.schema_map.insert(hash_key, v_clone.to_vec());
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

    pub fn get_schema_map(&self) -> &SCHEMA_TYPE {
        &self.schema_map
    }
}
