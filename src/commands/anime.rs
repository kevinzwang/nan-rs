use graphql_client::GraphQLQuery;
use serenity::{
  framework::standard::{macros::command, Args, CommandResult},
  model::prelude::*,
  prelude::*,
};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "static/graphql/anilist/schema.json",
  query_path = "static/graphql/anilist/SearchAnime.graphql"
)]
struct SearchAnime;

#[command]
#[description("Searches for an anime on AniList. You can also use this command `<like this>` if you're feeling lucky.")]
#[aliases("a")]
#[usage("<title>")]
#[example("anime fire forces")]
#[min_args(1)]
pub fn anime(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
  Ok(())
}

fn search_chooser(msg: &Message, args: &Args) -> Result<(Option<Message>, u32), ()> {
  let search_string = args.message();
  
  let resp_msg: Option<Message> = None;
  
}