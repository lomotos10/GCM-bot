use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    time::Duration,
};

use lazy_static::lazy_static;
use poise::{
    serenity_prelude::{
        self as serenity, model::interactions::InteractionResponseType, AttachmentType,
        CreateActionRow, CreateButton,
    },
    ReplyHandle,
};

use crate::utils::*;
use gcm_macro::{info_template, jacket_template};

lazy_static! {
    static ref CHUNI_INFO_REPLACEMENT: HashMap<String, String> = {
        [
            ("Reach for the Stars", "Reach For The Stars"),
            ("まっすぐ→→→ストリーム!", "まっすぐ→→→ストリーム！")
            // ("セイクリッド ルイン", "セイクリッド　ルイン"),
            // (
            //     "Bad Apple!! feat.nomico(REDALiCE Remix)",
            //     "Bad Apple!! feat.nomico (REDALiCE Remix)",
            // ),
            // (
            //     "Satellite System ft. Diana Chiaki",
            //     "Satellite System ft.Diana Chiaki",
            // ),
            // ("妖々跋扈～Who done it!!!", "妖々跋扈　～ Who done it！！！"),
            // ("DAZZLING♡SEAZON", "DAZZLING♡SEASON"),
            // ("The EmpErroR", "the EmpErroR"),
            // ("SQUAD-phvntom-", "SQUAD-Phvntom-"),
            // ("ピアノ協奏曲第1番\"蠍火\"", "ピアノ協奏曲第１番”蠍火”"),
            // ("Iudicium \"Apocalypsis Mix\"", "Iudicium “Apocalypsis Mix”"),
            // (
            //     "ナイト・オブ・ナイツ(かめりあ's \"ワンス・アポン・ア・ナイト\"Remix)",
            //     "ナイト・オブ・ナイツ (かめりあ’s“ワンス・アポン・ア・ナイト”Remix)",
            // ),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>()
    };
}

fn get_chuni_embed(title: String, ctx: Context<'_>) -> Result<(String, Option<String>), Error> {
    let song = ctx.data().chuni_charts.get(&title);

    let song = song.unwrap();

    let mut description = format!("**Artist:** {}", song.artist.replace('*', "\\*"));
    description = if let Some(version) = song.version.as_ref() {
        format!("{}\n**Version:** {}", description, version)
    } else {
        description
    };
    description = if let Some(bpm) = song.bpm {
        format!("{}\n**BPM:** {}", description, bpm)
    } else {
        description
    };

    let in_lv = &song.intl_lv;
    let jp_lv = &song.jp_lv;

    let jp_txt = if let Some(jp_lv) = jp_lv {
        level_description(jp_lv, &title)
    } else {
        "**Unreleased**".to_string()
    };
    let in_txt = if let Some(in_lv) = in_lv {
        level_description(in_lv, &title)
    } else {
        "**Unreleased**".to_string()
    };
    if in_txt == jp_txt {
        description = format!(
            "{}

**Level:**
:flag_jp::globe_with_meridians: {}",
            description, jp_txt
        );
    } else {
        description = format!(
            "{}

**Level:**
:flag_jp: {}
:globe_with_meridians: {}",
            description, jp_txt, in_txt
        );
    }
    //     description = format!(
    //         "{}

    // **Level:** {}",
    //         description, jp_txt
    //     );

    Ok((description, song.jp_jacket.clone()))
}

fn chuni_duplicate_alias_to_title(title: &String) -> String {
    title.to_string()
}

/// Get CHUNITHM song info
#[poise::command(slash_command, prefix_command, rename = "chuni-info")]
pub async fn chuni_info(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Xevel\", \"Ikazuchi\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    info_template!(
        "chuni",
        "255, 255, 0",
        "\"https://new.chunithm-net.com/chuni-mobile/html/mobile/img/\""
    );
    Ok(())
}

fn level_description(lv: &Difficulty, title: &str) -> String {
    let title = urlencoding::encode(title);
    format!(
        "[B](https://www.youtube.com/results?search_query=CHUNITHM+{}+BASIC) **{}**{} / [A](https://www.youtube.com/results?search_query=CHUNITHM+{}+ADVANCED) **{}**{} / [E](https://www.youtube.com/results?search_query=CHUNITHM+{}+EXPERT) **{}**{} / [M](https://www.youtube.com/results?search_query=CHUNITHM+{}+MASTER) **{}**{}{}",
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
            format!(" / [U](https://www.youtube.com/results?search_query=CHUNITHM+{}+ULTIMA) **{}**{}", title, rem, constant_to_string(lv.extra_c))
        } else {
            "".to_string()
        }
    )
}

/// Get CHUNITHM song jacket
#[poise::command(slash_command, prefix_command, rename = "chuni-jacket")]
pub async fn chuni_jacket(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"Xevel\", \"Ikazuchi\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    jacket_template!(
        "chuni",
        "\"https://new.chunithm-net.com/chuni-mobile/html/mobile/img/\""
    );
    Ok(())
}

