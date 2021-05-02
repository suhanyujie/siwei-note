extern crate reqwest;
extern crate tokio;
extern crate tokio_test;

use reqwest::Error;

use super::const_var::*;
use std::{fmt::format, time::Duration};

struct AuthHandler {
    is_sandbox: bool,
    china: bool,
    consumer_secret: String,
    token_secret: String,
}

impl AuthHandler {
    pub fn new() -> AuthHandler {
        AuthHandler {
            is_sandbox: true,
            china: true,
            consumer_secret: String::new(),
            token_secret: String::new(),
        }
    }

    pub fn authorize() -> Result<String, Error> {
        Ok(String::new())
    }

    pub async fn get_credentials(&self) -> Result<String, Error> {
        let api_url = self.get_base_url("");
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(1000))
            .build()
            .unwrap();
        let body = client.get(api_url).send().await?.text().await?;
        Ok(body)
    }

    pub fn get_base_url(&self, prefix: &str) -> String {
        let mut url = "";
        if self.is_sandbox {
            url = BASE_URL_SANDBOX;
        } else if self.china == true {
            url = BASE_URL;
        }
        if prefix.is_empty() {
            url.to_string()
        } else {
            format!("{}{}{}", url, "/", prefix)
        }
    }
}

pub async fn get_tmp_credentials() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .unwrap();
    let url = format!("http://{}/s?wd=123", BASE_URL);

    let res = client.get(&url).send().await;

    let err = res.unwrap_err();
}

pub fn oauth_handler() {
    let is_china = 1;
    let is_sandbox = 1;
    let key = "";
    let secret = "";
    let callback = "";
}

macro_rules! aw1 {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[allow(non_snake_case)]
mod tests {
    use tokio_test::assert_ok;

    use super::*;

    #[test]
    fn test_get_credentials() {
        let h = AuthHandler::new();
        let res = aw1!(h.get_credentials());
        assert_ok!(res);
    }
}
