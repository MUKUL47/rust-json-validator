#[derive(Debug, Clone, PartialEq)]
pub enum StringOptions {
    ShouldMatch(&'static str),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectOptions {
    NestedRequired,
    AllowUnknown,
    RequiredFields(Vec<&'static str>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Options{
    Required,
    Example(&'static str),
    ObjectOptions(ObjectOptions),
}