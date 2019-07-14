use serenity::{
  prelude::*,
  model::prelude::*,
  framework::standard::{
    CommandResult,
    macros::command
  }
};
use serde::Deserialize;

#[derive(Deserialize)]
struct RandomCat {
  file: String
}

#[command]
pub fn cat(ctx: &mut Context, msg: &Message) -> CommandResult {
  let url = match reqwest::get("http://aws.radom.cat/meow") {
    Ok(mut r) => {
      let resp:RandomCat = r.json()?;
      resp.file
    },
    Err(_) => reqwest::get("http://thecatapi.com/api/images/get")?.url().to_string()
  };

  let _ = msg.channel_id.say(&ctx.http, url);

  Ok(())
}