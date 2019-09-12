//! Lib for matching route strings based on tokens generated from the yew_router_route_parser crate.

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

pub use yew_router_route_parser::{MatcherToken, CaptureVariant, FromMatches, FromMatchesError};

mod match_paths;
mod util;

use nom::IResult;
use std::collections::{HashMap, HashSet};
use yew_router_route_parser::{optimize_tokens, parser};
use yew::{Html, Component};
use nom::combinator::all_consuming;

/// Render function.
pub trait RenderFn<CTX: Component>: Fn(&Matches) -> Option<Html<CTX>> {}


impl <CTX, T> RenderFn<CTX> for T
    where
    T: Fn(&Matches) -> Option<Html<CTX>>,
    CTX: Component
{}

/// Matches contain keys corresponding to named capture sections,
/// and values containing the content captured by those sections.
pub type Matches<'a> = HashMap<&'a str, String>;

/// Attempts to match routes, transform the route to Component props and render that Component.
///
/// The CTX refers to the context of the parent rendering this (The Router).
#[derive(Debug, PartialEq, Clone)]
pub struct PathMatcher {
    /// Tokens used to determine how the matcher will match a route string.
    pub tokens: Vec<MatcherToken>,
    /// Settings
    pub settings: MatcherSettings
}

/// Settings used for the matcher (and optimization of the parsed tokens that make up the matcher).
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MatcherSettings {
    /// Disallow insertion of Optional `/` at the end of paths.
    pub strict: bool,
    /// A matcher must consume all of the input to succeed.
    pub complete: bool,
    /// All literal matches do not care about case.
    pub case_insensitive: bool
}
impl Default for MatcherSettings {
    fn default() -> Self {
        MatcherSettings {
            strict: false,
            complete: true,
            case_insensitive: false
        }
    }
}

impl PathMatcher {

    /// Attempt to create a PathMatcher from a "matcher string".
    pub fn try_from(i: &str) -> Result<Self, ()> // TODO: Error handling
        where
    {
        let tokens = parser::parse(i).map_err(|_| ())?;
        let settings = MatcherSettings::default();
        let pm = PathMatcher {
            tokens: optimize_tokens(tokens, !settings.strict),
            settings
        };
        Ok(pm)
    }

    // TODO see if more error handling can be done here.
    // TODO, should find some way to support '/' characters in fragment. In the transform function, it could keep track of the seen hash begin yet, and transform captures based on that.

    /// Match a route string.
    pub fn match_path<'a, 'b: 'a>(&'b self, i: &'a str) -> IResult<&'a str, Matches<'a>> {
        if self.settings.complete {
            all_consuming(match_paths::match_path(&self.tokens, &self.settings))(i)
        } else {
            match_paths::match_path(&self.tokens, &self.settings)(i)
        }
    }

    /// Gets a set of all names that will be captured.
    /// This is useful in determining if a given struct will be able to be populated by a given path matcher before being given a concrete path to match.
    pub fn capture_names(&self) -> HashSet<&str> {
        fn capture_names_impl(tokens: &[MatcherToken]) -> HashSet<&str> {
            tokens.iter().fold(HashSet::new(), |mut acc: HashSet<&str>, token| {
                match token {
                    MatcherToken::Optional(t) => {
                        let captures = capture_names_impl(&t);
                        acc.extend(captures)
                    } ,
                    MatcherToken::Match(_) => {}
                    MatcherToken::Capture(variant)  => {
                        match variant {
                            CaptureVariant::ManyNamed(name) | CaptureVariant::Named(name) | CaptureVariant::NumberedNamed {name, ..} => {acc.insert(&name);},
                            CaptureVariant::ManyUnnamed | CaptureVariant::Unnamed | CaptureVariant::NumberedUnnamed {..} => {}
                        }
                    },
                }
                acc
            })
        }
        capture_names_impl(&self.tokens)
    }
}




    #[cfg(test)]
