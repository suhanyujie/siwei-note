extern crate reqwest;
extern crate tokio;
extern crate tokio_test;

use reqwest::Error;

use super::const_var::*;
use std::{fmt::format, io::IntoInnerError, time::Duration};

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
        println!("{}", api_url);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(2000))
            .build()
            .unwrap();
        let body_param = AuthHandler::get_auth_param();
        let resp_body = client
            .post(api_url)
            .header(
                "Authorization",
                "Bearer e470e83254161fff2e166032c1cc9139e883b906d4c857092ecbc9f23c0cf82b",
            )
            .header("Contet-Type", "application/json")
            .body(body_param)
            .send()
            .await?
            .text()
            .await?;

        Ok(resp_body)
    }

    pub fn get_auth_param() -> &'static str {
        return r#"{
            "client_id": "a3d847c065114fefa86421638555f2969e0f9f5377b13b005f98b84f950f9961",
            "client_secret": "b0b287656b61c9ae297319e4139c4b891480f3d38f652a765f5b66a4af56c244",
            "code": "e470e83254161fff2e166032c1cc9139e883b906d4c857092ecbc9f23c0cf82b",
            "grant_type": "authorization_code",
            "redirect_uri": "urn:ietf:wg:oauth:2.0:oob"
        }"#;
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
        println!("{:?}", res);
        assert!(false);
    }
}
