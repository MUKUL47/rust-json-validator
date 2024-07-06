use crate::schema::schema_type::MatchType;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidateError {
    Expected(String, MatchType, Vec<MatchType>),
    MissingTypes(String, Vec<MatchType>),
    UnexpectedTypes(String),
    StringMisMatch(String, String, Vec<String>)
}

pub struct ErrorController {
    pub errors: Vec<ValidateError>,
}
impl ErrorController {
    pub fn new() -> Self {
        ErrorController { errors: vec![] }
    }
    pub fn throw_error(&mut self, validate_error: ValidateError) {
        self.errors.push(validate_error);
    }
}
