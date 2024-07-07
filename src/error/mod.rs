use crate::schema::schema_type::{MatchType, MatchTypeString};

#[derive(Debug, Clone, PartialEq)]
pub enum ValidateError {
    Expected(String, MatchType, Vec<MatchType>),
    MissingTypes(String, Vec<MatchType>),
    UnexpectedTypeFound(String),
    StringMisMatch(String, String, Vec<String>),
    ArrayMaxRange(String, usize, usize),
    ArrayMinRange(String, usize, usize),
    ObjectMissingKeys(String, Vec<String>),
    ForbiddenObjectKey(Vec<String>),
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

    pub fn get_errors_messages(&self) -> Vec<String> {
        let mut errors: Vec<String> = vec![];
        for e in self.errors.iter() {
            errors.push(self.get_error_message(e.clone()));
        }
        return errors;
    }

    fn get_error_message(&self, error: ValidateError) -> String {
        match error {
            ValidateError::Expected(a, b, c) => {
                return format!("Expected {:?} at {:?} but found {:?}", c, a, b.to_string());
            }
            ValidateError::MissingTypes(a, b) => {
                return format!("Missing types {:?} at {:?}", b, a);
            }
            ValidateError::UnexpectedTypeFound(a) => {
                return format!("Unexpected property {:?}", a);
            }
            ValidateError::StringMisMatch(a, b, c) => {
                return format!(
                    "String at {:?} doesn't match expected = {:?}, but found = {:?}",
                    a, c, b
                );
            }
            ValidateError::ArrayMaxRange(a, b, c) => {
                return format!(
                    "Array length at {:?} exceeded {:?}, MAX RANGE = {:?}",
                    a, c, b
                );
            }
            ValidateError::ArrayMinRange(a, b, c) => {
                return format!(
                    "Array length at {:?} less than min {:?}, MIN RANGE = {:?}",
                    a, c, b
                );
            }
            ValidateError::ObjectMissingKeys(a, b) => {
                return format!("Object keys at {:?} missing, expected = {:?}", a, b);
            }
            ValidateError::ForbiddenObjectKey(a) => {
                return format!("{:?} properties are forbidden", a);
            }
        }
    }
}
