use poise::serenity_prelude::{self as serenity, ChannelId, GuildId};
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
mod chuni;
use chuni::*;
mod ongeki;
use ongeki::*;

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

For detailed info or bug reports or suggestions,
please refer to <https://github.com/lomotos10/GCM-bot>";
    ctx.say(help).await?;
    Ok(())
}

/// Print Korean help message
#[poise::command(slash_command, prefix_command, rename = "help-kr")]
async fn help_kr(ctx: Context<'_>) -> Result<(), Error> {
    let help = "**게키츄마이 채보 정보 전달 디스코드 봇 GCM-bot입니다.**

**사용 방법:**
방법 1. 슬래시 명령어 (추천 방법)
방법 2. @GCM-bot `명령어-이름` `명령어-변수`

**노래 제목으로는 한글 제목 및 영어 별명들이 지원됩니다. 이것저것 시도해 보세요!**

**사용 예시:**
/mai-info 브브브
@GCM-bot mai-info 새벽까지 앞으로 3초

**개발 예정:** 츄니즘, 온게키 지원

한글 곡제목 건의, 상세 사용법, 버그 리포트를 위해서는
다음 링크를 참조해주세요: <https://github.com/lomotos10/GCM-bot/blob/main/README-kr.md>";
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
                chuni_info(),
                chuni_jacket(),
                ongeki_info(),
                ongeki_jacket(),
                help(),
                help_kr(),
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
                let mai_aliases = set_aliases(mai_charts.keys(), "maimai")?;
                let chuni_charts = set_chuni_charts()?;
                let chuni_aliases = set_aliases(chuni_charts.keys(), "chuni")?;
                let ongeki_charts = set_ongeki_charts()?;
                let ongeki_aliases = set_aliases(ongeki_charts.keys(), "ongeki")?;
                let cooldown_server_ids = {
                    let file = File::open("data/cooldown-server-ids.txt")?;
                    BufReader::new(file)
                        .lines()
                        .map(|l| l.unwrap().parse::<u64>())
                        .filter(|b| b.is_ok())
                        .map(|l| GuildId(l.unwrap()))
                        .collect::<HashSet<_>>()
                };
                let cooldown_channel_exception_ids = {
                    let file = File::open("data/cooldown-channel-exception-ids.txt")?;
                    BufReader::new(file)
                        .lines()
                        .map(|l| l.unwrap().parse::<u64>())
                        .filter(|b| b.is_ok())
                        .map(|l| ChannelId(l.unwrap()))
                        .collect::<HashSet<_>>()
                };
                let timestamps = Arc::new(Mutex::new(
                    cooldown_server_ids
                        .iter()
                        .map(|k| (*k, (HashMap::new(), HashMap::new())))
                        .collect(),
                ));
                let alias_log = Arc::new(Mutex::new(File::create(format!(
                    "alias_log_{}.txt",
                    chrono::prelude::Utc::now()
                ))?));

                Ok(Data {
                    mai_charts,
                    mai_aliases,
                    mai_jacket_prefix: fs::read_to_string("data/maimai-jacket-prefix.txt")?,

                    chuni_charts,
                    chuni_aliases,

                    ongeki_charts,
                    ongeki_aliases,

                    cooldown_server_ids,
                    cooldown_channel_exception_ids,
                    timestamps,
                    alias_log,
                })
            })
        });

    println!("Starting run:");
    framework.run().await.unwrap();
}
