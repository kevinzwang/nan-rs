use serenity::{model::channel::Message, model::gateway::Ready, prelude::*};
use std::collections::HashMap;

pub struct NanoHandler {
  pub global_prefix: String,
  pub guild_prefixes: HashMap<u64, String>,
}

impl EventHandler for NanoHandler {
  /**
   * CUSTOM ACTIONS
   */
  fn ready(&self, _: Context, ready: Ready) {
    println!("{:?}  connected to Discord!", ready.user.name);
  }

  /**
   * COMMANDS
   */
  fn message(&self, ctx: Context, msg: Message) {}
}

impl NanoHandler {
  fn parse_command(&self, msg: Message) -> Option<(String, Vec<String>)> {
    let prefix = match msg.guild_id {
      Some(id) => match self.guild_prefixes.get(id.as_u64()) {
        Some(p) => p,
        None => &self.global_prefix,
      },
      None => "",
    };

    if msg.content.starts_with(prefix) {
      let cmd_contents = msg
        .content
        .chars()
        .skip(prefix.chars().count())
        .collect::<String>();
      let mut cmd_contents = cmd_contents.split_whitespace();
      let cmd_name = cmd_contents.next();
      if let Some(cmd_name) = cmd_name {
        let cmd_name = String::from(cmd_name);
        let cmd_args = cmd_contents
          .map(std::borrow::ToOwned::to_owned)
          .collect::<Vec<_>>();
        Some((cmd_name, cmd_args))
      } else {
        None
      }
    } else {
      None
    }
  }
}
