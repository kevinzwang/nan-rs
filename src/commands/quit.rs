use serenity::{
  framework::standard::{macros::command, CommandResult},
  model::prelude::*,
  prelude::*,
};
use std::{process, thread::sleep, time::Duration};

#[command]
#[description("Stops the bot")]
#[aliases("exit", "q", "stop")]
#[owners_only]
pub fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, "See you soon!")?;
  ctx.invisible();
  sleep(Duration::from_secs(5));
  process::exit(0);
}
