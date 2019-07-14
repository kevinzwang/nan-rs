use serenity::{
  framework::standard::{macros::command, CommandResult},
  model::prelude::*,
  prelude::*,
};

#[command]
#[description("Pong!")]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
  let mut resp = msg.channel_id.say(&ctx.http, "Pong!")?;

  let diff = resp.timestamp.signed_duration_since(msg.timestamp);
  let edited_resp = format!("{} ping: {}ms", resp.content, diff.num_milliseconds());
  let _ = resp.edit(&ctx, |m| m.content(edited_resp))?;

  Ok(())
}