mod tests {
    use super::*;
    use yew_router_route_parser::parser::RouteParserToken;



//    use std::sync::{Once, ONCE_INIT};
//    static INIT: Once = ONCE_INIT;
//    fn setup_logs() {
//        INIT.call_once(|| {
//            env_logger::init();
//        });
//    }

    impl From<Vec<RouteParserToken>> for PathMatcher {
        fn from(tokens: Vec<RouteParserToken>) -> Self {
            let settings = MatcherSettings::default();
            PathMatcher {
                tokens: optimize_tokens(tokens, !settings.strict),
                settings
            }
        }
    }

    #[test]
    fn basic_separator() {
        let tokens = vec![RouteParserToken::Separator];
        let path_matcher = PathMatcher::from(tokens);
        path_matcher.match_path("/").expect("should parse");
    }

    #[test]
    fn multiple_tokens() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Match("hello".to_string()), RouteParserToken::Separator];

        let path_matcher = PathMatcher::from(tokens);
        path_matcher.match_path("/hello/").expect("should parse");
    }


    #[test]
    fn simple_capture() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::Named("hello".to_string())), RouteParserToken::Separator];
        let path_matcher = PathMatcher::from(tokens);
        let (_, matches) = path_matcher.match_path("/general_kenobi/").expect("should parse");
        assert_eq!(matches["hello"], "general_kenobi".to_string())
    }


    #[test]
    fn simple_capture_with_no_trailing_separator() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::Named("hello".to_string()))];
        let path_matcher = PathMatcher::from(tokens);
        let (_, matches) = path_matcher.match_path("/general_kenobi").expect("should parse");
        assert_eq!(matches["hello"], "general_kenobi".to_string())
    }


    #[test]
    fn match_with_trailing_match_any() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Match("a".to_string()), RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::Unnamed)];
        let path_matcher = PathMatcher::from(tokens);
        let (_, _matches) = path_matcher.match_path("/a/").expect("should parse");
    }

    #[test]
    fn match_n() {
//        setup_logs();
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::NumberedUnnamed {sections: 3}), RouteParserToken::Separator, RouteParserToken::Match("a".to_string())];
        let path_matcher = PathMatcher::from(tokens);
        let (_, _matches) = path_matcher.match_path("/garbage1/garbage2/garbage3/a").expect("should parse");
    }

    #[test]
    fn match_n_no_overrun() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::NumberedUnnamed {sections: 3})];
        let path_matcher = PathMatcher::from(tokens);
        let (s, _matches) = path_matcher.match_path("/garbage1/garbage2/garbage3").expect("should parse");
        assert_eq!(s.len(), 0)
    }


    #[test]
    fn match_n_named() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::NumberedNamed {sections: 3, name: "captured".to_string() }), RouteParserToken::Separator, RouteParserToken::Match("a".to_string())];
        let path_matcher = PathMatcher::from(tokens);
        let (_, matches) = path_matcher.match_path("/garbage1/garbage2/garbage3/a").expect("should parse");
        assert_eq!(matches["captured"], "garbage1/garbage2/garbage3".to_string())
    }


    #[test]
    fn match_many() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::ManyUnnamed), RouteParserToken::Separator, RouteParserToken::Match("a".to_string())];
        let path_matcher = PathMatcher::from(tokens);
        let (_, _matches) = path_matcher.match_path("/garbage1/garbage2/garbage3/a").expect("should parse");
    }

    #[test]
    fn match_many_named() {
        let tokens = vec![RouteParserToken::Separator, RouteParserToken::Capture(CaptureVariant::ManyNamed("captured".to_string())), RouteParserToken::Separator, RouteParserToken::Match("a".to_string())];
        let path_matcher = PathMatcher::from(tokens);
        let (_, matches) = path_matcher.match_path("/garbage1/garbage2/garbage3/a").expect("should parse");
        assert_eq!(matches["captured"], "garbage1/garbage2/garbage3".to_string())
    }

}