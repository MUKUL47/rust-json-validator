use crate::schema::{
    schema_type::{ArrayTypeOptions, MatchType, Type},
    Schema,
};
use std::collections::HashMap;

pub struct SchemaParser {
    pub k: Vec<Vec<String>>,
    pub hm: HashMap<String, Vec<(MatchType, Type)>>,
}
impl SchemaParser {
    pub fn new() -> Self {
        SchemaParser {
            k: vec![],
            hm: HashMap::default(),
        }
    }
    pub fn parse(&mut self, t: Type, kk: Vec<String>) {
        let mut keys: Vec<String> = kk.clone();
        if keys.len() == 0 {
            keys = match t {
                Type::ArrayTypeOptions(_) => vec!["Array".to_string()],
                Type::ObjectType(_) => vec!["Object".to_string()],
                _ => panic!("Expected Array or Object"),
            };
            let k = keys.get(0).unwrap();
            if k == &String::from("Array") {
                self.push(keys.clone(), Schema::array(vec![]));
            } else {
                self.push(keys.clone(), Schema::object(&mut vec![]));
            }
        }
        match t {
            Type::ArrayTypeOptions(v) => {
                for c in v.children.into_iter().enumerate() {
                    let mut vec_clone = keys.clone();
                    vec_clone.push("[INDEX]".to_string());
                    self.push(vec_clone.clone(), c.1.clone());
                    SchemaParser::parse(self, c.1, vec_clone);
                }
            }
            Type::ObjectType(v) => {
                for c in v.records.into_iter().enumerate() {
                    let mut vec_clone = keys.clone();
                    vec_clone.push(c.1 .0);
                    self.push(vec_clone.clone(), c.1 .1.clone());
                    SchemaParser::parse(self, c.1 .1, vec_clone);
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
                    .insert(key, vec![(SchemaParser::get_type_match(&t), t)]);
            }
            Some(v) => {
                v.push((SchemaParser::get_type_match(&t), t));
                self.hm.insert(key, v.to_vec());
            }
        }
    }

    fn get_type_match(t: &Type) -> MatchType {
        match t {
            Type::AnyType(_) => MatchType::Any,
            Type::ArrayTypeOptions(_) => MatchType::Array,
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
