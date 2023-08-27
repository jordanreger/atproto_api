#[derive(Debug)]
pub struct BskyAgent {
    pub service: String,
}

#[derive(serde::Serialize)]
struct Auth {
    identifier: String,
    password: String,
}

impl Default for BskyAgent {
    fn default() -> BskyAgent {
        BskyAgent {
            service: "https://bsky.social".to_string(),
        }
    }
}

impl BskyAgent {
    pub async fn login(
        &self,
        identifier: String,
        password: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // check url is valid or panic
        let service = match url::Url::parse(&self.service) {
            Ok(service) => service.to_string(),
            Err(err) => panic!("{}", err),
        };

        let client = reqwest::Client::builder().build()?;

        let auth = Auth {
            identifier: identifier.into(),
            password: password.into(),
        };

        // create session
        let session = client
            .post(format!("{service}xrpc/com.atproto.server.createSession"))
            .header("User-Agent", "atproto_api/0.1.0")
            .json(&auth)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // check if error and panic otherwise return session json
        let res = match &session.get("error") {
            Some(x) if !x.is_null() => {
                let error = &session.get("error").unwrap().to_string().replace("\"", "");
                let message = &session
                    .get("message")
                    .unwrap()
                    .to_string()
                    .replace("\"", "");
                panic!(
                    "{}",
                    format!("{service} error {error}: {message}").to_string()
                )
            }
            _ => session.to_string(),
        };

        Ok(res)
    }
}
