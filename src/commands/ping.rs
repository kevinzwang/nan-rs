use crate::lib::command::Command;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct PingCommand;

impl Command for PingCommand {
  fn name(&self) -> String {
    String::from("ping")
  }

  fn run(&self, ctx: &Context, msg: &Message, _: Vec<String>) -> Result<(), ()> {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");
    Ok(())
  }
}
