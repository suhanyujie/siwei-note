extern crate reqwest;

pub fn request() {}

pub fn post() {}

pub async fn get() {
    let body = reqwest::get("https://www.rust-lang.org").await;
    println!("resp body: {:?}", body.unwrap_err());
    return;
}

mod tests {
    use super::*;

    #[test]
    fn test_get() {
        get();
        assert!(false);
    }
}
