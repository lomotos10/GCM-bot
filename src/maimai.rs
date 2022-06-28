use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader},
};

use lazy_static::lazy_static;
use poise::serenity_prelude as serenity;

use crate::utils::*;

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

/// Get maimai song info
#[poise::command(
    slash_command,
    prefix_command,
    rename = "mai-info",
    aliases("maiinfo", "mai_info")
)]
pub async fn mai_info(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Selector\", \"bbb\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    println!("1");
    let actual_title = get_title(&title, &ctx.data().mai_aliases);
    println!("{:?}", actual_title);
    if actual_title == None {
        let closest = get_closest_title(&title, &ctx.data().mai_aliases);
        let reply = format!(
            "I couldn't find the results for **{}**;
Did you mean **{}** (for **{}**)?",
            title, closest.0, closest.1
        );
        ctx.send(|f| f.ephemeral(true).content(reply)).await?;
        return Ok(());
    }
    let title = actual_title.unwrap();

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
                .title(if title == "Link (maimai)" {
                    "Link"
                } else {
                    &title
                })
                .description(description)
                .color(serenity::utils::Color::from_rgb(0, 255, 255));
            if let Some(jacket) = &song.jp_jacket {
                f = f.thumbnail(format!("{}{}", ctx.data().mai_jacket_prefix, jacket));
            }

            f
        })
    })
    .await?;
    Ok(())
}

/// Get maimai song jacket
#[poise::command(
    slash_command,
    prefix_command,
    rename = "mai-jacket",
    aliases("maijacket", "mai_jacket")
)]
pub async fn mai_jacket(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Selector\", \"bbb\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    let actual_title = get_title(&title, &ctx.data().mai_aliases);
    if actual_title == None {
        let closest = get_closest_title(&title, &ctx.data().mai_aliases);
        let reply = format!(
            "We couldn't find the results for **{}**;
Did you mean **{}** (for **{}**)?",
            title, closest.0, closest.1
        );
        ctx.send(|f| f.ephemeral(true).content(reply)).await?;
        return Ok(());
    }
    let title = actual_title.unwrap();
    let jacket = &ctx.data().mai_charts[&title].jp_jacket;
    if let Some(jacket) = jacket {
        ctx.send(|f| {
            f.attachment(serenity::AttachmentType::Image(
                url::Url::parse(&format!("{}{}", ctx.data().mai_jacket_prefix, jacket)).unwrap(),
            ))
        })
        .await?;
    }
    Ok(())
}

pub fn set_mai_charts() -> Result<HashMap<String, MaiInfo>, Error> {
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
    let file = File::open("data/jp-del.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        jp_del_songs.insert(line);
    }
    let mut intl_del_songs = HashSet::new();
    let file = File::open("data/intl-del.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        intl_del_songs.insert(line);
    }

    let file = File::open("in_lv.csv")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.split('\t').collect::<Vec<_>>();
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
        if jp_del_songs.contains(&title) || intl_del_songs.contains(&title) {
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
                    title,
                    artist: "TODO".to_string(),
                    bpm: None,
                    dx_sheets: vec![],
                    st_sheets: vec![],
                },
            );
        }
    }

    // Get info DB
    let info = fs::read_to_string("data/maimai-info.txt")?;
    let info = info.trim();
    let s = get_curl(info);

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
            let sheet_info = MaiSheet {
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
            bpm.map(serdest_to_usize)
        };

        let r = charts.get_mut(&title).unwrap();
        r.jp_jacket = Some(jp_jacket);
        r.bpm = bpm;
        r.dx_sheets = dx_sheet_data;
        r.st_sheets = st_sheet_data;

        if r.jp_lv == None {
            if let Some(artist) = song.get("artist") {
                r.artist = serdest_to_string(artist);
            }
        }
    }

    Ok(charts)
}

