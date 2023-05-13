use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use poise::{
    serenity_prelude::{
        interaction::InteractionResponseType, Color, CreateActionRow, CreateButton,
    },
    ReplyHandle,
};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    sync::Arc,
    time::Duration,
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
    static ref DX_SONGS_WITH_ST: HashSet<String> = {
        [
            "Technicians High",
            "Destr0yer",
            "Halcyon",
            "サンバランド",
            "VIIIbit Explorer",
        ]
        .iter()
        .map(|k| k.to_string())
        .collect::<HashSet<_>>()
    };
    static ref EXCEPTIONS_ST_AFTER_DX: Vec<(String, String)> = {
        [
            ("Technicians High", "maimai UNiVERSE"),
            ("Destr0yer", "maimai UNiVERSE"),
            ("Halcyon", "maimai UNiVERSE PLUS"),
            ("サンバランド", "maimai UNiVERSE PLUS"),
            ("VIIIbit Explorer", "maimai FESTiVAL"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    };
}

fn get_mai_embed(title: String, ctx: &Context<'_>) -> Result<(String, Option<String>), Error> {
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
                level_description(song.jp_lv.as_ref().unwrap().dx.as_ref().unwrap(), &title)
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
            level_description(jp_lv.as_ref().unwrap().dx.as_ref().unwrap(), &title)
        } else {
            "**Unreleased**".to_string()
        };
        let in_dx_txt = if in_dx {
            level_description(in_lv.as_ref().unwrap().dx.as_ref().unwrap(), &title)
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
            level_description(in_lv.as_ref().unwrap().st.as_ref().unwrap(), &title)
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

/// Get maimai song info
#[poise::command(slash_command, prefix_command, rename = "mai-info")]
pub async fn mai_info(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Selector\", \"bbb\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    info_template(
        ctx,
        title,
        Game::Maimai,
        Arc::new(get_mai_embed),
        (0, 255, 255),
        Arc::new(mai_duplicate_alias_to_title),
    )
    .await?;
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
    jacket_template(ctx, title, Game::Maimai).await?;
    Ok(())
}

fn set_jp_difficulty(charts: &mut HashMap<String, MaiInfo>) {
    // Get JP difficulty.
    let jp_url = fs::read_to_string("data/maimai/maimai-jp.txt").unwrap();
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

        let title = song["title"].as_str().unwrap().to_string();
        // Edge case handling for duplicate title
        let title = if title == "Link" && song["catcode"].as_str().unwrap() == "maimai" {
            "Link (maimai)".to_string()
        } else {
            title
        };

        let artist = song["artist"].as_str().unwrap().to_string();

        let st_lv = if song.contains_key("lev_bas") {
            Some(Difficulty {
                bas: song["lev_bas"].as_str().unwrap().to_string(),
                adv: song["lev_adv"].as_str().unwrap().to_string(),
                exp: song["lev_exp"].as_str().unwrap().to_string(),
                mas: song["lev_mas"].as_str().unwrap().to_string(),
                extra: if song.contains_key("lev_remas") {
                    Some(song["lev_remas"].as_str().unwrap().to_string())
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
                bas: song["dx_lev_bas"].as_str().unwrap().to_string(),
                adv: song["dx_lev_adv"].as_str().unwrap().to_string(),
                exp: song["dx_lev_exp"].as_str().unwrap().to_string(),
                mas: song["dx_lev_mas"].as_str().unwrap().to_string(),
                extra: if song.contains_key("dx_lev_remas") {
                    Some(song["dx_lev_remas"].as_str().unwrap().to_string())
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

        let order = song["sort"].as_str().unwrap().parse::<usize>().unwrap();

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
                order: Some(order),
                category: MaiCategory::Error,
            },
        );
        assert_eq!(r, None);
    }
}

fn set_jp_constants(charts: &mut HashMap<String, MaiInfo>) {
    // Get jp constants
    let file = File::open("data/maimai/jp_lv.csv").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
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
                        eprintln!(
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
                        eprintln!(
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
                        eprintln!(
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
                        eprintln!(
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
                            eprintln!(
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
}

fn set_actual_jp_constants(charts: &mut HashMap<String, MaiInfo>) {
    // Get jp constants from second source.
    let file = File::open("data/maimai/festival_16-09-2022.json").unwrap();
    let songs: serde_json::Value = serde_json::from_reader(&file).unwrap();
    let songs = songs.as_array().unwrap();
    for song in songs {
        let song = song.as_object().unwrap();
        let mut title = song["Song"].as_str().unwrap();
        let version = song["Version added"].as_str().unwrap();
        let mut dx = version.contains("でらっくす")
            || version.contains("スプラッシュ")
            || version.contains("UNiVERSE")
            || version.contains("FESTiVAL");
        if EXCEPTIONS_ST_AFTER_DX.contains(&(title.to_string(), version.to_string())) {
            dx = !dx;
        }
        if (title, version) == ("Link", "maimai PLUS") {
            title = "Link (maimai)";
        }

        for (diff, chart) in song["Charts"].as_array().unwrap().iter().enumerate() {
            let cc = chart["Level Constant"].as_str().unwrap();
            let jp_diff = charts.get_mut(title).unwrap().jp_lv.as_mut().unwrap();
            let dx_or_st_chart = if dx { &mut jp_diff.dx } else { &mut jp_diff.st };
            let mai_diff = dx_or_st_chart.as_mut().unwrap();
            let current_cc = mai_diff.get_constant(diff);
            if current_cc.is_some() && format!("{:.1}", current_cc.unwrap()) != cc {
                eprintln!(
                    "JP constant sources different on song {} {} {} - {:.1} vs {}",
                    title,
                    dx,
                    diff,
                    current_cc.unwrap(),
                    cc
                );
            }
            mai_diff.set_constant(diff, cc.to_string());
        }
    }
}

fn set_intl_difficulty(charts: &mut HashMap<String, MaiInfo>) {
    // Get intl difficulty.
    let jp_and_intl_version_is_different = true;
    if jp_and_intl_version_is_different {
        let file = File::open("data/maimai/in_lv.csv").unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.split('\t').collect::<Vec<_>>();
            assert_eq!(
                line.len(),
                7,
                "Line parse mismatch with original level on {:?}",
                line
            );
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
                        order: None,
                        category: MaiCategory::Error,
                    },
                );
            }
        }
    } else {
        // Same version; copy jp difficulty into intl
        for info in charts.values_mut() {
            if info.jp_lv.is_some() {
                info.intl_lv = info.jp_lv.clone();
            }
        }
    }
}

fn set_song_info(charts: &mut HashMap<String, MaiInfo>) {
    // Get info DB
    let info = fs::read_to_string("data/maimai/maimai-info.txt").unwrap();
    let info = info.trim();
    let s = get_curl(info);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();
    let songs = songs.as_object().unwrap()["songs"].as_array().unwrap();

    for song in songs {
        let song = song.as_object().unwrap();

        let title = song["songId"].as_str().unwrap();
        // Edge case handling for duplicate title
        let title = if title == "Link" {
            "Link (maimai)".to_string()
        } else if title == "Link (2)" {
            "Link".to_string()
        } else {
            title.to_string()
        };

        let exists_in_jp = song["sheets"].as_array().unwrap()[0].as_object().unwrap()["regions"]
            .as_object()
            .unwrap()["jp"]
            .as_bool()
            .unwrap();
        let exists_in_intl = song["sheets"].as_array().unwrap()[0].as_object().unwrap()["regions"]
            .as_object()
            .unwrap()["intl"]
            .as_bool()
            .unwrap();

        if !charts.contains_key(&title) {
            // Is either Utage, deleted, or intl only
            if song.get("category").unwrap() == "宴会場" {
                // Utage
                // TODO
                continue;
            } else {
                // Deleted or intl only
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
                        deleted: !exists_in_intl,
                        order: None,
                        category: mai_get_category(song["category"].as_str().unwrap()),
                    },
                );
            }
        }
        let jp_jacket = song["imageName"].as_str().unwrap().to_string();

        let sheets = song["sheets"].as_array().unwrap();
        let mut st_sheet_data = vec![];
        let mut dx_sheet_data = vec![];
        let mut st_constants = vec![];
        let mut dx_constants = vec![];
        let mut dx_levels = vec![];
        let mut st_levels = vec![];

        let r = charts.get_mut(&title).unwrap();

        for sheet in sheets {
            let sheet = sheet.as_object().unwrap();

            // Get notes info.
            let notes = sheet["noteCounts"].as_object().unwrap();
            // if notes["tap"].is_null() {
            //     break;
            // }
            let designer = sheet["noteDesigner"].as_str().map(|s| s.to_string());
            let sheet_info = MaiSheet {
                designer,
                brk: notes["break"].as_u64().unwrap_or(99999) as usize,
                hold: notes["hold"].as_u64().unwrap_or(99999) as usize,
                slide: notes["slide"].as_u64().unwrap_or(99999) as usize,
                tap: notes["tap"].as_u64().unwrap_or(99999) as usize,
                touch: if notes.contains_key("touch") {
                    if notes["touch"].is_null() {
                        0
                    } else {
                        notes["touch"].as_u64().unwrap_or(99999) as usize
                    }
                } else {
                    0
                },
            };
            let dx_type = sheet["type"].as_str().unwrap();
            if dx_type == "dx" {
                dx_sheet_data.push(sheet_info);
                dx_constants.push(match &sheet["internalLevel"] {
                    serde_json::Value::Null => None,
                    serde_json::Value::String(s) => float_to_constant(s),
                    _ => panic!("Unexpected value for sheet.internalLevel"),
                });
                dx_levels.push(sheet["level"].clone());
            } else if dx_type == "std" {
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

            // Get region info.
            let regions = sheet["regions"].as_object().unwrap();
            let jp_region = regions["jp"].as_bool().unwrap();
            let intl_region = regions["intl"].as_bool().unwrap();

            let diff_idx = diff_to_idx(sheet["difficulty"].as_str().unwrap());

            // We assume Basic~Master has same region availability
            if diff_idx == 0 {
                // Basic (and everything else except Remas)
                if !jp_region && !intl_region {
                    // I don't think there'll be a case where only one of ST/DX chart gets deleted and the other stays.
                    r.deleted = true;
                } else {
                    if !jp_region {
                        if let Some(lv) = r.jp_lv.as_mut() {
                            let lv = if dx_type == "dx" {
                                &mut lv.dx
                            } else if dx_type == "std" {
                                &mut lv.st
                            } else {
                                panic!()
                            };
                            *lv = None;
                        }
                    }
                    if !intl_region {
                        if let Some(lv) = r.intl_lv.as_mut() {
                            let lv = if dx_type == "dx" {
                                &mut lv.dx
                            } else if dx_type == "std" {
                                &mut lv.st
                            } else {
                                panic!()
                            };
                            *lv = None;
                        }
                    }
                    r.deleted = false;
                }
            } else if diff_idx == 4 {
                // Remas
                if !jp_region && !intl_region {
                    // If song is deleted, this is already taken care of on the Basic~Master branch.
                } else {
                    // Sorry for this code, I wish to kms
                    if !jp_region {
                        if let Some(lv) = r.jp_lv.as_mut() {
                            let lv = if dx_type == "dx" {
                                &mut lv.dx
                            } else if dx_type == "std" {
                                &mut lv.st
                            } else {
                                panic!()
                            };
                            if let Some(remas) = lv.as_mut() {
                                remas.extra = None;
                                remas.extra_c = None;
                            }
                        }
                    }
                    if !intl_region {
                        if let Some(lv) = r.intl_lv.as_mut() {
                            let lv = if dx_type == "dx" {
                                &mut lv.dx
                            } else if dx_type == "std" {
                                &mut lv.st
                            } else {
                                panic!()
                            };
                            if let Some(remas) = lv.as_mut() {
                                remas.extra = None;
                                remas.extra_c = None;
                            }
                        }
                    }
                }
            }
        }

        let bpm = song.get("bpm");
        let bpm = if bpm == Some(&serde_json::Value::Null) {
            None
        } else {
            bpm.map(|b| OrderedFloat(b.as_f64().unwrap()))
        };
        let version = if DX_SONGS_WITH_ST.contains(&title) {
            // song.version contains ST info - we need DX info instead
            sheets[0]
                .as_object()
                .unwrap()
                .get("version")
                .map(|s| s.as_str().unwrap().to_string())
        } else {
            song.get("version")
                .map(|s| s.as_str().unwrap_or("N/A").to_string())
        };

        r.jp_jacket = Some(jp_jacket);
        r.bpm = bpm;
        r.dx_sheets = dx_sheet_data;
        r.st_sheets = st_sheet_data;
        r.version = version;
        r.category = mai_get_category(song["category"].as_str().unwrap());

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
                r.artist = artist.as_str().unwrap().to_string();
            }

            if r.deleted || (!exists_in_jp && exists_in_intl) {
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

                if r.deleted {
                    r.jp_lv = Some(MaiDifficulty {
                        dx: dx_diff,
                        st: st_diff,
                    })
                } else if !exists_in_jp && exists_in_intl {
                    r.intl_lv = Some(MaiDifficulty {
                        dx: dx_diff,
                        st: st_diff,
                    })
                }
            }
        }
    }
}

fn set_manual_constants(charts: &mut HashMap<String, MaiInfo>) {
    // Add manual constant info
    let file = File::open("data/maimai/maimai-manual-add.txt").unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let line = line.split('\t').collect::<Vec<_>>();
        assert_eq!(line.len(), 5);
        let title = line[0];
        let chart = charts
            .get_mut(title)
            .unwrap_or_else(|| panic!("{} <- title does not exist", title));
        chart.deleted = false;
        let inner = if line[3] == "JP" {
            chart.jp_lv.as_mut()
        } else if line[3] == "IN" {
            if chart.intl_lv.is_none() {
                chart.intl_lv = Some(MaiDifficulty::default());
            }
            chart.intl_lv.as_mut()
        } else {
            todo!()
        }
        .unwrap();
        let inner = if line[1] == "DX" {
            if inner.dx.is_none() {
                inner.dx = Some(Difficulty::default());
            }
            inner.dx.as_mut()
        } else if line[1] == "ST" {
            if inner.st.is_none() {
                inner.st = Some(Difficulty::default());
            }
            inner.st.as_mut()
        } else {
            panic!()
        }
        .unwrap_or_else(|| panic!("Panic on song {}", title));
        if line[4].contains('.') {
            // Add constant
            let cst = float_to_constant(line[4]);
            if line[2] == "EXP" {
                assert!(inner.exp_c.is_none() || inner.exp_c == cst);
                if inner.exp_c == cst {
                    eprintln!("{:?} exists on server", line);
                } else {
                    // eprintln!("{:?} enter success", line);
                }
                inner.exp_c = cst;
            } else if line[2] == "MAS" {
                if inner.mas_c.is_some() && inner.mas_c != cst {
                    eprintln!(
                        "Constant mismatch on manual constant line {:?}\n{:?}, {:?}",
                        line, inner.mas_c, cst
                    );
                } else if inner.mas_c == cst {
                    eprintln!("{:?} exists on server", line);
                } else {
                    // eprintln!("{:?} enter success", line);
                }
                inner.mas_c = cst;
            } else if line[2] == "REM" {
                if inner.extra_c.is_some() && inner.extra_c != cst {
                    eprintln!(
                        "Constant mismatch on manual constant line {:?}\n{:?}, {:?}",
                        line, inner.extra_c, cst
                    );
                } else if inner.extra_c == cst {
                    eprintln!("{:?} exists on server", line);
                } else {
                    // eprintln!("{:?} enter success", line);
                }
                inner.extra_c = cst;
            } else {
                panic!()
            }
        } else {
            // Add level
            let diff_idx = diff_to_idx(line[2]);
            let diff_str = inner.lv(diff_idx);
            assert_eq!(diff_str, "?");
            inner.set_lv(diff_idx, line[4].to_string());
        }
    }
}

