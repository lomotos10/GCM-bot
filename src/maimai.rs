use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    io::{BufRead, BufReader},
    time::Duration,
};

use gcm_macro::{info_template, jacket_template};
use lazy_static::lazy_static;
use poise::{
    serenity_prelude::{
        self as serenity, AttachmentType, CreateActionRow, CreateButton, InteractionResponseType,
    },
    ReplyHandle,
};

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

fn get_mai_embed(title: String, ctx: Context<'_>) -> Result<(String, Option<String>), Error> {
    let song = ctx.data().mai_charts.get(&title);

    let song = song.unwrap();

    let mut description = if song.deleted {
        "**THIS SONG IS DELETED**\n\n"
    } else {
        ""
    }
    .to_string();

    description = format!(
        "{}**Artist:** {}",
        description,
        song.artist.replace('*', "\\*")
    );
    if let Some(version) = &song.version {
        description = format!("{}\n**Version:** {}", description, version);
    }
    if let Some(bpm) = song.bpm {
        description = format!("{}\n**BPM:** {}", description, bpm);
    }
    if song.deleted {
        let (st, dx) = if let Some(jp_lv) = &song.jp_lv {
            (jp_lv.st.is_some(), jp_lv.dx.is_some())
        } else {
            (false, false)
        };

        if dx {
            description = format!(
                "{}\n\n**Level(DX):**\n{}",
                description,
                level_description(song.jp_lv.as_ref().unwrap().dx.as_ref().unwrap(),  &title)
            )
        }
        if st {
            description = format!(
                "{}\n\n**Level(ST):**\n{}",
                description,
                level_description(song.jp_lv.as_ref().unwrap().st.as_ref().unwrap(), &title)
            )
        }
    } else {
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
            level_description(jp_lv.as_ref().unwrap().dx.as_ref().unwrap(),  &title)
        } else {
            "**Unreleased**".to_string()
        };
        let in_dx_txt = if in_dx {
            level_description(in_lv.as_ref().unwrap().dx.as_ref().unwrap(),  &title)
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
            level_description(jp_lv.as_ref().unwrap().st.as_ref().unwrap(), &title)
        } else {
            "**Unreleased**".to_string()
        };
        let in_st_txt = if in_st {
            level_description(in_lv.as_ref().unwrap().st.as_ref().unwrap(),  &title)
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
    }
    Ok((description, song.jp_jacket.clone()))
}

fn mai_duplicate_alias_to_title(title: &String) -> String {
    if title == "Link (maimai)" {
        "Link".to_string()
    } else {
        title.to_string()
    }
}

extern crate gcm_macro;

/// Get maimai song info
#[poise::command(slash_command, prefix_command, rename = "mai-info")]
pub async fn mai_info(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Selector\", \"bbb\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    info_template!("mai", "0, 255, 255", "ctx.data().mai_jacket_prefix");
    Ok(())
}

fn level_description(lv: &Difficulty, title: &str) -> String {
    let title = urlencoding::encode(title);
    format!(
        // "BAS **{}{}**/ADV **{}{}**/EXP **{}{}**/MAS **{}{}**{}",
        "[B](https://www.youtube.com/results?search_query=maimai+{}+BASIC) **{}**{} / [A](https://www.youtube.com/results?search_query=maimai+{}+ADVANCED) **{}**{} / [E](https://www.youtube.com/results?search_query=maimai+{}+EXPERT) **{}**{} / [M](https://www.youtube.com/results?search_query=maimai+{}+MASTER) **{}**{}{}",
        title,
        lv.bas,
        constant_to_string(lv.bas_c),
        title,
        lv.adv,
        constant_to_string(lv.adv_c),
        title,
        lv.exp,
        constant_to_string(lv.exp_c),
        title,
        lv.mas,
        constant_to_string(lv.mas_c),
        if let Some(rem) = &lv.extra {
            format!(" / [R](https://www.youtube.com/results?search_query=maimai+{}+Re:MASTER) **{}**{}", title, rem, constant_to_string(lv.extra_c))
        } else {
            "".to_string()
        }
    )
}

