use itertools::izip;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, PartialEq, Default)]
struct MaiDifficulty {
    st: Option<Difficulty>,
    dx: Option<Difficulty>,
}

#[derive(Debug, PartialEq, Clone, Default)]
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

#[derive(Debug, PartialEq, Clone)]
struct Sheet {
    brk: usize,
    hold: usize,
    slide: usize,
    tap: usize,
    touch: usize,
}

#[derive(Debug, PartialEq, Default)]
struct MaiInfo {
    jp_lv: Option<MaiDifficulty>,
    intl_lv: Option<MaiDifficulty>,
    jp_jacket: Option<String>,
    // intl_jacket: Option<String>,
    title: String,
    artist: String,
    bpm: Option<usize>,
    dx_sheets: Vec<Sheet>,
    st_sheets: Vec<Sheet>,
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

fn serdest_to_usize(st: &serde_json::Value) -> usize {
    if let serde_json::Value::Number(s) = st {
        s.as_u64().unwrap() as usize
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

/// Get maimai song info
#[poise::command(slash_command, prefix_command)]
async fn mai_info(
    ctx: Context<'_>,
    #[description = "Song title"] title: String,
    #[description = "Include note info"] notes: Option<bool>,
) -> Result<(), Error> {
    let song = ctx.data().mai_charts.get(&title);

    if song == None {
        return Ok(());
    }
    let song = song.unwrap();

    let mut description = format!("**Artist:** {}", song.artist);
    if let Some(bpm) = song.bpm {
        description = format!("{}\n**BPM:** {}", description, bpm);
    }

    let in_lv = &song.intl_lv;
    let jp_lv = &song.jp_lv;

    let (in_st, in_dx) = if let Some(in_lv) = in_lv {
        (in_lv.st.is_some(), in_lv.dx.is_some())
    } else {
        (false, false)
    };
    let (jp_st, jp_dx) = if let Some(jp_lv) = jp_lv {
        (jp_lv.st.is_some(), jp_lv.dx.is_some())
    } else {
        (false, false)
    };

    let jp_dx_txt = if jp_dx {
        let jp_dx_lv = jp_lv.as_ref().unwrap().dx.as_ref().unwrap();
        format!(
            "BAS **{}**/ADV **{}**/EXP **{}**/MAS **{}**{}",
            jp_dx_lv.bas,
            jp_dx_lv.adv,
            jp_dx_lv.exp,
            jp_dx_lv.mas,
            if let Some(rem) = &jp_dx_lv.extra {
                format!("/REM **{}**", rem)
            } else {
                "".to_string()
            }
        )
    } else {
        "**Unreleased**".to_string()
    };
    let in_dx_txt = if in_dx {
        let in_dx_lv = in_lv.as_ref().unwrap().dx.as_ref().unwrap();
        format!(
            "BAS **{}**/ADV **{}**/EXP **{}**/MAS **{}**{}",
            in_dx_lv.bas,
            in_dx_lv.adv,
            in_dx_lv.exp,
            in_dx_lv.mas,
            if let Some(rem) = &in_dx_lv.extra {
                format!("/REM **{}**", rem)
            } else {
                "".to_string()
            }
        )
    } else {
        "**Unreleased**".to_string()
    };
    if in_dx || jp_dx {
        if jp_dx_txt == in_dx_txt {
            description = format!(
                "{}

**Level(DX):**
:flag_jp::globe_with_meridians: {}",
                description, jp_dx_txt
            );
        } else {
            description = format!(
                "{}

**Level(DX):**
:flag_jp: {}
:globe_with_meridians: {}",
                description, jp_dx_txt, in_dx_txt
            );
        }
    };

    let jp_st_txt = if jp_st {
        let jp_st_lv = jp_lv.as_ref().unwrap().st.as_ref().unwrap();
        format!(
            "BAS **{}**/ADV **{}**/EXP **{}**/MAS **{}**{}",
            jp_st_lv.bas,
            jp_st_lv.adv,
            jp_st_lv.exp,
            jp_st_lv.mas,
            if let Some(rem) = &jp_st_lv.extra {
                format!("/REM **{}**", rem)
            } else {
                "".to_string()
            }
        )
    } else {
        "**Unreleased**".to_string()
    };
    let in_st_txt = if in_st {
        let in_st_lv = in_lv.as_ref().unwrap().st.as_ref().unwrap();
        format!(
            "BAS **{}**/ADV **{}**/EXP **{}**/MAS **{}**{}",
            in_st_lv.bas,
            in_st_lv.adv,
            in_st_lv.exp,
            in_st_lv.mas,
            if let Some(rem) = &in_st_lv.extra {
                format!("/REM **{}**", rem)
            } else {
                "".to_string()
            }
        )
    } else {
        "**Unreleased**".to_string()
    };
    if in_st || jp_st {
        if in_st_txt == jp_st_txt {
            description = format!(
                "{}

**Level(ST):**
:flag_jp::globe_with_meridians: {}",
                description, jp_st_txt
            );
        } else {
            description = format!(
                "{}

**Level(ST):**
:flag_jp: {}
:globe_with_meridians: {}",
                description, jp_st_txt, in_st_txt
            );
        }
    };

    ctx.send(|f| {
        f.embed(|f| {
            let mut f = f
                .title(&title)
                .description(description)
                .color(serenity::utils::Color::from_rgb(0, 255, 255));
            if let Some(jacket) = &song.jp_jacket {
                f = f.image(format!(
                    "***REMOVED***{}",
                    jacket
                ));
                // .thumbnail("https://info-maimai.sega.jp/wp-content/uploads/2022/05/88cc6aab008536ed331ce614d377607b.jpg");
            }

            if notes == Some(true) && (!song.dx_sheets.is_empty() || !song.st_sheets.is_empty()) {
                let mut idx = "".to_string();
                let mut notes = vec![];
                for _ in 0..6 {
                    notes.push("".to_string());
                }
                if !song.dx_sheets.is_empty() && !song.st_sheets.is_empty() {
                    idx = "DX".to_string();
                    for note in &mut notes {
                        *note = "\n".to_string();
                    }
                }
                for (diff, sheet) in izip!(["**BAS**", "**ADV**", "**EXP**", "**MAS**", "**REM**"], &song.dx_sheets) {
                    idx = format!("{}\n{}", idx, diff);
                    notes[0] = format!("{}\n{}", notes[0], sheet.tap);
                    notes[1] = format!("{}\n{}", notes[1], sheet.hold);
                    notes[2] = format!("{}\n{}", notes[2], sheet.slide);
                    notes[3] = format!("{}\n{}", notes[3], sheet.touch);
                    notes[4] = format!("{}\n{}", notes[4], sheet.brk);
                    notes[5] = format!(
                        "{}\n{}",
                        notes[5],
                        sheet.tap + sheet.hold + sheet.slide + sheet.touch + sheet.brk
                    );
                }
                if !song.dx_sheets.is_empty() && !song.st_sheets.is_empty() {
                    idx = "ST".to_string();
                    for note in &mut notes {
                        *note = "\n".to_string();
                    }
                }
                for (diff, sheet) in izip!(["**BAS**", "**ADV**", "**EXP**", "**MAS**", "**REM**"], &song.st_sheets) {
                    idx = format!("{}\n{}", idx, diff);
                    notes[0] = format!("{}\n{}", notes[0], sheet.tap);
                    notes[1] = format!("{}\n{}", notes[1], sheet.hold);
                    notes[2] = format!("{}\n{}", notes[2], sheet.slide);
                    notes[3] = format!("{}\n", sheet.touch);
                    notes[4] = format!("{}\n{}", notes[4], sheet.brk);
                    notes[5] = format!(
                        "{}\n{}",
                        notes[5],
                        sheet.tap + sheet.hold + sheet.slide + sheet.touch + sheet.brk
                    );
                }
                f = f
                    .field("ㅤ", idx, true)
                    .field("TAP", &notes[0], true)
                    .field("HLD", &notes[1], true)
                    .field("SLD", &notes[2], true);
                if !song.dx_sheets.is_empty() {
                    f = f.field("TCH", &notes[3], true);
                }
                f = f
                    .field("BRK", &notes[4], true)
                    .field("TOT", &notes[5], true);
            }
            f
        })
    })
    .await?;
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

        // let jp_jacket = serdest_to_string(song.get("image_url").unwrap());
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
                jp_jacket: None,
                // intl_jacket: None,
                title,
                artist,
                bpm: None,
                dx_sheets: vec![],
                st_sheets: vec![],
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
            println!("{}", &title); // This isn't for debug; don't delete!
            charts.insert(
                title.clone(),
                MaiInfo {
                    jp_lv: None,
                    intl_lv: Some(mai_difficulty),
                    jp_jacket: None,
                    title: title,
                    artist: "TODO".to_string(),
                    bpm: None,
                    dx_sheets: vec![],
                    st_sheets: vec![],
                },
            );
        }
    }

    // Get zeta DB
    let zeta = fs::read_to_string("data/zeta.txt")?;
    let zeta = zeta.trim();
    let s = get_curl(zeta);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();
    let songs = &if let serde_json::Value::Object(s) = songs {
        s
    } else {
        panic!()
    }["songs"];
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

        let title = serdest_to_string(song.get("songId").unwrap());
        // Edge case handling for duplicate title
        let title = if title == "Link" {
            "Link (maimai)".to_string()
        } else if title == "Link (2)" {
            "Link".to_string()
        } else {
            title
        };

        if !charts.contains_key(&title) {
            continue;
        }
        let jp_jacket = serdest_to_string(song.get("imageName").unwrap());

        let sheets = if let serde_json::Value::Array(m) = &song["sheets"] {
            m
        } else {
            panic!()
        };
        let mut st_sheet_data = vec![];
        let mut dx_sheet_data = vec![];
        for sheet in sheets {
            let sheet = &if let serde_json::Value::Object(m) = sheet {
                m
            } else {
                panic!()
            };
            let notes = if let serde_json::Value::Object(m) = &sheet["noteCounts"] {
                m
            } else {
                panic!()
            };
            if notes["tap"] == serde_json::Value::Null {
                break;
            }
            let sheet_info = Sheet {
                brk: serdest_to_usize(&notes["break"]),
                hold: serdest_to_usize(&notes["hold"]),
                slide: serdest_to_usize(&notes["slide"]),
                tap: serdest_to_usize(&notes["tap"]),
                touch: if notes.contains_key("touch") {
                    if notes["touch"] == serde_json::Value::Null {
                        0
                    } else {
                        serdest_to_usize(&notes["touch"])
                    }
                } else {
                    0
                },
            };
            if sheet["type"] == "dx" {
                dx_sheet_data.push(sheet_info);
            } else if sheet["type"] == "std" {
                st_sheet_data.push(sheet_info);
            } else {
                panic!();
            }
        }

        let bpm = song.get("bpm");
        let bpm = if bpm == Some(&serde_json::Value::Null) {
            None
        } else {
            bpm.map(|v| serdest_to_usize(v))
        };

        let r = charts.get_mut(&title).unwrap();
        r.jp_jacket = Some(jp_jacket);
        r.bpm = bpm;
        r.dx_sheets = dx_sheet_data;
        r.st_sheets = st_sheet_data;
    }

    Ok(charts)
}

// User data, which is stored and accessible in all command invocations
struct Data {
    mai_charts: Box<HashMap<String, MaiInfo>>,
}

#[tokio::main]
async fn main() {
    // set_mai_charts();
    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![mai_info(), register()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    mai_charts: Box::new(set_mai_charts()?),
                })
            })
        });

    framework.run().await.unwrap();
}
