use core::{core_validator::CoreValidator, schema_validator::SchemaValidator};
mod schema;
use json::parse;
use schema::{
    schema_type::Record,
    schema_type_options::{ArrayOptions, ObjectOptions, Options, StringOptions},
    Schema,
};
mod core;
mod error;
fn main() {
    let a = parse(r#"[{"vv" : 3, "bb" : [132]}, {"vv" : 3, "bb" : [1,"aaaa", 2]}]"#).unwrap();
    let s = Schema::array_options(
        vec![Schema::object_options(
            &mut vec![
                Record::V("aa", Schema::string()),
                Record::V(
                    "bb",
                    Schema::array_options(
                        vec![Schema::string_options(vec![
                            StringOptions::ShouldMatch("123321"),
                            StringOptions::ShouldMatch("aaaa"),
                        ])],
                        vec![ArrayOptions::AllowUnknown],
                    ),
                ),
                Record::V("vv", Schema::number()),
            ],
            vec![
                ObjectOptions::Forbidden(vec!["a", "v"]),
                ObjectOptions::RequiredFields(vec!["aa", "vv"]),
            ],
        )],
        vec![ArrayOptions::MinRange(2), ArrayOptions::NestedRequired],
    );

    let mut pp = SchemaValidator::new();
    pp.parse(s.clone(), vec![]);
    let mut parse = CoreValidator::new(s, pp.hm);
    parse.start(a);
    for e in parse.error_controller.get_errors_messages().iter() {
        println!("{:?}", e); //.len());
    }
}