pub fn set_mai_aliases() -> Result<Aliases, Error> {
    let mut lowercased = HashMap::new();
    let mut lowercased_and_unspaced = HashMap::new();
    let mut alphanumeric_only = HashMap::new();
    let mut alphanumeric_and_ascii = HashMap::new();
    let mut nicknames = HashMap::new();
    let file = File::open("data/aliases/en/maimai.tsv")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let split = line.split('\t');
        let split = split.collect::<Vec<_>>();
        let title = split[0];

        let namem1 = title.to_lowercase();
        let a = lowercased.insert(namem1.to_string(), title.to_string());
        if let Some(a) = a {
            println!(
                "Alias-1 {} (for {}) shadowed by same alias-1 for {}",
                namem1, a, title
            );
        }

        let name0 = title.to_lowercase().split_whitespace().collect::<String>();
        let a = lowercased_and_unspaced.insert(name0.to_string(), title.to_string());
        if let Some(a) = a {
            println!(
                "Alias0 {} (for {}) shadowed by same alias0 for {}",
                name0, a, title
            );
        }

        let name1 = name0
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        if !name1.is_empty() {
            let a = alphanumeric_only.insert(name1.to_string(), title.to_string());
            if let Some(a) = a {
                println!(
                    "Alias1 {} (for {}) shadowed by same alias1 for {}",
                    name1, a, title
                );
            }
        }

        let name2 = name1.chars().filter(|c| c.is_ascii()).collect::<String>();
        if !name2.is_empty() {
            let a = alphanumeric_and_ascii.insert(name2.to_string(), title.to_string());
            if let Some(a) = a {
                println!(
                    "Alias2 {} (for {}) shadowed by same alias2 for {}",
                    name2, a, title
                );
            }
        }

        let nickname_slice = &split[1..];
        for nickname in nickname_slice {
            let nick = nickname
                .to_lowercase()
                .split_whitespace()
                .collect::<String>();
            let nick = nick
                .chars()
                .filter(|c| c.is_alphanumeric() && c.is_ascii())
                .collect::<String>();
            let a = nicknames.insert(nick.to_string(), title.to_string());
            if let Some(a) = a {
                println!(
                    "Alias3 {} (for {}) shadowed by same alias3 for {}",
                    nick, a, title
                );
            }
        }
    }

    // I fucking hate myself but I don't have the energy to fix this
    for (name0, title) in lowercased_and_unspaced.iter() {
        if lowercased.contains_key(name0) {
            // Don't delete this; it's for actual debugging!
            if title != &lowercased_and_unspaced[name0] {
                println!(
                    "Alias0 {} (for {}) shadowed by same alias-1 for {}",
                    name0, title, lowercased_and_unspaced[name0]
                );
            }
        }
    }
    for (name1, title) in alphanumeric_only.iter() {
        if lowercased_and_unspaced.contains_key(name1) {
            // Don't delete this; it's for actual debugging!
            if title != &lowercased_and_unspaced[name1] {
                println!(
                    "Alias1 {} (for {}) shadowed by same alias0 for {}",
                    name1, title, lowercased_and_unspaced[name1]
                );
            }
        }
    }
    for (name2, title) in alphanumeric_and_ascii.iter() {
        if alphanumeric_only.contains_key(name2) {
            // Don't delete this; it's for actual debugging!
            if title != &alphanumeric_only[name2] {
                println!(
                    "Alias2 {} (for {}) shadowed by same alias1 for {}",
                    name2, title, alphanumeric_only[name2]
                );
            }
        }
    }
    for (nick, title) in nicknames.iter() {
        if alphanumeric_and_ascii.contains_key(nick) {
            // Don't delete this; it's for actual debugging!
            if title != &alphanumeric_and_ascii[nick] {
                println!(
                    "Alias3 {} (for {}) shadowed by same alias2 for {}",
                    nick, title, alphanumeric_and_ascii[nick]
                );
            }
        }
    }

    Ok(Aliases {
        lowercased,
        lowercased_and_unspaced,
        alphanumeric_only,
        alphanumeric_and_ascii,
        nicknames,
    })
}