/// Get maimai song jacket
#[poise::command(slash_command, prefix_command, rename = "mai-jacket")]
pub async fn mai_jacket(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Selector\", \"bbb\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    jacket_template!("mai", "ctx.data().mai_jacket_prefix");
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
                ..Default::default()
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
                ..Default::default()
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
                version: None,
                deleted: false,
            },
        );
        assert_eq!(r, None);
    }

    // Get jp constants
    let file = File::open("jp_lv.csv")?;
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
        let title = if title == "Link" && line[3] == "-12" {
            "Link (maimai)".to_string()
        } else {
            title
        };

        if charts.contains_key(&title) {
            let entry = charts.get_mut(&title).unwrap();

            let l = &mut entry.jp_lv;
            match l {
                None => {
                    let difficulty = Difficulty {
                        bas: float_to_level(line[1]),
                        bas_c: float_to_constant(line[1]),
                        adv: float_to_level(line[2]),
                        adv_c: float_to_constant(line[2]),
                        exp: float_to_level(line[3]),
                        exp_c: float_to_constant(line[3]),
                        mas: float_to_level(line[4]),
                        mas_c: float_to_constant(line[4]),
                        extra: if line[5] == "0" {
                            None
                        } else {
                            Some(float_to_level(line[5]))
                        },
                        extra_c: if line[5] == "0" {
                            None
                        } else {
                            float_to_constant(line[5])
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
                    *l = Some(mai_difficulty);
                }
                Some(v) => {
                    let diff = if line[0] == "0" {
                        v.st.as_mut().unwrap()
                    } else {
                        v.dx.as_mut().unwrap()
                    };
                    if diff.bas == float_to_level(line[1]) {
                        diff.bas_c = float_to_constant(line[1]);
                    } else {
                        println!(
                            "Conflict on {} {} BAS: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.bas,
                            float_to_level(line[1])
                        );
                    }
                    if diff.adv == float_to_level(line[2]) {
                        diff.adv_c = float_to_constant(line[2]);
                    } else {
                        println!(
                            "Conflict on {} {} ADV: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.adv,
                            float_to_level(line[2])
                        );
                    }
                    if diff.exp == float_to_level(line[3]) {
                        diff.exp_c = float_to_constant(line[3]);
                    } else {
                        println!(
                            "Conflict on {} {} EXP: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.exp,
                            float_to_level(line[3])
                        );
                    }
                    if diff.mas == float_to_level(line[4]) {
                        diff.mas_c = float_to_constant(line[4]);
                    } else {
                        println!(
                            "Conflict on {} {} MAS: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.mas,
                            float_to_level(line[4])
                        );
                    }
                    if line[5] != "0" {
                        if diff.extra == Some(float_to_level(line[5])) {
                            diff.extra_c = float_to_constant(line[5]);
                        } else {
                            println!(
                                "Conflict on {} {} REM: {:?} vs {}",
                                title,
                                if line[0] == "0" { "ST" } else { "DX" },
                                diff.extra,
                                float_to_level(line[5])
                            );
                        }
                    }
                }
            }
        } else {
            panic!("Sus");
        }
    }

    // Get intl difficulty.
    let jp_and_intl_version_is_different = false;
    if jp_and_intl_version_is_different {
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
            let title = if title == "Link" && line[3] == "-12" {
                "Link (maimai)".to_string()
            } else {
                title
            };

            let difficulty = Difficulty {
                bas: float_to_level(line[1]),
                bas_c: float_to_constant(line[1]),
                adv: float_to_level(line[2]),
                adv_c: float_to_constant(line[2]),
                exp: float_to_level(line[3]),
                exp_c: float_to_constant(line[3]),
                mas: float_to_level(line[4]),
                mas_c: float_to_constant(line[4]),
                extra: if line[5] == "0" {
                    None
                } else {
                    Some(float_to_level(line[5]))
                },
                extra_c: if line[5] == "0" {
                    None
                } else {
                    float_to_constant(line[5])
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
                        version: None,
                        deleted: false,
                    },
                );
            }
        }
    } else {
        // Same version; copy jp difficulty into intl
        for info in charts.values_mut() {
            if info.jp_lv.is_some() {
                (*info).intl_lv = info.jp_lv.clone();
            }
        }
        let file = File::open("data/intl-add.txt")?;
        let reader = BufReader::new(file).lines();
        for line in reader.flatten() {
            let x = charts.insert(
                line.clone(),
                MaiInfo {
                    title: line.trim().to_string(),
                    ..Default::default()
                },
            );
            assert!(x.is_none());
        }
    }

    // deleted songs
    let file = File::open("data/intl-del.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let tokens = line.split('\t').collect::<Vec<_>>();
        if tokens.len() == 1 {
            if tokens[0].is_empty() {
                continue;
            }
            (*charts.get_mut(tokens[0]).unwrap()).intl_lv = None;
        } else if tokens.len() == 2 {
            let intl_lv = (*charts.get_mut(tokens[0]).unwrap())
                .intl_lv
                .as_mut()
                .unwrap();
            if tokens[1] == "DX" {
                (*intl_lv).dx = None;
            } else if tokens[1] == "ST" {
                (*intl_lv).st = None;
            } else {
                unreachable!();
            }
        } else if tokens.len() > 2 {
            let intl_lv = (*charts.get_mut(tokens[0]).unwrap())
                .intl_lv
                .as_mut()
                .unwrap();
            let target_lv = if tokens[1] == "DX" {
                &mut intl_lv.dx
            } else if tokens[1] == "ST" {
                &mut intl_lv.st
            } else {
                unreachable!();
            }
            .as_mut()
            .unwrap();
            for token in &tokens[2..tokens.len()] {
                if *token == "REM" {
                    target_lv.extra = None;
                    target_lv.extra_c = None;
                } else {
                    unreachable!();
                }
            }
        } else {
            unreachable!();
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

        let title = song.get("songId").unwrap().as_str().unwrap();
        // Edge case handling for duplicate title
        let title = if title == "Link" {
            "Link (maimai)".to_string()
        } else if title == "Link (2)" {
            "Link".to_string()
        } else {
            title.to_string()
        };

        if !charts.contains_key(&title) {
            // Is either Utage or deleted
            if song.get("category").unwrap() == "宴会場" {
                // Utage
                // TODO
                continue;
            } else {
                // Deleted
                let title = song.get("title").unwrap().as_str().unwrap();
                charts.insert(
                    title.to_string(),
                    MaiInfo {
                        jp_lv: None,
                        intl_lv: None,
                        jp_jacket: None,
                        title: title.to_string(),
                        artist: song.get("artist").unwrap().as_str().unwrap().to_string(),
                        bpm: None,
                        dx_sheets: vec![],
                        st_sheets: vec![],
                        version: None,
                        deleted: true,
                    },
                );
            }
        }
        let jp_jacket = serdest_to_string(song.get("imageName").unwrap());

        let sheets = if let serde_json::Value::Array(m) = &song["sheets"] {
            m
        } else {
            panic!()
        };
        let mut st_sheet_data = vec![];
        let mut dx_sheet_data = vec![];
        let mut st_constants = vec![];
        let mut dx_constants = vec![];
        let mut dx_levels = vec![];
        let mut st_levels = vec![];
        for sheet in sheets {
            let sheet = if let serde_json::Value::Object(m) = sheet {
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
                dx_constants.push(match &sheet["internalLevel"] {
                    serde_json::Value::Null => None,
                    serde_json::Value::String(s) => float_to_constant(s),
                    _ => panic!("Unexpected value for sheet.internalLevel"),
                });
                dx_levels.push(sheet["level"].clone());
            } else if sheet["type"] == "std" {
                st_sheet_data.push(sheet_info);
                st_constants.push(match &sheet["internalLevel"] {
                    serde_json::Value::Null => None,
                    serde_json::Value::String(s) => float_to_constant(s),
                    _ => panic!("Unexpected value for sheet.internalLevel"),
                });
                st_levels.push(sheet["level"].clone());
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
        let version = song.get("version");
        let version = if version == Some(&serde_json::Value::Null) {
            None
        } else {
            version.map(serdest_to_string)
        };

        let r = charts.get_mut(&title).unwrap();
        r.jp_jacket = Some(jp_jacket);
        r.bpm = bpm;
        r.dx_sheets = dx_sheet_data;
        r.st_sheets = st_sheet_data;
        r.version = version;

        if let Some(_jp_lv) = &mut r.jp_lv {
            // info site has less constant info; use other site instead

            // if !dx_constants.is_empty() {
            //     let dx_diff = jp_lv.dx.as_mut().unwrap();
            //     dx_diff.bas_c = dx_constants[0];
            //     dx_diff.adv_c = dx_constants[1];
            //     dx_diff.exp_c = dx_constants[2];
            //     dx_diff.mas_c = dx_constants[3];
            //     if let Some(rem_c) = dx_constants.get(4) {
            //         dx_diff.extra_c = *rem_c;
            //     }
            // }
            // if !st_constants.is_empty() {
            //     let st_diff = jp_lv.st.as_mut().unwrap();
            //     st_diff.bas_c = st_constants[0];
            //     st_diff.adv_c = st_constants[1];
            //     st_diff.exp_c = st_constants[2];
            //     st_diff.mas_c = st_constants[3];
            //     if let Some(rem_c) = st_constants.get(4) {
            //         st_diff.extra_c = *rem_c;
            //     }
            // }
        } else {
            if let Some(artist) = song.get("artist") {
                r.artist = serdest_to_string(artist);
            }
            let dx_diff = if !dx_levels.is_empty() {
                let mut dx_diff = Difficulty {
                    bas: dx_levels[0].as_str().unwrap().to_string(),
                    adv: dx_levels[1].as_str().unwrap().to_string(),
                    exp: dx_levels[2].as_str().unwrap().to_string(),
                    mas: dx_levels[3].as_str().unwrap().to_string(),
                    extra: dx_levels.get(4).map(|x| x.as_str().unwrap().to_string()),

                    ..Default::default()
                };
                if !dx_constants.is_empty() {
                    dx_diff.bas_c = dx_constants[0];
                    dx_diff.adv_c = dx_constants[1];
                    dx_diff.exp_c = dx_constants[2];
                    dx_diff.mas_c = dx_constants[3];
                    if let Some(rem_c) = dx_constants.get(4) {
                        dx_diff.extra_c = *rem_c;
                    }
                }
                Some(dx_diff)
            } else {
                None
            };

            let st_diff = if !st_levels.is_empty() {
                let mut st_diff = Difficulty {
                    bas: st_levels[0].as_str().unwrap().to_string(),
                    adv: st_levels[1].as_str().unwrap().to_string(),
                    exp: st_levels[2].as_str().unwrap().to_string(),
                    mas: st_levels[3].as_str().unwrap().to_string(),
                    extra: st_levels.get(4).map(|x| x.as_str().unwrap().to_string()),

                    ..Default::default()
                };
                if !st_constants.is_empty() {
                    st_diff.bas_c = st_constants[0];
                    st_diff.adv_c = st_constants[1];
                    st_diff.exp_c = st_constants[2];
                    st_diff.mas_c = st_constants[3];
                    if let Some(rem_c) = st_constants.get(4) {
                        st_diff.extra_c = *rem_c;
                    }
                }
                Some(st_diff)
            } else {
                None
            };

            r.jp_lv = Some(MaiDifficulty {
                dx: dx_diff,
                st: st_diff,
            })
        }
    }

    // Add manual constant info
    let file = File::open("data/maimai-manual-add.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let line = line.split('\t').collect::<Vec<_>>();
        assert_eq!(line.len(), 5);
        let title = line[0];
        let chart = charts.get_mut(title).unwrap();
        let inner = if line[3] == "JP" {
            chart.jp_lv.as_mut()
        } else if line[3] == "IN" {
            if chart.intl_lv.is_none() {
                (*chart).intl_lv = Some(MaiDifficulty::default());
            }
            chart.intl_lv.as_mut()
        } else {
            todo!()
        }
        .unwrap();
        let inner = if line[1] == "DX" {
            inner.dx.as_mut()
        } else if line[1] == "ST" {
            if inner.st.is_none() {
                (*inner).st = Some(Difficulty::default());
            }
            inner.st.as_mut()
        } else {
            panic!()
        }
        .unwrap();
        if line[4].contains('.') {
            // Add constant
            let cst = float_to_constant(line[4]);
            if line[2] == "EXP" {
                assert!(inner.exp_c.is_none() || inner.exp_c == cst);
                if inner.exp_c == cst {
                    println!("{:?} exists on server", line);
                }
                (*inner).exp_c = cst;
            } else if line[2] == "MAS" {
                assert!(inner.mas_c.is_none() || inner.mas_c == cst);
                if inner.mas_c == cst {
                    println!("{:?} exists on server", line);
                }
                (*inner).mas_c = cst;
            } else if line[2] == "REM" {
                assert!(inner.extra_c.is_none() || inner.extra_c == cst);
                if inner.extra_c == cst {
                    println!("{:?} exists on server", line);
                }
                (*inner).extra_c = cst;
            } else {
                panic!()
            }
        } else {
            // Add level
            let diff_idx = diff_to_idx(line[2]);
            let diff_str = inner.lv(diff_idx);
            assert!(diff_str == "?");
            inner.set_lv(diff_idx, line[4].to_string());
        }
    }

    Ok(charts)
}
