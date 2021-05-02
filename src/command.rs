#![feature(str_split_once)]
extern crate clap;

use clap::{App, Arg, SubCommand};

pub struct SiWeiApp<'a, 'b> {
    cli_app: App<'a, 'b>,
}

impl<'a, 'b> SiWeiApp<'a, 'b> {
    pub fn new(app_name: &str) -> SiWeiApp<'a, 'b> {
        SiWeiApp {
            cli_app: App::new(app_name),
        }
    }
}

/// todo
fn run() {}

/// todo
pub fn get_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("note")
        .version("0.1.0")
        .author("suhanyujie<suhanyujie@qq.com>")
        .about("Private note")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("note")
                .about("some operation for note")
                .arg(
                    Arg::with_name("create")
                        .takes_value(false)
                        .help("create one note post"),
                ),
        );
    // .get_matches();
    return app;
}

pub fn handle_app(app: App) -> Result<(), clap::Error> {
    let arg_matchs = app.get_matches();
    if let Some(sub_m) = arg_matchs.subcommand_matches("note") {
        if let Some(key_value_str) = sub_m.value_of("create") {
            let value = get_param(key_value_str);
            match value {
                Some(val) => {
                    eprintln!("this is create: {}", val);
                }
                val => {
                    eprintln!("no run {:?}", val);
                }
            }
        } else {
            return Err(clap::Error {
                message: "invalid param...".to_string(),
                kind: clap::ErrorKind::InvalidValue,
                info: Some(vec![]),
            });
        }
    } else {
    }
    Ok(())
}

/// 用 `=` 号将字符串分割成一个键值对，并返回其中的值。
pub fn get_param<'a>(param_str: &'a str) -> Option<&'a str> {
    let arr: Vec<&'a str> = param_str.splitn(2, '=').collect();
    if arr.len() == 2 {
        return Some(arr[1]);
    } else {
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_app_handle() {}

    #[test]
    fn test_get_param() {
        let s1 = "create=key=value";
        let res1 = get_param(s1);
        assert_eq!(Some("key=value"), res1);
    }
}
