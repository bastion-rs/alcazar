use crate::error::{AlcazarError, Result};
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
                    Some(capture) => {
                        let regex_part =
                            format!("(?P<{}>{})", &capture["part"], &ANY_VALUE_REGEX.as_str());
                        pattern.push_str(&regex_part);
                    }
                    _ => {
                        let regex_part = escape(&part.to_string());
                        pattern.push_str(&regex_part);
                    }
                };
            }

            // TODO: Add error handling
            let regex_pattern = Regex::new(&pattern).unwrap();
            return Ok(PatternType::Dynamic(regex_pattern));
        }

        Ok(PatternType::Static(path.to_string()))
    }
}
