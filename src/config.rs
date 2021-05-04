use serde_derive::{Deserialize, Serialize};
use std::io::Error;
use std::{fs, io::Read};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct DevConfig {
    pub auth_key_info: AuthKeyInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthKeyInfo {
    tower_app_id: String,
    tower_secret: String,
}

/// 根据 env.toml 文件读取配置
impl DevConfig {
    /// 从配置文件 env.toml 中获取配置，存入变量中。
    pub fn get_config() -> Result<DevConfig, Error> {
        // 读取配置文件
        let mut fd = fs::File::open("env.toml")?;
        let mut config_str = String::new();
        fd.read_to_string(&mut config_str)?;
        let config_obj = toml::from_str(&config_str);
        if config_obj.is_err() {
            panic!("请检查配置文件。")
        }

        Ok(config_obj.unwrap())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        let config_obj = DevConfig::get_config();
        assert!(config_obj.is_ok());
        assert!(config_obj.unwrap().auth_key_info.tower_app_id.len() > 0);
    }
}
