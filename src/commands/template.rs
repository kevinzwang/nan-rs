use serenity::{
  framework::standard::{macros::command, CommandResult, Args},
  model::prelude::*,
  prelude::*,
};

#[command]
#[description("template")]
#[aliases("templ", "t")]
#[usage("<foo>")]
#[example("template foo")]
#[owners_only]
pub fn template(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, "this is a template command")?;

  Ok(())
}
