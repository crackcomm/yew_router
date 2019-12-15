#[cfg(test)]
mod tests {
    use yew_router::Switch;

    #[test]
    fn single_enum_variant() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant"]
            Variant,
        }
        let route = String::from("/variant");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant)
    }

    #[test]
    fn single_enum_variant_unnamed_without_corresponding_capture_group() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant"]
            Variant(String),
        }
        let route = String::from("/variant");
        assert!(
            Test::switch(route).is_none(),
            "there should not be a way to ever create this variant."
        );
        let route = String::from("/variant/some/stuff");
        assert!(
            Test::switch(route).is_none(),
            "there should not be a way to ever create this variant."
        );
    }

    #[test]
    fn single_enum_variant_named_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{item}"]
            Variant { item: String },
        }
        let route = String::from("/variant/thing");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant {
                item: "thing".to_string()
            }
        )
    }

    #[test]
    fn single_enum_variant_unnamed_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{item}"]
            Variant(String),
        }
        let route = String::from("/variant/thing");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("thing".to_string()))
    }

    #[test]
    fn single_enum_variant_multiple_unnamed_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{}/{}"] // For unnamed variants, the names don't matter at all
            Variant(String, String),
        }
        let route = String::from("/variant/thing/other");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant("thing".to_string(), "other".to_string())
        )
    }

    #[test]
    fn single_enum_variant_multiple_named_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{item1}/{item2}"]
            Variant { item1: String, item2: String },
        }
        let route = String::from("/variant/thing/other");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant {
                item1: "thing".to_string(),
                item2: "other".to_string()
            }
        )
    }

    #[test]
    fn single_enum_variant_named_capture_without_leading_separator() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant{item}"]
            Variant { item: String },
        }
        let route = String::from("/variantthing");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant {
                item: "thing".to_string()
            }
        )
    }

    #[test]
    fn single_enum_variant_named_capture_without_any_separator() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant{item}stuff"]
            Variant { item: String },
        }
        let route = String::from("/variantthingstuff");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant {
                item: "thing".to_string()
            }
        )
    }

    #[test]
    fn single_enum_variant_end() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant!"]
            Variant,
        }
        let route = String::from("/variant/");
        assert!(Test::switch(route).is_none());
    }

    #[test]
    fn multiple_enum_variant_end_precedence() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant!"]
            Variant1,
            #[to = "/variant/stuff"]
            Variant2,
        }
        let route = String::from("/variant/stuff");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant2,
            "The first variant should be passed over"
        )
    }

    #[test]
    fn multiple_enum_variant_eager_matching() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant"]
            Variant1,
            #[to = "/variant/stuff"]
            Variant2,
        }
        let route = String::from("/variant/stuff");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant1,
            "The first variant should match first"
        )
    }

    #[test]
    fn single_enum_variant_convert_usize() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{item}"]
            Variant(usize),
        }
        let route = String::from("/variant/42");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant(42))
    }

    #[test]
    fn single_enum_variant_convert_usize_rejects_negative() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{item}"]
            Variant(usize),
        }
        let route = String::from("/variant/-42");
        assert!(Test::switch(route).is_none());
    }

    #[test]
    fn single_enum_variant_convert_isize() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant/{item}"]
            Variant(isize),
        }
        let route = String::from("/variant/-42");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant(-42))
    }

    #[test]
    fn single_enum_variant_missing_cap_produces_option_none() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/variant"]
            Variant(Option<String>),
        }
        let route = String::from("/variant");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant(None))
    }

    // TODO allow missing is a little broken at the moment.
    //    #[test]
    //    fn single_enum_variant_missing_section_produces_none() {
    //    use yew_router::switch::AllowMissing;
    //        #[derive(Debug, Switch, PartialEq)]
    //        pub enum Test {
    //            #[to = "/variant/{cap}"]
    //            Variant(AllowMissing<String>),
    //        }
    //        let route = String::from("/variant/");
    //        let switched = Test::switch(route).expect("should produce item");
    //        assert_eq!(switched, Test::Variant(AllowMissing(None)))
    //    }

    #[test]
    fn leading_slash() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/"]
            Variant,
        }
        let route = String::from("/");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant)
    }

    #[test]
    fn leading_named_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "{cap}"]
            Variant(String),
        }
        let route = String::from("hello");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("hello".to_string()))
    }

    #[test]
    fn leading_unnamed_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "{}"]
            Variant(String),
        }
        let route = String::from("hello");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("hello".to_string()))
    }

    #[test]
    fn leading_number_capture() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "{2:cap}"]
            Variant(String),
        }
        let route = String::from("hello/there");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("hello/there".to_string()))
    }

    #[test]
    fn leading_number_capture_unnamed() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "{2}"]
            Variant(String),
        }
        let route = String::from("hello/there");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("hello/there".to_string()))
    }

    #[test]
    fn leading_many_capture_named() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "{*:cap}"]
            Variant(String),
        }
        let route = String::from("hello/there");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("hello/there".to_string()))
    }

    #[test]
    fn leading_many_capture_unnamed() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "{*}"]
            Variant(String),
        }
        let route = String::from("hello/there");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("hello/there".to_string()))
    }

    #[test]
    fn leading_query_named() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "?query={hello}"]
            Variant(String),
        }
        let route = String::from("?query=lorem");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("lorem".to_string()))
    }

    #[test]
    fn leading_query_unnamed() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "?query={}"]
            Variant(String),
        }
        let route = String::from("?query=lorem");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant("lorem".to_string()))
    }

    #[test]
    fn leading_fragment() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "#fragment"]
            Variant,
        }
        let route = String::from("#fragment");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant)
    }

    #[test]
    fn fragment_with_named_captures() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "#{cap}ipsum{cap}"]
            Variant(String, String),
        }
        let route = String::from("#loremipsumdolor");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant("lorem".to_string(), "dolor".to_string())
        )
    }

    #[test]
    fn fragment_with_unnamed_captures() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "#{}ipsum{}"]
            Variant(String, String),
        }
        let route = String::from("#loremipsumdolor");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(
            switched,
            Test::Variant("lorem".to_string(), "dolor".to_string())
        )
    }

    #[test]
    fn escape_exclaim() {
        #[derive(Debug, Switch, PartialEq, Clone)]
        pub enum Test {
            #[to = "/escape!!"]
            Variant,
        }
        let route = String::from("/escape!");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant)
    }

    #[test]
    fn escape_bracket() {
        #[derive(Debug, Switch, Clone, PartialEq)]
        pub enum Test {
            #[to = "/escape{{}}a"]
            Variant,
        }
        let route = String::from("/escape{}a");
        let switched = Test::switch(route).expect("should produce item");
        assert_eq!(switched, Test::Variant)
    }

    mod fragment_routing_tests {
        use super::*;

        #[test]
        fn basic_fragment() {
            #[derive(Debug, Switch, Clone, PartialEq)]
            pub enum Test {
                #[to = "#/lorem"]
                Variant,
            }
            let route = String::from("#/lorem");
            Test::switch(route).expect("should produce item");
        }

        #[test]
        fn query_within_fragment() {
            #[derive(Debug, Switch, Clone, PartialEq)]
            pub enum Test {
                #[to = "#/lorem=ipsum"]
                Variant,
            }
            let route = String::from("#/lorem=ipsum");
            Test::switch(route).expect("should produce item");
        }

        #[test]
        fn capture_query_within_fragment() {
            #[derive(Debug, Switch, Clone, PartialEq)]
            pub enum Test {
                #[to = "#/lorem={ipsum}"]
                Variant { ipsum: String },
            }
            let route = String::from("#/lorem=dolor");
            let switched = Test::switch(route).expect("should produce item");
            assert_eq!(
                switched,
                Test::Variant {
                    ipsum: "dolor".to_string()
                }
            )
        }
    }
}
