# yew-router (minimal derive fork)

Minimal fork of routing library for the [Yew](https://github.com/yewstack/yew) frontend framework.

## Example

```rust
#[derive(Switch, Debug)]
pub enum AppRoute {
    #[to = "/profile/{id}"]
    Profile(u32),
    #[to = "/forum{*:rest}"]
    Forum(ForumRoute),
    #[to = "/"]
    Index,
}

#[derive(Switch, Debug)]
pub enum ForumRoute {
    #[to = "/{subforum}/{thread_slug}"]
    SubForumAndThread{subforum: String, thread_slug: String}
    #[to = "/{subforum}"]
    SubForum{subforum: String}
}

fn main() {
    let app_route = AppRoute::from_path("/forum/test/12");
    assert_eq!(
        app_route.unwrap(),
        AppRoute::Forum(ForumRoute::SubForumAndThread {
            subforum: "test".to_owned(),
            thread_slug: "12".to_owned(),
        })
    );
}
```

### How to Include

You can use the in-development version in your project by adding it to your dependencies like so:

```toml
[dependencies]
yew-router = { git = "https://github.com/crackcomm/yew_router", branch="master" }
yew = { git = "https://github.com/crackcomm/yew", branch = "master" }
```

## Minimum rustc
Currently, this library targets rustc 1.39.0, but development is done on the latest stable release.
This library aims to track Yew`s minimum supported rustc version.

## Contributions/Requests

If you have any questions, suggestions, or want to contribute, please open an Issue or PR and we will get back to you in a timely manner.
