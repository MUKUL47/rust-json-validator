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
    let a = parse(r#"[{"names" : 2}]"#).unwrap();
    let s = Schema::array_options(
        vec![Schema::object(&mut vec![
            Record::V(
                "names",
                Schema::string_options(vec![StringOptions::ShouldMatch("222")]),
            ),
            Record::V("nameas", Schema::array(vec![])),
        ])],
        vec![ArrayOptions::MinRange(10), ArrayOptions::NestedRequired],
    );

    let mut pp = SchemaParser::new();
    pp.parse(s.clone(), vec![]);
    println!("{:?}", pp.hm);
    let mut parse = Parser::new(s, pp.hm);
    parse.start(a);
    println!("{:?}", parse.error_controller.errors) //.len());
}
