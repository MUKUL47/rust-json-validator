use core::{parser::Parser, schema_parser::SchemaParser};
use std::{
    any::{self, Any},
    collections::HashMap,
};
mod schema;
use json::{number::Number, object::Object, parse};
use schema::{
    schema_type::{ArrayType, ArrayTypeOptions, Record},
    schema_type_options::{ArrayOptions, ObjectOptions, StringOptions},
    Schema,
};
mod core;
mod error;
fn main() {
    let a = parse(r#"[[]]"#).unwrap();
    let s = Schema::array_options(
        vec![
            Schema::string(),
            Schema::array_options(
                vec![
                    Schema::string(),
                    Schema::array(vec![Schema::string_options(vec![])]),
                ],
                vec![ArrayOptions::NestedRequired, ArrayOptions::Required],
            )
        ],
        vec![],
    );

    let mut pp = SchemaParser::new();
    pp.parse(s.clone(), vec![]);
    println!("{:?}", s);
    let mut parse = Parser::new(s, pp.hm);
    parse.start(a);
    println!("{:?}", parse.error_controller.errors) //.len());
}
