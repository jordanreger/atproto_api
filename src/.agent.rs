#[derive(Debug)]
#[allow(dead_code)]
pub struct AtpAgent {
    pub service: String,
    session: Session,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    did: String,
    email: String,
    handle: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
}

#[derive(serde::Serialize)]
pub struct Auth {
    identifier: String,
    password: String,
}

impl AtpAgent {
    pub async fn login(
        &mut self,
        identifier: String,
        password: String,
    ) -> Result<Session, Box<dyn std::error::Error>> {
        // check url is valid or panic
        let service = crate::tools::validate_url(&self.service);

        let client = reqwest::Client::builder().build()?;

        let auth = Auth {
            identifier: identifier.into(),
            password: password.into(),
        };

        let create_session = client
            .post(format!("{service}xrpc/com.atproto.server.createSession"))
            .header("User-Agent", "atproto_api/0.1.0")
            .json(&auth)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // check if error and panic otherwise return session json
        let res = match &create_session.get("error") {
            Some(x) if !x.is_null() => {
                let error = &create_session
                    .get("error")
                    .unwrap()
                    .to_string()
                    .replace("\"", "");
                let message = &create_session
                    .get("message")
                    .unwrap()
                    .to_string()
                    .replace("\"", "");
                panic!(
                    "{}",
                    format!("{service} error {error}: {message}").to_string()
                )
            }
            _ => create_session.to_string(),
        };

        let session: Session = serde_json::from_str(res.as_str()).unwrap();

        Ok(session)
    }

    /*pub async fn post(
        &self
    ) -> Result<String, Box<dyn std::error::Error>> {
        let service = crate::tools::validate_url
    }*/
}
