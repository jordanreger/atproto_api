# atproto_api
A simple [ATProto](https://atproto.com) implementation in Rust

## `BskyAgent`
`BskyAgent` is the main agent struct, particularly meant for Bluesky operations. Currently only `.login` is implemented.

`BskyAgent` provides a default struct, `BskyAgent { service: "https://bsky.social" }` that can be accessed by initializing the agent like so:

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent: atproto_api::BskyAgent = Default::default();

  Ok(())
}

```

## `AtpAgent`
`AtpAgent` is meant for general AT Protocol operations.

`AtpAgent` does *not* provide a default struct, as it's expected that you're planning to do anything but the default (i.e. using `BskyAgent`).

It can be initialized the standard way:

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent = atproto_api::AtpAgent {
    service = "https://atp.deno.dev".to_string(),
  };

  Ok(())
}
```