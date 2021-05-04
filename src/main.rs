mod command;
mod config;
mod sdk;
use clap::Error;

use crate::sdk::enml::plain_text_to_enml_converter;
use crate::sdk::request;

fn main() -> Result<(), Error> {
    let app = command::get_app();
    command::handle_app(app)?;
    Ok(())
}
