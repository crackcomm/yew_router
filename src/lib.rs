#![recursion_limit = "128"]
//! Provides routing faculties for the Yew web framework.
//!
//! ## Contents
//! This crate consists of multiple types, some independently useful on their own,
//! that are used together to facilitate routing within the Yew framework.
//! Among them are:
//! * Switch - A trait/derive macro that allows specification of how enums or structs can be constructed
//! from Routes.

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
// This will break the project at some point, but it will break yew as well.
// It can be dealt with at the same time.
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

pub mod route;

pub use yew_router_route_parser;

/// Prelude module that can be imported when working with the yew_router
pub mod prelude {
    pub use super::matcher::Captures;
    pub use crate::route::Switch;
    pub use yew_router_macro::Switch;
}

pub mod matcher;

pub use matcher::Captures;

pub use route::Switch;
pub use yew_router_macro::Switch;
