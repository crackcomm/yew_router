//! Parser for a "matcher string". The tokens produced by this parser are used to construct a matcher.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

pub mod parser;
mod token_optimizer;

pub use parser::CaptureVariant;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
pub use token_optimizer::{
    next_delimiters, optimize_tokens, parse_str_and_optimize_tokens, MatcherToken,
};

/// An error type used when implementing `FromCaptures`.
#[derive(Debug)]
pub enum FromCapturesError {
    /// Missing field
    MissingField {
        /// The name of the field expected to be present
        field_name: String,
    },
    /// Dynamic error
    Error(Box<dyn Error>),
    /// Unknown error
    UnknownErr, // TODO Will be removed soon. dyn error above needs to go, and replaced with the names of the failed type conversions.
}

impl Display for FromCapturesError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FromCapturesError::MissingField { field_name } => {
                write! {f, "The field: '{}' was not present in your path matcher.", field_name}
            }
            FromCapturesError::Error(e) => e.fmt(f),
            FromCapturesError::UnknownErr => write!(f, "unknown error"),
        }
    }
}

impl Error for FromCapturesError {
    //    fn source(&self) -> Option<&(dyn Error + 'static)> {
    //        match self  {
    //            FromCapturesError::MissingField {..} => None,
    //            FromCapturesError::Error(e) => Some(&e )
    //        }
    //    }
}


/// Captures contain keys corresponding to named match sections,
/// and values containing the content captured by those sections.
pub type Captures<'a> = HashMap<&'a str, String>;

/// Used for constructing `Properties` from URL matches.
///
/// # Note
/// FromCaptures, as derived, is pretty dumb and unreliable.
/// It is only suggested to derive FromCaptures if the types in your struct are reliably convertible from `&str`.
/// In practice, this means that `String`, and the numeric types are safe bets.
///
/// The derive relies on [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) for converting types.
///
/// # Suggestions
/// * If you have one or more optional sections in your path matcher, you are best off implementing this yourself.
pub trait FromCaptures: Sized {
    /// Produces the props from the hashmap.
    /// It is expected that `TryFrom<String>` be implemented on all of the types contained within the props.
    fn from_captures(captures: &Captures) -> Result<Self, FromCapturesError>;
    /// Verifies that all of the field names produced by the PathMatcher exist on the target props.
    /// Should panic if not all match.
    /// Should only be used at compile time.
    fn verify(_field_names: &HashSet<String>) {}
}

impl FromCaptures for () {
    fn from_captures(_captures: &Captures) -> Result<Self, FromCapturesError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;

    #[allow(unused)]
    #[derive(Debug)]
    struct TestStruct {
        hello: String,
        there: String,
    }

    impl FromCaptures for TestStruct {
        fn from_captures(captures: &HashMap<&str, String>) -> Result<Self, FromCapturesError> {
            let hello = captures
                .get("hello")
                .ok_or_else(|| FromCapturesError::MissingField {
                    field_name: "hello".to_string(),
                })
                .and_then(|m: &String| {
                    String::try_from(m.clone()).map_err(|_| FromCapturesError::UnknownErr)
                })?;

            let there = captures
                .get("there")
                .ok_or_else(|| FromCapturesError::MissingField {
                    field_name: "there".to_string(),
                })
                .and_then(|m: &String| {
                    String::try_from(m.clone()).map_err(|_| FromCapturesError::UnknownErr)
                })?;

            let x = TestStruct { hello, there };
            Ok(x)
        }

        fn verify(field_names: &HashSet<String>) {
            if !field_names.contains(&"hello".to_string()) {
                panic!(
                    "The struct expected the matches to contain a field named '{}'",
                    "hello".to_string()
                )
            }
            if !field_names.contains(&"there".to_string()) {
                panic!(
                    "The struct expected the matches to contain a field named '{}'",
                    "there".to_string()
                )
            }
        }
    }

    #[test]
    fn underived_verify_impl_is_valid() {
        let mut hs = HashSet::new();
        hs.insert("hello".to_string());
        hs.insert("there".to_string());
        TestStruct::verify(&hs);
    }

    #[test]
    #[should_panic]
    fn underived_verify_impl_rejects_incomplete_matches_hello() {
        let mut hs = HashSet::new();
        hs.insert("hello".to_string());
        TestStruct::verify(&hs);
    }

    #[test]
    #[should_panic]
    fn underived_verify_impl_rejects_incomplete_matches_there() {
        let mut hs = HashSet::new();
        hs.insert("there".to_string());
        TestStruct::verify(&hs);
    }

    #[test]
    fn underived_matches_impl_is_valid() {
        let mut hm = HashMap::new();
        hm.insert("hello", "You are".to_string());
        hm.insert("there", "a".to_string());
        TestStruct::from_captures(&hm).expect("should generate struct");
    }

    #[test]
    fn underived_matches_rejects_incomplete_hello() {
        let mut hm = HashMap::new();
        hm.insert("hello", "You are".to_string());
        TestStruct::from_captures(&hm).expect_err("should not generate struct");
    }

    #[test]
    fn underived_matches_rejects_incomplete_there() {
        let mut hm = HashMap::new();
        hm.insert("there", "You are".to_string());
        TestStruct::from_captures(&hm).expect_err("should not generate struct");
    }

    #[test]
    fn error_display_missing_field() {
        let err = FromCapturesError::MissingField {
            field_name: "hello".to_string(),
        };
        let displayed = format!("{}", err);
        let expected = "The field: 'hello' was not present in your path matcher.";
        assert_eq!(displayed, expected);
    }
}