use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {
    mai_charts: Box<HashMap<String, MaiInfo>>,
}

#[derive(Debug, PartialEq)]
struct MaiDifficulty {
    st: Option<Difficulty>,
    dx: Option<Difficulty>,
}

#[derive(Debug, PartialEq, Clone)]
struct Difficulty {
    bas: String,
    adv: String,
    exp: String,
    mas: String,
    extra: Option<String>,
}

lazy_static! {
    static ref SONG_REPLACEMENT: HashMap<String, String> = {
        [
            ("GIGANTOMAKHIA", "GIGANTØMAKHIA"),
            ("D✪N’T ST✪P R✪CKIN’", "D✪N’T  ST✪P  R✪CKIN’"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>()
    };
}

#[derive(Debug, PartialEq)]
struct MaiInfo {
    jp_lv: Option<MaiDifficulty>,
    intl_lv: Option<MaiDifficulty>,
    jp_jacket: Option<String>,
    // intl_jacket: Option<String>,
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

fn float_to_level(f: &str) -> String {
    let f = f.parse::<f32>().unwrap().abs();
    let decimal = f - f.floor();

    if decimal < 0.65 {
        f.floor().to_string()
    } else {
        format!("{}+", f.floor())
    }
}

fn set_mai_charts() -> Result<HashMap<String, MaiInfo>, Error> {
    let mut charts = HashMap::new();

    // Get JP difficulty.
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
        // Edge case handling for duplicate title
        let title =
            if title == "Link" && serdest_to_string(song.get("catcode").unwrap()) == "maimai" {
                "Link (maimai)".to_string()
            } else {
                title
            };

        let jp_jacket = serdest_to_string(song.get("image_url").unwrap());
        let artist = serdest_to_string(song.get("artist").unwrap());

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
        let r = charts.insert(
            title.clone(),
            MaiInfo {
                jp_lv: Some(jp_lv),
                intl_lv: None,
                jp_jacket: Some(jp_jacket),
                // intl_jacket: None,
                title,
                artist,
            },
        );
        assert_eq!(r, None);
    }

    // Get intl difficulty.
    // deleted songs
    let mut jp_del_songs = HashSet::new();
    let file = File::open("data/jp_del.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        jp_del_songs.insert(line);
    }

    let file = File::open("in_lv.csv")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.split("\t").collect::<Vec<_>>();
        assert_eq!(line.len(), 7);
        let title = SONG_REPLACEMENT
            .get(line[6])
            .unwrap_or(&line[6].to_string())
            .to_string();
        // Edge case handling for duplicate title
        let title = if title == "Link" && line[4] == "-12" {
            "Link (maimai)".to_string()
        } else {
            title
        };
        if jp_del_songs.contains(&title) {
            continue;
        }

        let difficulty = Difficulty {
            bas: float_to_level(line[1]),
            adv: float_to_level(line[2]),
            exp: float_to_level(line[3]),
            mas: float_to_level(line[4]),
            extra: if line[5] == "0" {
                None
            } else {
                Some(float_to_level(line[5]))
            },
        };
        let mai_difficulty = if line[0] == "0" {
            MaiDifficulty {
                st: Some(difficulty.clone()),
                dx: None,
            }
        } else {
            MaiDifficulty {
                st: None,
                dx: Some(difficulty.clone()),
            }
        };

        if charts.contains_key(&title) {
            let entry = charts.get_mut(&title).unwrap();

            let l = &mut entry.intl_lv;
            if line[0] == "0" {
                // ST chart
                match l {
                    None => {
                        *l = Some(mai_difficulty);
                    }
                    Some(v) => {
                        assert_eq!(v.st, None);
                        v.st = Some(difficulty);
                    }
                }
            } else {
                // DX chart
                match l {
                    None => {
                        *l = Some(mai_difficulty);
                    }
                    Some(v) => {
                        assert_eq!(v.dx, None);
                        v.dx = Some(difficulty);
                    }
                }
            }
        } else {
            println!("{}", &title);
            charts.insert(
                title.clone(),
                MaiInfo {
                    jp_lv: None,
                    intl_lv: Some(mai_difficulty),
                    jp_jacket: None,
                    title: title,
                    artist: "TODO".to_string(),
                },
            );
        }
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