pub fn set_chuni_charts() -> Result<HashMap<String, ChuniInfo>, Error> {
    let mut charts = HashMap::new();

    // Get JP difficulty.
    let url = fs::read_to_string("data/chuni/chuni-url.txt")?;
    let url = url.trim();
    let s = get_curl(url);

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
        let artist = serdest_to_string(song.get("artist").unwrap());
        let jacket = serdest_to_string(song.get("image").unwrap());

        let jp_lv = Difficulty {
            bas: serdest_to_string(song.get("lev_bas").unwrap()),
            adv: serdest_to_string(song.get("lev_adv").unwrap()),
            exp: serdest_to_string(song.get("lev_exp").unwrap()),
            mas: serdest_to_string(song.get("lev_mas").unwrap()),
            extra: if song.contains_key("lev_ult") {
                let ult = song.get("lev_ult").unwrap();
                if ult == "" {
                    None
                } else {
                    Some(serdest_to_string(ult))
                }
            } else {
                None
            },
            ..Default::default()
        };

        if charts.get(&title).is_some() {
            // WORLD'S END items have empty level items
            assert_eq!(serdest_to_string(song.get("lev_bas").unwrap()), "");
            // TODO: implement WORLD'S END
        } else if serdest_to_string(song.get("lev_bas").unwrap()).is_empty() {
            // TODO: implement WORLD'S END
        } else {
            charts.insert(
                title.clone(),
                ChuniInfo {
                    jp_lv: Some(jp_lv),
                    jp_jacket: Some(jacket),
                    title,
                    artist,
                    // version: None,
                    ..Default::default()
                },
            );
        }
    }

    // Get intl difficulty.
    let url = fs::read_to_string("data/chuni/chuni-intl.txt")?;
    let url = url.trim();
    let s = get_curl(url);
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
        if song.get("lev_bas") != None {
            let intl_lv = Difficulty {
                bas: serdest_to_string(song.get("lev_bas").unwrap()),
                adv: serdest_to_string(song.get("lev_adv").unwrap()),
                exp: serdest_to_string(song.get("lev_exp").unwrap()),
                mas: serdest_to_string(song.get("lev_mas").unwrap()),
                extra: if song.contains_key("lev_ul") {
                    let ult = song.get("lev_ul").unwrap();
                    if ult == "" {
                        None
                    } else {
                        Some(serdest_to_string(ult))
                    }
                } else {
                    None
                },
                ..Default::default()
            };

            if let Some(data) = charts.get_mut(&title) {
                if let Some(intl_lv) = &mut (*data).intl_lv {
                    if let Some(ult) = song.get("lev_ul") {
                        assert_eq!((*intl_lv).extra, None);
                        (*intl_lv).extra = Some(serdest_to_string(ult));
                    }
                } else {
                    (*data).intl_lv = Some(intl_lv);
                }
            }
        } else {
            // WORLD'S END item; TODO implement
        }
    }

    // Get constants
    let constants = fs::read_to_string("data/chuni/chuni-info.txt")?;
    let url = constants.trim();
    let s = get_curl(url);
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();
    let songs = songs.as_object().unwrap()["songs"].as_array().unwrap();
    for song in songs {
        let song = song.as_object().unwrap();

        let meta = song;
        let title = meta["title"].as_str().unwrap().to_string();

        // skip WE
        if meta["category"] == "WORLD'S END" {
            continue;
        }

        let title = CHUNI_INFO_REPLACEMENT.get(&title).unwrap_or(&title);
        let bpm = meta["bpm"].as_u64();
        let chart = charts.get_mut(title);
        let chart = if let Some(c) = chart {
            c
        } else {
            // deleted song; todo
            continue;
        };
        chart.bpm = bpm.map(|i| i as usize);
        if let Some(version) = meta["version"].as_str() {
            chart.version = Some(version.to_string());
        }

        let diffs = song["sheets"].as_array().unwrap();
        let difficulty = chart.jp_lv.as_mut().unwrap();
        for data in diffs.iter() {
            let data = data.as_object().unwrap();
            let diff_c = diff_to_idx(data["difficulty"].as_str().unwrap());
            if data.get("internalLevelValue") == None {
                continue;
            }
            let c = if let Some(c) = data["internalLevelValue"].as_f64() {
                c
            } else {
                continue;
            };
            if c != 0.0 {
                difficulty.set_constant(diff_c, c.to_string());
            }
        }
    }

    // Add intl del info
    let file = File::open("data/chuni/chuni-intl-del.txt")?;
    let intl_del = BufReader::new(file).lines().flatten().collect::<Vec<_>>();

    // Add intl level info
    let file = File::open("data/chuni/chuni-new-plus-lv.csv")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let line = line.split('\t').collect::<Vec<_>>();
        assert_eq!(line.len(), 4);
        let title = line[0];
        if intl_del.contains(&title.to_string()) {
            continue;
        }
        let chart = charts.get_mut(title).unwrap();
        if chart.intl_lv.is_none() {
            chart.intl_lv = Some(Difficulty::default());
        }
        let inner = chart.intl_lv.as_mut().unwrap();
        // Add level
        let diff_idx = diff_to_idx(line[2]);
        // let diff_str = inner.lv(diff_idx);
        // assert_eq!(diff_str, "?");
        inner.set_lv(diff_idx, line[3].to_string());
    }

    // Add intl constant info
    let file = File::open("data/chuni/chuni-new-plus-cst.csv")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let line = line.split('\t').collect::<Vec<_>>();
        assert_eq!(line.len(), 4);
        let title = line[0];
        if intl_del.contains(&title.to_string()) {
            continue;
        }
        let chart = charts.get_mut(title).unwrap();
        let inner = chart.intl_lv.as_mut().unwrap();
        let cst = float_to_constant(line[3]);
        let diff_idx = diff_to_idx(line[2]);
        inner.set_constant(diff_idx, cst.unwrap().to_string());
    }

    Ok(charts)
}
