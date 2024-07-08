use std::collections::{HashMap, HashSet};

use schema_type::{ArrayType, BooleanType, MatchType, NullType, StringTypeOptions};
use schema_type::{NumberType, ObjectType, Record, Type};
use schema_type_options::{ArrayOptions, NumberOptions, ObjectOptions, Options, StringOptions};

pub mod schema_type;
pub mod schema_type_options;
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
        Type::NumberType(NumberType { options: vec![] })
    }

    pub fn number_options(options: Vec<NumberOptions>) -> Type {
        Type::NumberType(NumberType { options })
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
            forbidden_hashset: HashSet::new(),
            required_hashset: HashSet::new(),
        })
    }

    pub fn object_options(rr: &mut Vec<Record>, options: Vec<ObjectOptions>) -> Type {
        let mut s = Schema::object(rr);
        match &mut s {
            Type::ObjectType(v) => {
                v.options = options;
                let mut fhs: HashSet<String> = HashSet::new();
                let mut rhs: HashSet<String> = HashSet::new();
                for i in v.options.iter_mut() {
                    match i {
                        ObjectOptions::Forbidden(f) => {
                            for v in f.iter_mut() {
                                fhs.insert(v.to_string());
                            }
                        }
                        ObjectOptions::RequiredFields(f) => {
                            for v in f.iter_mut() {
                                rhs.insert(v.to_string());
                            }
                        }
                        _ => {}
                    }
                }
                v.forbidden_hashset = fhs;
                v.required_hashset = rhs;
            }
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
        Type::BooleanType(BooleanType { options: vec![] })
    }

    pub fn boolean_options(options: Vec<Options>) -> Type {
        Type::BooleanType(BooleanType { options })
    }
    pub fn any() -> Type {
        Type::AnyType
    }

    pub fn null() -> Type {
        Type::Null(NullType { options: vec![] })
    }

    pub fn null_options(options: Vec<Options>) -> Type {
        Type::Null(NullType { options })
    }
}
