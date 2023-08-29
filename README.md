> [!WARNING]
> Do not use this yet, it's nowhere near done. It will be posted on [crates.io](https://crates.io) under the same name when it's complete.

# atproto_api
A simple [ATProto](https://atproto.com) implementation in Rust

> [!NOTE]
> `BskyAgent` is currently on the backburner, as it's a superset of `AtpAgent`. If for some reason you're using this library *now*, please use `AtpAgent` instead of `BskyAgent` for the time being.

<!-- ## `BskyAgent`
`BskyAgent` is the main agent struct, particularly meant for Bluesky operations. Currently only `.login` is implemented.

`BskyAgent` provides a default struct, `BskyAgent { service: "https://bsky.social" }` that can be accessed by initializing the agent like so:

```rs
use atproto_api::{Agent, AtpAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent = AtpAgent::default();

  Ok(())
}

``` -->

## `AtpAgent`
`AtpAgent` is meant for general AT Protocol operations.

`AtpAgent` does *not* provide a default struct, as it's expected that you're planning to do anything but the default (i.e. using `BskyAgent`).

It can be initialized in two ways:

1. [`default()`](https://git.sr.ht/~jordanreger/atproto_api/tree/main/item/examples/default.rs)
```rs
use atproto_api::{Agent, AtpAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent = AtpAgent::default();

  println!("{:?}", agent);

  Ok(())
}

// AtpAgent { service: "https://bsky.social/", session: None }
```

2. [`new()`](https://git.sr.ht/~jordanreger/atproto_api/tree/main/item/examples/new.rs)
```rs
use atproto_api::{Agent, AtpAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent = AtpAgent::new("https://fjall.net".to_string());

  println!("{:?}", agent);

  Ok(())
}
```