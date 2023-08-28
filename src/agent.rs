pub mod atp_agent {
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
    struct Auth {
        identifier: String,
        password: String,
    }

    #[derive(Debug)]
    pub struct Agent {
        pub service: String,
    }

    #[derive(Debug)]
    pub struct AtpAgent {
        pub service: String,
        pub session: Session,
    }

    impl Agent {
        pub async fn login(
            &self,
            identifier: String,
            password: String,
        ) -> Result<AtpAgent, Box<dyn std::error::Error>> {
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

            let authorized = AtpAgent {
                service: service.into(),
                session: session.into(),
            };

            Ok(authorized)
        }
    }
}
