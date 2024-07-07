use core::{parser::Parser, schema_parser::SchemaParser};
use std::{
    any::{self, Any},
    collections::HashMap,
};
mod schema;
use json::{number::Number, object::Object, parse};
use schema::{
    schema_type::{ArrayType, Record},
    schema_type_options::{ArrayOptions, ObjectOptions, StringOptions},
    Schema,
};
mod core;
mod error;
fn main() {
    let a = parse(r#"["123","123","123","123","123",2]"#).unwrap();
    let s = Schema::array_options(
        vec![
            Schema::string(),
        ],
        vec![ArrayOptions::MinRange(10)],
    );

    let mut pp = SchemaParser::new();
    pp.parse(s.clone(), vec![]);
    println!("{:?}", s);
    let mut parse = Parser::new(s, pp.hm);
    parse.start(a);
    println!("{:?}", parse.error_controller.errors) //.len());
}
