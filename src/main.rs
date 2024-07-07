use core::{core_validator::CoreValidator, schema_validator::SchemaValidator};
mod schema;
use json::{parse};
use schema::{
    schema_type_options::{ArrayOptions, Options},
    Schema,
};
mod core;
mod error;
fn main() {
    let a = parse(r#"[null,[2],[],[]]"#).unwrap();
    let s = Schema::array_options(
        vec![
            Schema::null_options(vec![Options::Required]),
            Schema::array(vec![Schema::number()]),
        ],
        vec![ArrayOptions::MinRange(2), ArrayOptions::NestedRequired],
    );

    let mut pp = SchemaValidator::new();
    pp.parse(s.clone(), vec![]);
    println!("{:?}", pp.hm);
    let mut parse = CoreValidator::new(s, pp.hm);
    parse.start(a);
    println!("{:?}", parse.error_controller.errors) //.len());
}
