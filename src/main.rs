use serenity::prelude::*;
use std::fs;
use toml::Value;

mod commands;
mod lib;

use commands::ping::PingCommand;
use lib::group::CommandGroup;
use lib::handler::NanoHandler;

fn main() {
  let config_file = "config.toml";
  let contents = fs::read_to_string(config_file).expect("config file pls");
  let values = contents.parse::<Value>().expect("valid config file pls");
  let token = values["token"].as_str().expect("bot token pls");

  let mut handle = NanoHandler::new("!");

  if let Some(owner) = values["owner"].as_str() {
    let owner = owner.parse::<u64>().expect("valid owner id pls");
    handle.set_owner(owner);
  }

  let misc_group = CommandGroup {
    name: String::from("misc"),
    description: String::from("Miscellaneous"),
    commands: vec![&PingCommand],
  };

  handle.add_group(misc_group);

  let mut client = Client::new(&token, handle).expect("oof couldn't create client");

  if let Err(why) = client.start() {
    println!("oof client error: {:?}", why);
  }
}
