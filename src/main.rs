use serenity::{
    prelude::*,
    model::gateway::Ready,
    framework::standard::{
        StandardFramework, DispatchError,
        macros::{group}
    },
};
use std::{fs, collections::HashSet};
use toml::Value;

mod commands;
use commands::cat::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{:?} is connected!", ready.user.name);
    }
}

group!({
    name: "fun",
    commands: [cat]
});

fn main() {
    let config_file = "config.toml";
    let contents = fs::read_to_string(config_file)
        .expect("config file pls");
    let values = contents
        .parse::<Value>()
        .expect("parseable config file pls");
    let token = values["token"]
        .as_str()
        .expect("bot token pls");

    let mut client = Client::new(&token, Handler)
        .expect("oof couldn't create client");

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("oof couldn't access app info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("!")
            .owners(owners)
        )
        .on_dispatch_error(|ctx, msg, error| {
            if let DispatchError::Ratelimited(seconds) = error {
                let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds.", seconds));
            }
        })
        .group(&FUN_GROUP)
    );

    if let Err(why) = client.start() {
        println!("oof client error: {:?}", why);
    }
}
