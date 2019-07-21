use serenity::{model::channel::Message, model::gateway::Ready, prelude::*};
use std::collections::HashMap;

use crate::lib::command::{ArgSplit, Command};
use crate::lib::group::CommandGroup;

pub struct NanoHandler<'nh> {
  global_prefix: String,
  guild_prefixes: HashMap<u64, String>,
  commands: HashMap<String, &'nh (Command + Sync + Send)>,
  groups: Vec<CommandGroup<'nh>>,
  owner: Option<u64>,
}

impl EventHandler for NanoHandler<'_> {
  /* CUSTOM ACTIONS */
  fn ready(&self, _: Context, ready: Ready) {
    println!("{}  connected to Discord!", ready.user.name);
  }

  /* COMMANDS */
  fn message(&self, ctx: Context, msg: Message) {
    if let Err(resp) = self.handle_command(&ctx, &msg) {
      let _ = msg.channel_id.say(&ctx.http, resp);
    }
  }
}

impl NanoHandler<'_> {
  pub fn new(prefix: &str) -> NanoHandler {
    NanoHandler {
      global_prefix: String::from(prefix),
      guild_prefixes: HashMap::new(),
      commands: HashMap::new(),
      groups: vec![],
      owner: None,
    }
  }

  pub fn set_owner(&mut self, owner: u64) {
    self.owner = Some(owner);
  }

  fn handle_command(&self, ctx: &Context, msg: &Message) -> Result<(), String> {
    if !msg.author.bot {
      if let Some((cmd_name, cmd_args)) = self.parse_command(&msg) {
        if let Some(cmd) = self.commands.get(&cmd_name) {
          let cmd_args = match cmd.arg_split_count() {
            ArgSplit::None => vec![cmd_args],
            ArgSplit::Some(n) => cmd_args
              .splitn(n, " ")
              .map(std::borrow::ToOwned::to_owned)
              .collect::<Vec<String>>(),
            ArgSplit::All => cmd_args
              .split_whitespace()
              .map(std::borrow::ToOwned::to_owned)
              .collect(),
          };

          if cmd.owner_only() {
            if let Some(owner) = self.owner {
              if owner != *msg.author.id.as_u64() {
                return Err(format!(
                  "Only this bot's owner can run command `{}`",
                  cmd_name
                ));
              }
            } else {
              return Err(format!("Only this bot's owner can run command `{}`, but there seems to be no owner set for this bot.", cmd_name));
            }
          }

          if let Some(min) = cmd.min_args() {
            if cmd_args.len() < min {
              return Err(format!(
                "Command `{}` takes at least {} arguments, but was given {}.",
                cmd_name,
                min,
                cmd_args.len()
              ));
            }
          }
          if let Some(max) = cmd.max_args() {
            if cmd_args.len() > max {
              return Err(format!(
                "Command `{}` takes at most {} arguments, but was given {}.",
                cmd_name,
                max,
                cmd_args.len()
              ));
            }
          }

          cmd.run(ctx, msg, cmd_args);
        }
      }
    }
    Ok(())
  }

  fn parse_command(&self, msg: &Message) -> Option<(String, String)> {
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

      if cmd_contents.len() > 0 {
        if let Some(separator_index) = cmd_contents.chars().position(|c| c == ' ') {
          let mut cmd_name = cmd_contents
            .chars()
            .take(separator_index)
            .collect::<String>();
          let cmd_args = cmd_contents
            .chars()
            .skip(separator_index + 1)
            .collect::<String>();
          cmd_name = cmd_name.to_lowercase();
          return Some((cmd_name, cmd_args));
        } else {
          let cmd_name = cmd_contents.to_lowercase();
          return Some((cmd_name, String::from("")));
        }
      }
    }

    None
  }

  // TODO
  pub fn add_group(&mut self, group: CommandGroup) {
    // self.groups.push(group);

    // if let Some(group) = self.groups.last() {
    //   for cmd in group.commands {
    //     self.commands.insert(cmd.name(), cmd);
    //   }
    // }
  }
}
