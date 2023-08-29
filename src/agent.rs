pub mod atp_agent {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

    #[derive(Debug, Clone)]
    pub struct AtpAgent {
        pub service: String,
        pub session: Option<Session>,
    }

    #[async_trait::async_trait]
    pub trait Agent: Sized {
        fn default() -> AtpAgent;
        fn new(url: String) -> AtpAgent;
        async fn login(
            mut self,
            identifier: String,
            password: String,
        ) -> Result<AtpAgent, Box<dyn std::error::Error>>;
    }

    #[async_trait::async_trait]
    impl Agent for AtpAgent {
        fn default() -> AtpAgent {
            AtpAgent {
                service: "https://bsky.social/".to_string(),
                session: None,
            }
        }

        fn new(url: String) -> AtpAgent {
            AtpAgent {
                service: crate::tools::validate_url(&url),
                session: None,
            }
        }

        async fn login(
            mut self,
            identifier: String,
            password: String,
        ) -> Result<AtpAgent, Box<dyn std::error::Error>> {
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
                        format!("{service} returns error {error}: {message}").to_string()
                    )
                }
                _ => session,
            };

            let session: Session = serde_json::from_value(res).unwrap();

            self.session = Some(session);
            Ok(self)
        }
    }
}
