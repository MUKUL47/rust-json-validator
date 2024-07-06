use std::collections::HashMap;

use schema_type::{AnyType, ArrayType, ArrayTypeOptions, BooleanType, NullType, StringTypeOptions};
use schema_type::{NumberType, ObjectType, Record, StringType, Type};
use schema_type_options::{ObjectOptions, Options, StringOptions};

pub mod schema_type;
pub mod schema_type_options;
pub mod schema_validator;

pub struct Schema;

impl Schema {
    pub fn string() -> Type {
        Type::StringType(StringType)
    }
    pub fn string_options(options: Vec<Options>) -> Type {
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
        Type::ObjectType(ObjectType { records })
    }
    pub fn array(children: Vec<Type>) -> Type {
        Type::ArrayType(ArrayType { children })
    }
    pub fn array_options(children: Vec<Type>, options: Vec<Options>) -> Type {
        Type::ArrayTypeOptions(ArrayTypeOptions { children, options })
    }

    pub fn boolean() -> Type {
        Type::BooleanType(BooleanType)
    }

    pub fn any() -> Type {
        Type::AnyType(AnyType)
    }

    pub fn null() -> Type {
        Type::Null(NullType)
    }
}
