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
