use serde::Deserialize;
use serenity::{
  framework::standard::{macros::command, CommandResult},
  model::prelude::*,
  prelude::*,
};

#[derive(Deserialize)]
struct RandomCatResponse {
  file: String,
}

#[command]
#[description("Gives you a cute cat pic!")]
pub fn cat(ctx: &mut Context, msg: &Message) -> CommandResult {
  let url = match reqwest::get("http://aws.random.cat/meow") {
    Ok(mut r) => {
      let resp: RandomCatResponse = r.json()?;
      resp.file
    }
    Err(_) => reqwest::get("http://thecatapi.com/api/images/get")?
      .url()
      .to_string(),
  };

  let _ = msg.channel_id.say(&ctx.http, url)?;

  Ok(())
}
