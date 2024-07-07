#[derive(Debug, Clone, PartialEq)]
pub enum StringOptions {
    Required,
    ShouldMatch(&'static str),
    Example(&'static str),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectOptions {
    Required,
    NestedRequired,
    Forbidden(Vec<&'static str>),
    AllowUnknown,
    RequiredFields(Vec<&'static str>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayOptions {
    Required,
    Example(&'static str),
    NestedRequired,
    AllowUnknown,
    MaxRange(usize),
    MinRange(usize),
}
