use serde::Deserialize;
use serenity::{
  framework::standard::{macros::command, CommandResult},
  model::prelude::*,
  prelude::*,
};
use std::error::Error;

#[derive(Deserialize)]
struct NekosLifeResponse {
  url: String,
}

fn fetch_nekoslife(cat: &str) -> Result<String, Box<Error>> {
  let url = format!("https://nekos.life/api/v2/img/{}", cat);
  let resp: NekosLifeResponse = reqwest::get(url.as_str())?.json()?;
  Ok(resp.url)
}

#[command]
#[aliases("neko")]
#[description("Gives you a catgirl image!")]
pub fn catgirl(ctx: &mut Context, msg: &Message) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, fetch_nekoslife("neko")?);
  Ok(())
}

#[command]
#[description("Gives you a foxgirl image!")]
pub fn foxgirl(ctx: &mut Context, msg: &Message) -> CommandResult {
  let _ = msg.channel_id.say(&ctx.http, fetch_nekoslife("fox_girl")?);
  Ok(())
}
