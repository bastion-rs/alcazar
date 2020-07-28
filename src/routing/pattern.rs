use crate::error::RoutingError::RegexCompileError;
use crate::error::{AlcazarError, Result, RoutingError};
use lazy_static::lazy_static;
use regex::{escape, Regex};
use std::str::FromStr;

lazy_static! {
    static ref ANY_VALUE_REGEX: Regex = Regex::new(r"[^{}/]+").unwrap();
    static ref DYN_PARAM_REGEX: Regex = Regex::new(r"({\s*[\w\d_]+\s*})").unwrap();
    static ref VALID_DYN_PARAM_REGEX: Regex = Regex::new(r"{(?P<part>[\w][\w\d_]*)}").unwrap();
}

#[derive(Debug, Clone)]
pub enum PatternType {
    Static(String),
    Dynamic(Regex),
}

impl PatternType {
    // Checks that the given path matches against the regex or static path
    pub fn is_match(&self, path: &str) -> bool {
        match self {
            PatternType::Static(string) => string == path,
            PatternType::Dynamic(regex) => regex.is_match(path),
        }
    }
}

impl FromStr for PatternType {
    type Err = AlcazarError;

    fn from_str(path: &str) -> Result<PatternType> {
        // URL with dynamic parts must be wrapped in curly braces
        if path.contains('{') && path.contains('}') {
            let mut pattern = String::new();

            for part in DYN_PARAM_REGEX.split(path) {
                match VALID_DYN_PARAM_REGEX.captures(part) {
                    // Construct a regular expression for the dynamic part
                    Some(capture) => {
                        let regex_part =
                            format!("(?P<{}>{})", &capture["part"], &ANY_VALUE_REGEX.as_str());
                        pattern.push_str(&regex_part);
                    }
                    // Use static parts as-is
                    _ => {
                        if part.contains('{') && part.contains('}') {
                            return Err(AlcazarError::RoutingError(
                                RoutingError::InvalidPathError {
                                    part: part.to_string(),
                                    path: path.to_string(),
                                },
                            ));
                        }

                        let regex_part = escape(&part.to_string());
                        pattern.push_str(&regex_part);
                    }
                };
            }

            // Compile the whole regular expression that matches to the path
            let regex_pattern = match Regex::new(&pattern) {
                Ok(regex) => regex,
                Err(err) => {
                    return Err(AlcazarError::RoutingError(RegexCompileError(
                        err.to_string(),
                    )))
                }
            };
            return Ok(PatternType::Dynamic(regex_pattern));
        }

        Ok(PatternType::Static(path.to_string()))
    }
}
