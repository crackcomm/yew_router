//! Parses routes into enums or structs.

/// Derivable routing trait that allows instances of implementors to be constructed from Routes.
///
/// # Note
/// Don't try to implement this yourself, rely on the derive macro.
///
/// # Example
/// ```
/// use yew_router::Switch;
/// #[derive(Debug, Switch, PartialEq)]
/// enum TestEnum {
///     #[to = "/test/route"]
///     TestRoute,
///     #[to = "/capture/string/{path}"]
///     CaptureString { path: String },
///     #[to = "/capture/number/{num}"]
///     CaptureNumber { num: usize },
///     #[to = "/capture/unnamed/{doot}"]
///     CaptureUnnamed(String),
/// }
///
/// assert_eq!(
///     TestEnum::from_path("/test/route"),
///     Some(TestEnum::TestRoute)
/// );
/// assert_eq!(
///     TestEnum::from_path("/capture/string/lorem"),
///     Some(TestEnum::CaptureString {
///         path: "lorem".to_string()
///     })
/// );
/// assert_eq!(
///     TestEnum::from_path("/capture/number/22"),
///     Some(TestEnum::CaptureNumber { num: 22 })
/// );
/// assert_eq!(
///     TestEnum::from_path("/capture/unnamed/lorem"),
///     Some(TestEnum::CaptureUnnamed("lorem".to_string()))
/// );
/// ```
pub trait Switch: Sized {
    /// Based on a route, possibly produce an itself.
    fn from_path(path: &str) -> Option<Self>;

    /// Parses route.
    fn from_route(part: String) -> Option<Self> {
        Self::from_path(&part)
    }
}

/// Wrapper that requires that an implementor of Switch must start with a `/`.
///
/// This is needed for any non-derived type provided by yew-router to be used by itself.
///
/// This is because route strings will almost always start with `/`, so in order to get a std type
/// with the `rest` attribute, without a specified leading `/`, this wrapper is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeadingSlash<T>(pub T);

impl<U: Switch> Switch for LeadingSlash<U> {
    fn from_path(part: &str) -> Option<Self> {
        if part.starts_with('/') {
            U::from_path(&part[1..]).map(LeadingSlash)
        } else {
            None
        }
    }
}

impl<U: Switch> Switch for Option<U> {
    /// Option is very permissive in what is allowed.
    fn from_path(part: &str) -> Option<Self> {
        Some(U::from_path(part))
    }
}

/// Allows a section to match, providing a None value,
/// if its contents are entirely missing, or starts with a '/'.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AllowMissing<T: std::fmt::Debug>(pub Option<T>);

impl<U: Switch + std::fmt::Debug> Switch for AllowMissing<U> {
    fn from_path(part: &str) -> Option<Self> {
        let inner = U::from_path(&part);

        if inner.is_some() {
            Some(AllowMissing(inner))
        } else if part == ""
            || part.starts_with('/')
            || part.starts_with('?')
            || part.starts_with('&')
            || part.starts_with('#')
        {
            Some(AllowMissing(None))
        } else {
            None
        }
    }
}

macro_rules! impl_switch_for_from_to_str {
    ($($SelfT: ty),*) => {
        $(
        impl Switch for $SelfT {
            fn from_path(part: &str) -> Option<Self> {
                ::std::str::FromStr::from_str(&part).ok()
            }
        }
        )*
    };
}

impl_switch_for_from_to_str! {
    String,
    bool,
    f64,
    f32,
    usize,
    u128,
    u64,
    u32,
    u16,
    u8,
    isize,
    i128,
    i64,
    i32,
    i16,
    i8,
    std::num::NonZeroU128,
    std::num::NonZeroU64,
    std::num::NonZeroU32,
    std::num::NonZeroU16,
    std::num::NonZeroU8,
    std::num::NonZeroI128,
    std::num::NonZeroI64,
    std::num::NonZeroI32,
    std::num::NonZeroI16,
    std::num::NonZeroI8
}
