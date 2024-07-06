use core::{parser::Parser, schema_parser::SchemaParser};
use std::{
    any::{self, Any},
    collections::HashMap,
};
mod schema;
use json::{number::Number, object::Object, parse};
use schema::{
    schema_type::Record,
    schema_type_options::{ObjectOptions, Options, StringOptions},
    Schema,
};
mod core;
mod error;
fn main() {
    let a = parse(r#"["1ss23"]"#).unwrap();
    let s = Schema::array(vec![Schema::string_options(vec![
        StringOptions::ShouldMatch("123"),
        StringOptions::ShouldMatch("1s23"),
    ])]);

    let mut pp = SchemaParser::new();
    pp.parse(s.clone(), vec![]);
    println!("{:?}", pp.hm);
    let mut parse = Parser::new(s, pp.hm);
    parse.start(a);
    println!("{:?}", parse.error_controller.errors) //.len());
}
