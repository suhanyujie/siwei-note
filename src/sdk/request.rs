extern crate http;
extern crate reqwest;
extern crate tokio;
extern crate tokio_test;

use std::collections::HashMap;
use std::{fmt::format, time::Duration};

use http::header::{CONTENT_LENGTH, CONTENT_TYPE, HOST};
use http::HeaderMap;

pub fn request() {}

/// http post 请求
pub async fn post(url: &str, body: String, headers: HeaderMap) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1000))
        .build()
        .unwrap();

    let resp = client.post(url).body(body).headers(headers).send().await?;
    let body_str = resp.text().await?;
    Ok(body_str)
}

/// http get 请求
pub async fn get(url: &str, query_params: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1000))
        .build()
        .unwrap();
    let url = format!("{}?{}", url, query_params);
    let resp = client.get(&url).send().await?;
    let body = resp.text().await?;

    return Ok(body);
}

/// 用于测试 async 函数的宏。
#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let url = "https://www.baidu.com";
        let res = aw!(get(url, ""));
        match res {
            Ok(body) => {
                eprintln!("content is: {}", body);
            }
            _ => {}
        }

        assert!(true);
    }
}
