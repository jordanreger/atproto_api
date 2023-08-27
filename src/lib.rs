#[derive(Debug)]
pub struct BskyAgent {
    pub service: String,
}

#[derive(serde::Serialize)]
pub struct Auth {
    identifier: String,
    password: String,
}

impl BskyAgent {
    pub async fn login(
        &self,
        identifier: String,
        password: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build()?;

        let auth = Auth {
            identifier: identifier.into(),
            password: password.into(),
        };

        let session = client
            .post("https://bsky.social/xrpc/com.atproto.server.createSession")
            .header("User-Agent", "atproto_api/0.1.0")
            .json(&auth)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //let res: String;

        let res = match &session.get("error") {
            Some(x) if !x.is_null() => {
                let error = &session.get("error").unwrap().to_string().replace("\"", "");
                let message = &session
                    .get("message")
                    .unwrap()
                    .to_string()
                    .replace("\"", "");
                format!("{error}: {message}")
            }
            _ => session.to_string(),
        };

        Ok(res)
    }
}
