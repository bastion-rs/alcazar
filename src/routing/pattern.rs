use crate::error::RoutingError::RegexCompileError;
use crate::error::{AlcazarError, Result, RoutingError};
use lazy_static::lazy_static;
use regex::{escape, CaptureMatches, Captures, Regex};
use std::str::FromStr;

lazy_static! {
    static ref ANY_VALUE_REGEX: Regex = Regex::new(r"[^{}/]+").unwrap();
    static ref DYN_PARAM_REGEX: Regex = Regex::new(r"(\{\s*[\w\d_]+\s*\})").unwrap();
    static ref VALID_DYN_PARAM_REGEX: Regex = Regex::new(r"\{(?P<part>[\w][\w\d_]*)\}").unwrap();
}

#[derive(Debug, Clone)]
pub enum PatternType {
    Static(String),
    Dynamic(Regex),
}

impl PatternType {
    // Checks that the given path matches against the regex or static path
    pub(crate) fn is_match(&self, path: &str) -> bool {
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

            for state in SplitCaptures::new(&DYN_PARAM_REGEX, path) {
                let raw_part = match state {
                    SplitState::Unmatched(unmatched_part) => unmatched_part.to_string(),
                    SplitState::Captured(capture) => capture[0].to_string(),
                };

                match VALID_DYN_PARAM_REGEX.captures(&raw_part) {
                    // Construct a regular expression for the dynamic part
                    Some(capture) => {
                        let regex_part =
                            format!("(?P<{}>{})", &capture["part"], &ANY_VALUE_REGEX.as_str());
                        pattern.push_str(&regex_part);
                    }
                    // Use static parts as-is
                    _ => {
                        if raw_part.contains('{') || raw_part.contains('}') {
                            return Err(AlcazarError::RoutingError(
                                RoutingError::InvalidPathError {
                                    part: raw_part,
                                    path: path.to_string(),
                                },
                            ));
                        }

                        let regex_part = escape(&raw_part);
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

// Special wrapper around regex splits that handles matches / unmatched parts together.
// For more info check the GitHub issue: https://github.com/rust-lang/regex/issues/330
//
// TODO: Replace this code when the functionality will be added to the crate
// GitHub issue: https://github.com/rust-lang/regex/issues/681
struct SplitCaptures<'r, 't> {
    finder: CaptureMatches<'r, 't>,
    text: &'t str,
    last: usize,
    caps: Option<Captures<'t>>,
}

impl<'r, 't> SplitCaptures<'r, 't> {
    fn new(re: &'r Regex, text: &'t str) -> SplitCaptures<'r, 't> {
        SplitCaptures {
            finder: re.captures_iter(text),
            text,
            last: 0,
            caps: None,
        }
    }
}

#[derive(Debug)]
enum SplitState<'t> {
    Unmatched(&'t str),
    Captured(Captures<'t>),
}

impl<'r, 't> Iterator for SplitCaptures<'r, 't> {
    type Item = SplitState<'t>;

    fn next(&mut self) -> Option<SplitState<'t>> {
        if let Some(caps) = self.caps.take() {
            return Some(SplitState::Captured(caps));
        }
        match self.finder.next() {
            None => {
                if self.last >= self.text.len() {
                    None
                } else {
                    let s = &self.text[self.last..];
                    self.last = self.text.len();
                    Some(SplitState::Unmatched(s))
                }
            }
            Some(caps) => {
                let m = caps.get(0).unwrap();
                let unmatched = &self.text[self.last..m.start()];
                self.last = m.end();
                self.caps = Some(caps);
                Some(SplitState::Unmatched(unmatched))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::routing::pattern::PatternType;
    use std::str::FromStr;

    #[test]
    fn test_parse_static_path() {
        let path = "/static/files/icons/";

        let result = PatternType::from_str(path);
        assert_eq!(result.is_ok(), true);

        let pattern_type = result.unwrap();
        assert_eq!(pattern_type.is_match(path), true);
    }

    #[test]
    fn test_parse_dynamic_path() {
        let path = "/api/v1/users/{id}/detail/";

        let result = PatternType::from_str(path);
        assert_eq!(result.is_ok(), true);

        let pattern_type = result.unwrap();
        let url_example = "/api/v1/users/1000/detail/";
        assert_eq!(pattern_type.is_match(url_example), true);
    }

    #[test]
    fn test_parse_path_with_multiple_parameters() {
        let path = "/api/v1/blog/{blog_id}/users/{user_id}";

        let result = PatternType::from_str(path);
        assert_eq!(result.is_ok(), true);

        let pattern_type = result.unwrap();
        let url_example = "/api/v1/blog/1/users/100";
        assert_eq!(pattern_type.is_match(url_example), true);
    }

    #[test]
    fn test_get_invalid_path_error_for_incorrect_dynamic_path() {
        let path = "/api/v1/blog/{blog_id}}/";

        let result = PatternType::from_str(path);
        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.unwrap_err().to_string(),
            "found an invalid \"}/\" part of the \"/api/v1/blog/{blog_id}}/\" path."
        );
    }
}
