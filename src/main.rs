use serenity::prelude::*;
use std::fs;
use toml::Value;
use std::collections::HashMap;

mod handler;

fn main() {
  let config_file = "config.toml";
  let contents = fs::read_to_string(config_file).expect("config file pls");
  let values = contents
    .parse::<Value>()
    .expect("parseable config file pls");
  let token = values["token"].as_str().expect("bot token pls");

  let hndlr = handler::NanoHandler{
    global_prefix: String::from("!"),
    guild_prefixes: HashMap::new(),
  };

  let mut client = Client::new(&token, hndlr).expect("oof couldn't create client");

  if let Err(why) = client.start() {
    println!("oof client error: {:?}", why);
  }
}
