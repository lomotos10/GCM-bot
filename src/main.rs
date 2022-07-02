use poise::serenity_prelude::{self as serenity, GuildId};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
    sync::Arc,
};
use tokio::sync::Mutex;

mod utils;
use utils::*;
mod maimai;
use maimai::*;

/// Print help message
#[poise::command(slash_command, prefix_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let help = "**GCM-bot: Chart info provider for GekiChuMai**

**Usage:**
Method 1. Slash commands (recommended usage)
Method 2. @GCM-bot `command-name` `command-arguments`

**Nicknames for songs are supported - try stuff out!**

**Example usage:**
/mai-info bbb
@GCM-bot mai-info 3 seconds until dawn

**WIP:** Chunithm and Ongeki support

If you have any bug reports or suggestions, please contact @Lomo#2363 for help,
or send an issue or PR to https://github.com/lomotos10/GCM-bot !";
    ctx.say(help).await?;
    Ok(())
}

/// Advice on how to improve
#[poise::command(slash_command, prefix_command, rename = "how-to-improve")]
async fn how_to_improve(ctx: Context<'_>) -> Result<(), Error> {
    let help = ":regional_indicator_p: :regional_indicator_l: :a: :regional_indicator_y:  :m: :o2: :regional_indicator_r: :regional_indicator_e:";
    ctx.say(help).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![
                mai_info(),
                mai_jacket(),
                help(),
                how_to_improve(),
                register(),
            ],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                let mai_charts = set_mai_charts()?;
                let mai_aliases = set_mai_aliases(&mai_charts)?;
                let cooldown_server_ids = {
                    let file = File::open("data/cooldown-server-ids.txt")?;
                    BufReader::new(file)
                        .lines()
                        .map(|l| l.unwrap().parse::<u64>())
                        .filter(|b| b.is_ok())
                        .map(|l| GuildId(l.unwrap()))
                        .collect::<HashSet<_>>()
                };
                let user_timestamp = Arc::new(Mutex::new(
                    cooldown_server_ids
                        .iter()
                        .map(|k| (*k, HashMap::new()))
                        .collect(),
                ));

                Ok(Data {
                    mai_charts,
                    mai_aliases,
                    mai_jacket_prefix: fs::read_to_string("data/maimai-jacket-prefix.txt")?,

                    cooldown_server_ids,
                    user_timestamp,
                })
            })
        });

    println!("Starting run:");
    framework.run().await.unwrap();
}