pub fn set_mai_charts() -> Result<HashMap<String, MaiInfo>, Error> {
    let mut charts = HashMap::new();

    set_jp_difficulty(&mut charts);
    set_jp_constants(&mut charts);
    // set_actual_jp_constants(&mut charts);
    set_intl_difficulty(&mut charts);
    set_song_info(&mut charts);
    set_manual_constants(&mut charts);

    Ok(charts)
}

fn mai_chart_embed(title: String, ctx: &Context<'_>) -> Result<(String, Option<String>), Error> {
    let song = ctx.data().mai_charts.get(&title);
    let song = song.unwrap();

    let mut embed =
        String::from("Chart info legend:\n**Total notes** / Tap / Hold / Slide / Touch / Break");

    let squares = ["green", "yellow", "red", "purple", "white_large"];

    if !song.dx_sheets.is_empty() {
        let mut dx_str = String::from("**DX Chart Info:**");
        for (idx, sheet) in song.dx_sheets.iter().enumerate() {
            let lvs = song.jp_lv.as_ref().unwrap().dx.as_ref().unwrap();
            dx_str.push_str(&format!(
                "\n:{}_square: Lv.{}{}  Designer: {}",
                squares[idx],
                lvs.lv(idx),
                constant_to_string(lvs.get_constant(idx)),
                sheet.designer.as_ref().unwrap_or(&"-".to_string())
            ));
            let total = sheet.brk + sheet.tap + sheet.hold + sheet.slide + sheet.brk;
            if total >= 99999 {
                continue;
            }
            dx_str.push_str(&format!(
                "\n**{}** / {} / {} / {} / {} / {}",
                total, sheet.tap, sheet.hold, sheet.slide, sheet.touch, sheet.brk
            ));
        }
        embed.push_str("\n\n");
        embed.push_str(&dx_str);
    }

    if !song.st_sheets.is_empty() {
        let mut st_str = String::from("**ST Chart Info:**");
        for (idx, sheet) in song.st_sheets.iter().enumerate() {
            let lvs = song.jp_lv.as_ref().unwrap().st.as_ref().unwrap();
            st_str.push_str(&format!(
                "\n:{}_square: Lv.{}{}  Designer: {}",
                squares[idx],
                lvs.lv(idx),
                constant_to_string(lvs.get_constant(idx)),
                sheet.designer.as_ref().unwrap_or(&"-".to_string())
            ));
            let total = sheet.brk + sheet.tap + sheet.hold + sheet.slide + sheet.brk;
            if total >= 99999 {
                continue;
            }
            st_str.push_str(&format!(
                "\n**{}** / {} / {} / {} / {} / {}",
                total, sheet.tap, sheet.hold, sheet.slide, sheet.touch, sheet.brk
            ));
        }
        embed.push_str("\n\n");
        embed.push_str(&st_str);
    }

    Ok((embed, song.jp_jacket.clone()))
}

