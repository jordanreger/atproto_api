> [!NOTE]
> This code is mostly useable. A few things are missing, like `.post()` and some other minor methods, but the majority is complete. It is currently available on [crates.io](https://crates.io) under 

# atproto_api
![Crates.io (latest)](https://img.shields.io/crates/dv/atproto-api)
A simple [ATProto](https://atproto.com) implementation in Rust

## `AtpAgent`
`AtpAgent` is meant for general AT Protocol operations.

### Initialization
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
#[macro_use]
extern crate dotenv_codegen;
use serde_json::json
use atproto_api::{Agent, AtpAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent = AtpAgent::new("https://fjall.net".to_string());

  println!("{:?}", agent);

  Ok(())
}

// AtpAgent { service: "https://fjall.net/", session: None }
```

### Get request
You can perform a get request by doing the following:
```rs
// macros
use dotenv_codegen::dotenv;
use serde_json::json;

use atproto_api::{Agent, AtpAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let agent = AtpAgent::default();

  let agent = agent
    .login(
      dotenv!("BLUESKY_IDENTIFIER").to_string(),
      dotenv!("BLUESKY_PASSWORD").to_string(),
    )
    .await?;

  let record = json!({
    "repo": "fjall.net",
    "collection": "app.bsky.feed.post",
    "rkey": "3k653jvvxlw2v"
  });

  let res = agent
    .get("com.atproto.repo.getRecord".to_string(), record)
    .await?;

  println!("{:?}", res);

  Ok(())
}
```

The first parameter is a [lexicon](https://atproto.com/guides/lexicon), the specific ones you can view by scrolling down to the bottom of the page and looking through the "Lexicons" section. The second parameter is a record, which is a JSON object that contains the other important information (in the case of `com.atproto.repo.getRecord`, that's `repo`, `collection`, and `rkey`).


## `BskyAgent`
> [!NOTE]
> `BskyAgent` is currently on the backburner, as it's a superset of `AtpAgent`. If for some reason you're using this library *now*, please use `AtpAgent` instead of `BskyAgent` for the time being.


## License
This code is licensed under the BSD 3-Clause license. You can view the license [here](https://git.sr.ht/~jordanreger/atproto_api/tree/main/item/LICENSE).