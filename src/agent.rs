pub mod atp_agent {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Session {
        #[serde(rename = "accessJwt")]
        access_jwt: String,
        did: String,
        //email: String,
        handle: String,
        #[serde(rename = "refreshJwt")]
        refresh_jwt: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct RefreshSession {
        #[serde(rename = "accessJwt")]
        access_jwt: String,
        #[serde(rename = "refreshJwt")]
        refresh_jwt: String,
        did: String,
        handle: String,
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
        //fn default() -> AtpAgent;
        fn new(url: String) -> AtpAgent;
        async fn login(
            mut self,
            identifier: String,
            password: String,
        ) -> Result<AtpAgent, Box<dyn std::error::Error>>;
        async fn refresh_session(mut self) -> Result<(), Box<dyn std::error::Error>>;
        async fn get(
            mut self,
            lexicon: String,
            parameters: serde_json::Value,
        ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    }

    #[async_trait::async_trait]
    impl Agent for AtpAgent {
        // WARNING: default() is deprecated. You can
        // read more about this decision here:
        // https://todo.sr.ht/~jordanreger/atproto_api/7

        // AtpAgent::default()
        // fn default() -> AtpAgent {
        //     AtpAgent {
        //         service: "https://bsky.social/".to_string(),
        //         session: None,
        //     }
        // }

        // AtpAgent::new("https://fjall.net".to_string())
        fn new(url: String) -> AtpAgent {
            AtpAgent {
                service: crate::tools::validate_url(&url),
                session: None,
            }
        }

        // com.atproto.server.createSession
        async fn login(
            mut self,
            identifier: String,
            password: String,
        ) -> Result<AtpAgent, Box<dyn std::error::Error>> {
            let service = &self.service;

            let client = reqwest::Client::builder().build()?;

            let auth = Auth {
                identifier: identifier.into(),
                password: password.into(),
            };

            let session = client
                .post(format!("{service}xrpc/com.atproto.server.createSession"))
                .header("User-Agent", "atproto_api/0.1.0")
                .json(&auth)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

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

        // com.atproto.server.refreshSession
        async fn refresh_session(mut self) -> Result<(), Box<dyn std::error::Error>> {
            let session = self.session.unwrap();
            let refresh = RefreshSession {
                access_jwt: session.access_jwt,
                refresh_jwt: session.refresh_jwt,
                did: session.did,
                handle: session.handle,
            };
            let jwt = &refresh.refresh_jwt;
            let service = &self.service;
            let client = reqwest::Client::builder().build()?;
            let session = client
                .post(format!("{service}xrpc/com.atproto.server.refreshSession"))
                .header("User-Agent", "atproto_api/0.1.0")
                .header("Authorization", format!("Bearer {jwt}"))
                .query(&refresh)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            let res = match &session.get("error") {
                Some(x) if !x.is_null() => {
                    let error = &session.get("error").unwrap().to_string().replace("\"", "");
                    let message = &session
                        .get("message")
                        .unwrap()
                        .to_string()
                        .replace("\"", "");
                    panic!("{}", format!("{service} returns error {error}: {message}"))
                }
                _ => session,
            };

            let refreshed_session: Session = serde_json::from_value(res).unwrap();

            self.session = Some(refreshed_session);
            Ok(())
        }

        async fn get(
            mut self,
            lexicon: String,
            parameters: serde_json::Value,
        ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
            let service = &self.service;
            let session = &self.session.as_mut().unwrap();
            let jwt = &session.access_jwt;
            let client = reqwest::Client::builder().build()?;
            let res = client
                .get(format!("{service}xrpc/{lexicon}"))
                .header("User-Agent", "atproto_api/0.1.0")
                .header("Authorization", format!("Bearer {jwt}"))
                .query(&parameters)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            let res = match &res.get("error") {
                Some(x) if !x.is_null() => {
                    let error = &res.get("error").unwrap().to_string().replace("\"", "");
                    let message = &res.get("message").unwrap().to_string().replace("\"", "");
                    panic!("{}", format!("{service} returns error {error}: {message}"))
                }
                _ => res,
            };

            let _refresh_session = self.refresh_session();

            Ok(res)
        }
    }
}