/// Get info about song charts in maimai
#[poise::command(slash_command, prefix_command, rename = "mai-chart")]
pub async fn mai_chart(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Selector\", \"bbb\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    let aliases = &ctx.data().mai_aliases;
    let actual_title = get_title(
        &title,
        aliases,
        ctx.guild_id()
            .unwrap_or(poise::serenity_prelude::GuildId(0)),
    );
    if actual_title.is_none() {
        let mut log = ctx.data().alias_log.lock().await;
        writeln!(log, "{}\tmaimai", title)?;
        log.sync_all()?;
        drop(log);
        let closest = get_closest_title(
            &title,
            aliases,
            ctx.guild_id()
                .unwrap_or(poise::serenity_prelude::GuildId(0)),
        );
        let reply = format!(
            "I couldn't find the results for **{}**;
Did you mean **{}** (for **{}**)?
(P.S. You can also use the `/add-alias` command to add this alias to the bot.)",
            title, closest.0, closest.1
        );
        let sent = ctx
            .send(|f| {
                let mut f = f.ephemeral(true).content(reply);
                if let Context::Application(_) = ctx {
                    f = f.components(|c| {
                        let mut button = CreateButton::default();
                        button.custom_id(closest.0);
                        button.label(format!("Yes (times out after {} seconds)", 10));
                        let mut ar = CreateActionRow::default();
                        ar.add_button(button);
                        c.set_action_row(ar)
                    })
                }
                f
            })
            .await?;
        if let ReplyHandle::Unknown { interaction, http } = sent {
            if let Context::Application(poise_ctx) = ctx {
                let serenity_ctx = poise_ctx.discord;
                let m = interaction.get_interaction_response(http).await.unwrap();
                let mci = match m
                    .await_component_interaction(serenity_ctx)
                    .timeout(Duration::from_secs(10))
                    .await
                {
                    Some(ci) => ci,
                    None => {
                        // ctx.send(|f| f.ephemeral(true).content("Timed out"))
                        //     .await
                        //     .unwrap();
                        return Ok(());
                    }
                };
                let actual_title = get_title(
                    &mci.data.custom_id,
                    aliases,
                    ctx.guild_id()
                        .unwrap_or(poise::serenity_prelude::GuildId(0)),
                )
                .unwrap();
                mci.create_interaction_response(&http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| {
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            d.ephemeral(false)
                                .content(format!("Query by <@{}>", ctx.author().id))
                                .embed(|f| {
                                    let (description, jacket) =
                                        mai_chart_embed(actual_title.to_string(), &ctx).unwrap();

                                    let mut f = f
                                        .title(mai_duplicate_alias_to_title(&actual_title))
                                        .description(description)
                                        .color(Color::from_rgb(0, 255, 255));
                                    if let Some(jacket) = jacket {
                                        f = f.thumbnail(format!(
                                            "{}{}",
                                            ctx.data().mai_jacket_prefix,
                                            jacket
                                        ));
                                    }

                                    f
                                })
                        })
                })
                .await?;
            }
        }
        return Ok(());
    }

    let title = actual_title.unwrap();
    let (description, jacket) = mai_chart_embed(title.clone(), &ctx)?;

    ctx.send(|f| {
        f.embed(|f| {
            let mut f = f
                .title(mai_duplicate_alias_to_title(&title).replace('*', "\\*"))
                .description(description)
                .color(Color::from_rgb(0, 255, 255));
            if let Some(jacket) = jacket {
                f = f.thumbnail(format!("{}{}", ctx.data().mai_jacket_prefix, jacket));
            }

            f
        })
    })
    .await?;
    Ok(())
}
