use std::collections::HashMap;
use std::fs;
use std::fs::File;

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {
    mai_charts: Box<HashMap<String, MaiInfo>>,
}

#[derive(Debug)]
struct MaiDifficulty {
    st: Option<Difficulty>,
    dx: Option<Difficulty>,
}

#[derive(Debug)]
struct Difficulty {
    bas: String,
    adv: String,
    exp: String,
    mas: String,
    extra: Option<String>,
}

#[derive(Debug)]
struct MaiInfo {
    jp_lv: Option<MaiDifficulty>,
    intl_lv: Option<MaiDifficulty>,
    jp_jacket: Option<String>,
    intl_jacket: Option<String>,

    title: String,
    artist: String,
    // bpm: String,
}

#[derive(Debug)]
enum Game {
    Geki,
    Chu,
    Mai,
}

fn serdest_to_string(st: &serde_json::Value) -> String {
    if let serde_json::Value::String(s) = st {
        s.to_string()
    } else {
        panic!()
    }
}

fn get_curl(url: &str) -> String {
    let mut data = Vec::new();
    let mut handle = curl::easy::Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let s = match std::str::from_utf8(&data) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    s.to_string()
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn mai_info(
    ctx: Context<'_>,
    #[description = "Selected user"] title: String,
) -> Result<(), Error> {
    // ctx.send(|f| f
    //     .content(user)
    //     .embed(|f| f
    //         .title("Much versatile, very wow")
    //         .description("I need more documentation ok?")
    //     )
    //     .ephemeral(true) // this one only applies in application commands though
    // ).await?;
    Ok(())
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

fn set_mai_charts() -> Result<HashMap<String, MaiInfo>, Error> {
    let mut charts = HashMap::new();

    let jp_url = fs::read_to_string("data/maimai-jp.txt")?;
    let jp_url = jp_url.trim();
    let s = get_curl(jp_url);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();

    let songs = if let serde_json::Value::Array(s) = songs {
        s
    } else {
        panic!()
    };

    for song in songs {
        let song = if let serde_json::Value::Object(m) = song {
            m
        } else {
            panic!()
        };

        let title = serdest_to_string(song.get("title").unwrap());
        let jp_jacket = serdest_to_string(song.get("image_url").unwrap());
        let artist = serdest_to_string(song.get("artist").unwrap());
        // let ordering = serdest_to_string(song.get("sort").unwrap())
        //     .parse::<usize>()
        //     .unwrap();
        let st_lv = if song.contains_key("lev_bas") {
            Some(Difficulty {
                bas: serdest_to_string(song.get("lev_bas").unwrap()),
                adv: serdest_to_string(song.get("lev_adv").unwrap()),
                exp: serdest_to_string(song.get("lev_exp").unwrap()),
                mas: serdest_to_string(song.get("lev_mas").unwrap()),
                extra: if song.contains_key("lev_remas") {
                    Some(serdest_to_string(song.get("lev_remas").unwrap()))
                } else {
                    None
                },
            })
        } else {
            None
        };
        let dx_lv = if song.contains_key("dx_lev_bas") {
            Some(Difficulty {
                bas: serdest_to_string(song.get("dx_lev_bas").unwrap()),
                adv: serdest_to_string(song.get("dx_lev_adv").unwrap()),
                exp: serdest_to_string(song.get("dx_lev_exp").unwrap()),
                mas: serdest_to_string(song.get("dx_lev_mas").unwrap()),
                extra: if song.contains_key("dx_lev_remas") {
                    Some(serdest_to_string(song.get("dx_lev_remas").unwrap()))
                } else {
                    None
                },
            })
        } else {
            None
        };

        let jp_lv = MaiDifficulty {
            st: st_lv,
            dx: dx_lv,
        };
        charts.insert(
            title.clone(),
            MaiInfo {
                jp_lv: Some(jp_lv),
                intl_lv: None,
                jp_jacket: Some(jp_jacket),
                intl_jacket: None,
                title,
                artist,
            },
        );
    }

    Ok(charts)
}

#[tokio::main]
async fn main() {
    println!("{:#?}", set_mai_charts());

    // let framework = poise::Framework::build()
    //     .options(poise::FrameworkOptions {
    //         commands: vec![age(), register()],
    //         ..Default::default()
    //     })
    //     .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
    //     .intents(serenity::GatewayIntents::non_privileged())
    //     .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {
    //         mai_charts: Box::new(set_mai_charts())
    //     }) }));

    // framework.run().await.unwrap();
}
