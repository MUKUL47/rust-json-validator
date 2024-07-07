use super::schema_type_options::{ArrayOptions, ObjectOptions, StringOptions};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    StringTypeOptions(StringTypeOptions),
    NumberType(NumberType),
    ObjectType(ObjectType),
    ArrayType(ArrayType),
    BooleanType(BooleanType),
    AnyType(AnyType),
    None,
    Null(NullType),
}

pub trait TypeValidator {
    fn is_required(&self) -> bool;
    fn allow_unknown(&self) -> bool;
    fn is_nested_required(&self) -> bool;
}

impl TypeValidator for Type {
    fn is_required(&self) -> bool {
        match self {
            Type::StringTypeOptions(o) => o.options.contains(&StringOptions::Required),
            Type::ArrayType(o) => o.options.contains(&ArrayOptions::Required),
            _ => return false,
        }
    }
    fn allow_unknown(&self) -> bool {
        match self {
            Type::ArrayType(o) => o.options.contains(&ArrayOptions::AllowUnknown),
            _ => return false,
        }
    }

    fn is_nested_required(&self) -> bool {
        match self {
            Type::ArrayType(o) => o.options.contains(&ArrayOptions::NestedRequired),
            _ => return false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    String,
    Array,
    Number,
    Boolean,
    Null,
    Any,
    None,
    Object,
}
pub trait MatchTypeString {
    fn to_string(&self) -> String;
}
impl MatchTypeString for MatchType {
    fn to_string(&self) -> String {
        match self {
            MatchType::String => "String".to_string(),
            MatchType::Array => "Array".to_string(),
            MatchType::Number => "Number".to_string(),
            MatchType::Boolean => "Boolean".to_string(),
            MatchType::Null => "Null".to_string(),
            MatchType::Any => "Any".to_string(),
            MatchType::None => "None".to_string(),
            MatchType::Object => "Object".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CloneType {
    String,
    Array,
    Number,
    Boolean,
    Null,
    Any,
    None,
    Object,
}
pub enum Record {
    V(&'static str, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringType;
#[derive(Debug, Clone, PartialEq)]
pub struct StringTypeOptions {
    pub options: Vec<StringOptions>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberType;

#[derive(Debug, Clone, PartialEq)]
pub struct NullType;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub records: std::collections::HashMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub children: Vec<Type>,
    pub options: Vec<ArrayOptions>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanType;

#[derive(Debug, Clone, PartialEq)]
pub struct AnyType;
