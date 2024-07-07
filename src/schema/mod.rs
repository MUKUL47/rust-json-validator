use std::collections::HashMap;

use schema_type::{AnyType, ArrayType, BooleanType, MatchType, NullType, StringTypeOptions};
use schema_type::{NumberType, ObjectType, Record, StringType, Type};
use schema_type_options::{ArrayOptions, ObjectOptions, StringOptions};

pub mod schema_type;
pub mod schema_type_options;
pub mod schema_validator;
pub type SCHEMA_TYPE = HashMap<String, Vec<(MatchType, Type)>>;

pub struct Schema;

impl Schema {
    pub fn string() -> Type {
        Type::StringTypeOptions(StringTypeOptions { options: vec![] })
    }
    pub fn string_options(options: Vec<StringOptions>) -> Type {
        Type::StringTypeOptions(StringTypeOptions { options })
    }

    pub fn number() -> Type {
        Type::NumberType(NumberType)
    }

    pub fn object(rr: &mut Vec<Record>) -> Type {
        let mut records: HashMap<String, Type> = HashMap::new();
        for a in rr.iter_mut() {
            match a {
                Record::V(k, v) => {
                    records.insert(k.to_string(), v.clone());
                }
            }
        }
        Type::ObjectType(ObjectType {
            records,
            options: vec![],
        })
    }

    pub fn object_options(rr: &mut Vec<Record>, options: Vec<ObjectOptions>) -> Type {
        let mut s = Schema::object(rr);
        match &mut s {
            Type::ObjectType(v) => v.options = options,
            _ => {}
        }
        s
    }
    pub fn array(children: Vec<Type>) -> Type {
        Type::ArrayType(ArrayType {
            children,
            options: vec![],
        })
    }
    pub fn array_options(children: Vec<Type>, options: Vec<ArrayOptions>) -> Type {
        Type::ArrayType(ArrayType { children, options })
    }

    pub fn boolean() -> Type {
        Type::BooleanType(BooleanType)
    }

    pub fn any() -> Type {
        Type::AnyType
    }

    pub fn null() -> Type {
        Type::Null
    }
}
