pub fn validate_url(url: &String) -> String {
    let service = match url::Url::parse(url) {
        Ok(service) => service.to_string(),
        Err(err) => panic!("{}", err),
    };

    service
}
