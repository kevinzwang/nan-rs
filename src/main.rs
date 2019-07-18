use serenity::{
  framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
  },
  model::{channel::Message, gateway::Ready, id::UserId},
  prelude::*,
};
use std::{collections::HashSet, fs};
use toml::Value;

mod commands;
use commands::{anime::*, cat::*, nekoslife::*, ping::*, quit::*};

struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    println!("{:?} is connected!", ready.user.name);
  }
}

#[help]
fn default_help(
  context: &mut Context,
  msg: &Message,
  args: Args,
  help_options: &'static HelpOptions,
  groups: &[&'static CommandGroup],
  owners: HashSet<UserId>,
) -> CommandResult {
  help_commands::plain(context, msg, args, help_options, groups, owners)
}

group!({
  name: "fun",
  commands: [cat]
});

group!({
  name: "weeb",
  commands: [catgirl, foxgirl, anime]
});

group!({
  name: "util",
  commands: [ping, quit]
});

fn dispatch_error_handler<'a, 'b>(ctx: &'a mut Context, msg: &'b Message, error: DispatchError) {
  if let DispatchError::Ratelimited(seconds) = error {
    let _ = msg.channel_id.say(
      &ctx.http,
      &format!("Try this again in {} seconds.", seconds),
    );
  } else if let DispatchError::NotEnoughArguments { min, given } = error {
    let _ = msg.channel_id.say(
      &ctx.http,
      &format!(
        "You provided {} argument(s) but the command expected {}!",
        given, min
      ),
    );
  }
}

fn main() {
  let config_file = "config.toml";
  let contents = fs::read_to_string(config_file).expect("config file pls");
  let values = contents
    .parse::<Value>()
    .expect("parseable config file pls");
  let token = values["token"].as_str().expect("bot token pls");

  let mut client = Client::new(&token, Handler).expect("oof couldn't create client");

  let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
    Ok(info) => {
      let mut owners = HashSet::new();
      owners.insert(info.owner.id);

      (owners, info.id)
    }
    Err(why) => panic!("oof couldn't access app info: {:?}", why),
  };

  client.with_framework(
    StandardFramework::new()
      .configure(|c| {
        c.with_whitespace(true)
          .on_mention(Some(bot_id))
          .prefix("!")
          .owners(owners)
      })
      .on_dispatch_error(dispatch_error_handler)
      .help(&DEFAULT_HELP)
      .group(&FUN_GROUP)
      .group(&WEEB_GROUP)
      .group(&UTIL_GROUP),
  );

  if let Err(why) = client.start() {
    println!("oof client error: {:?}", why);
  }
}
