#[macro_use]
extern crate dotenv_codegen;

use atproto_api::{Agent, AtpAgent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AtpAgent::new("https://fjall.net".to_string());

    println!("{:?}", agent);

    let agent = agent
        .login(
            dotenv!("BLUESKY_IDENTIFIER").to_string(),
            dotenv!("BLUESKY_PASSWORD").to_string(),
        )
        .await?;

    println!("{:?}", agent);

    Ok(())
}
