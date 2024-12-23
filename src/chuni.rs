use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    sync::Arc,
};

use eyre::{bail, ensure};
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::utils::*;

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

fn get_chuni_embed(title: String, ctx: &Context<'_>) -> eyre::Result<(String, Option<String>)> {
    let song = ctx.data().chuni_charts.get(&title).unwrap();

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

    if song.deleted {
        description = format!(
            "{}\n\n**Level:**\n{}",
            description,
            level_description(song.jp_lv.as_ref().unwrap(), &title)
        )
    } else {
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
    }

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
    info_template(
        ctx,
        title,
        Game::Chunithm,
        Arc::new(get_chuni_embed),
        (255, 255, 0),
        Arc::new(chuni_duplicate_alias_to_title),
    )
    .await?;
    Ok(())
}

fn level_description(lv: &Difficulty, title: &str) -> String {
    let title = title.replace(" -", " ");
    let title = title.strip_prefix('-').unwrap_or(&title);
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
    jacket_template(ctx, title, Game::Chunithm).await?;
    Ok(())
}

fn set_jp_difficulty(charts: &mut HashMap<String, ChuniInfo>) -> eyre::Result<()> {
    // Get JP difficulty.
    let url = fs::read_to_string("data/chuni/chuni-url.txt")?;
    let url = url.trim();
    let s = get_curl(url);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();

    let songs = songs.as_array().unwrap();

    for song in songs {
        let song = song.as_object().unwrap();

        let title = song["title"].as_str().unwrap().to_string();
        let artist = song["artist"].as_str().unwrap().to_string();
        // let jacket = song["image"].as_str().unwrap().to_string();

        let jp_lv = Difficulty {
            bas: song["lev_bas"].as_str().unwrap().to_string(),
            adv: song["lev_adv"].as_str().unwrap().to_string(),
            exp: song["lev_exp"].as_str().unwrap().to_string(),
            mas: song["lev_mas"].as_str().unwrap().to_string(),
            extra: if song.contains_key("lev_ult") {
                let ult = song.get("lev_ult").unwrap();
                if ult == "" {
                    None
                } else {
                    Some(ult.as_str().unwrap().to_string())
                }
            } else {
                None
            },
            ..Default::default()
        };

        if charts.get(&title).is_some() {
            // WORLD'S END items have empty level items
            assert_eq!(song["lev_bas"].as_str().unwrap().to_string(), "");
            // TODO: implement WORLD'S END
        } else if song["lev_bas"].as_str().unwrap().to_string().is_empty() {
            // TODO: implement WORLD'S END
        } else {
            charts.insert(
                title.clone(),
                ChuniInfo {
                    jp_lv: Some(jp_lv),
                    // jp_jacket: Some(jacket),
                    title,
                    artist,
                    // version: None,
                    ..Default::default()
                },
            );
        }
    }
    Ok(())
}

fn set_intl_difficulty(charts: &mut HashMap<String, ChuniInfo>) -> eyre::Result<()> {
    // Get intl difficulty.
    let url = fs::read_to_string("data/chuni/chuni-intl.txt")?;
    let url = url.trim();
    let s = get_curl(url);
    let songs: serde_json::Value = serde_json::from_str(&s)?;

    let songs = songs.as_array().unwrap();

    for song in songs {
        let song = song.as_object().unwrap();

        let title = song["title"].as_str().unwrap().to_string();
        if song.get("lev_bas").is_some() {
            let intl_lv = Difficulty {
                bas: song["lev_bas"].as_str().unwrap().to_string(),
                adv: song["lev_adv"].as_str().unwrap().to_string(),
                exp: song["lev_exp"].as_str().unwrap().to_string(),
                mas: song["lev_mas"].as_str().unwrap().to_string(),
                extra: if song.contains_key("lev_ult") {
                    let ult = song.get("lev_ult").unwrap();
                    if ult == "" {
                        None
                    } else {
                        Some(ult.as_str().unwrap().to_string())
                    }
                } else {
                    None
                },
                ..Default::default()
            };

            if let Some(data) = charts.get_mut(&title) {
                if let Some(intl_lv) = &mut data.intl_lv {
                    if let Some(ult) = song.get("lev_ul") {
                        assert_eq!(intl_lv.extra, None);
                        intl_lv.extra = Some(ult.as_str().unwrap().to_string());
                    }
                } else {
                    data.intl_lv = Some(intl_lv);
                }
            }
        } else {
            // WORLD'S END item; TODO implement
        }
    }
    Ok(())
}

fn set_constants(
    charts: &mut HashMap<String, ChuniInfo>,
    jp_and_intl_version_is_different: bool,
) -> eyre::Result<()> {
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
        let jacket = meta["imageName"].as_str().unwrap();
        let chart = charts.get_mut(title);
        let chart = if let Some(c) = chart {
            c
        } else {
            // deleted song
            charts.insert(
                title.to_string(),
                ChuniInfo {
                    jp_lv: None,
                    // jp_jacket: Some(jacket),
                    title: title.to_string(),
                    deleted: true,
                    ..Default::default()
                },
            );
            charts.get_mut(title).unwrap()
        };
        chart.bpm = bpm.map(|i| i as usize);
        chart.jp_jacket = Some(jacket.to_string());
        if let Some(version) = meta["version"].as_str() {
            chart.version = Some(version.to_string());
        }
        chart.category = chuni_get_category(meta["category"].as_str().unwrap());

        let diffs = song["sheets"].as_array().unwrap();
        // For deleted songs, add difficulty info.
        if chart.deleted {
            let mut difficulty = Difficulty::default();
            for data in diffs.iter() {
                let data = data.as_object().unwrap();
                let diff_c = diff_to_idx(data["difficulty"].as_str().unwrap());
                let lv = data["level"].as_str().unwrap();
                difficulty.set_lv(diff_c, lv.to_string());
            }
            chart.jp_lv = Some(difficulty);
            chart.artist = meta["artist"].as_str().unwrap().to_string();
        }
        let difficulty = chart.jp_lv.as_mut().unwrap();
        for data in diffs.iter() {
            let data = data.as_object().unwrap();
            let diff_c = diff_to_idx(data["difficulty"].as_str().unwrap());
            if data.get("internalLevelValue").is_none() {
                continue;
            }
            let Some(c) = data["internalLevelValue"].as_f64() else {
                continue;
            };
            if c != 0.0 {
                difficulty.set_constant(diff_c, c.to_string());
                // Set intl cc too, if song isn't deleted.
                if !jp_and_intl_version_is_different && chart.intl_lv.is_some() {
                    chart
                        .intl_lv
                        .as_mut()
                        .unwrap()
                        .set_constant(diff_c, c.to_string());
                }
            }

            // Set difficulty by region.
            let regions = data["regions"].as_object().unwrap();
            // let jp_region = regions["jp"].as_bool().unwrap();
            let intl_region = regions["intl"].as_bool().unwrap();
            if !intl_region {
                if diff_c < 4 {
                    // song doesn't exist at all in intl
                    chart.intl_lv = None;
                    continue;
                } else {
                    // ultima doesn't exist
                    assert_eq!(diff_c, 4);
                    // If intl_lv wasn't deleted by an earlier iteration because other levels exist..
                    if let Some(intl) = chart.intl_lv.as_mut() {
                        intl.extra = None;
                        intl.extra_c = None;
                    }
                }
            }
            // Assert that if song exists in intl according to arcade-songs, intl level data exists.
            if intl_region && diff_c == 0 {
                // assert!(chart.intl_lv.is_some(), "{:#?}", song);
            }
        }
    }

    Ok(())
}

fn set_intl_info(
    charts: &mut HashMap<String, ChuniInfo>,
    jp_and_intl_version_is_different: bool,
) -> eyre::Result<()> {
    // Get constants
    let s = fs::read_to_string("data/chuni/chuni-info-luminous.json")?;
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
        let chart = charts.get_mut(title).unwrap();

        let diffs = song["sheets"].as_array().unwrap();

        if !diffs[0].as_object().unwrap()["regions"]
            .as_object()
            .unwrap()["intl"]
            .as_bool()
            .unwrap()
        {
            continue;
        }
        if chart.intl_lv.is_none() {
            chart.intl_lv = Some(Difficulty::default());
        }
        let difficulty = chart.intl_lv.as_mut().unwrap();
        for data in diffs.iter() {
            let data = data.as_object().unwrap();
            let diff_c = diff_to_idx(data["difficulty"].as_str().unwrap());
            if data.get("internalLevelValue").is_none() {
                continue;
            }
            let Some(c) = data["internalLevelValue"].as_f64() else {
                continue;
            };
            if c != 0.0 && jp_and_intl_version_is_different {
                difficulty.set_lv(diff_c, float_to_chuni_level(&c.to_string()));
                difficulty.set_constant(diff_c, c.to_string());
            }
        }
    }

    Ok(())
}

fn _set_intl_info_csv(
    charts: &mut HashMap<String, ChuniInfo>,
    jp_and_intl_version_is_different: bool,
) -> eyre::Result<()> {
    if jp_and_intl_version_is_different {
        // Add intl level info
        let file = File::open("data/chuni/chuni-sun-lv.csv")?;
        let lines = BufReader::new(file).lines();
        for line in lines.flatten() {
            let line = line.split('\t').collect_vec();
            assert_eq!(line.len(), 4);
            let title = line[0];
            let Some(chart) = charts.get_mut(title) else {
                continue;
            };
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
        let file = File::open("data/chuni/chuni-sun-cst.csv")?;
        let lines = BufReader::new(file).lines();
        for line in lines.flatten() {
            let line = line.split('\t').collect_vec();
            assert_eq!(line.len(), 4);
            let title = line[0];
            let Some(chart) = charts.get_mut(title) else {
                continue;
            };
            let inner = chart.intl_lv.as_mut().unwrap();
            let cst = float_to_constant(line[3]);
            let diff_idx = diff_to_idx(line[2]);
            inner.set_constant(diff_idx, cst.unwrap().to_string());
        }
    } else {
        // Same version; copy jp difficulty into intl
        for info in charts.values_mut() {
            if info.jp_lv.is_some() {
                info.intl_lv = info.jp_lv.clone();
            }
        }
    }

    // Levels up until 9+ have only one constant - manually assign
    for song in charts {
        for diff in &mut song.1.intl_lv {
            for i in 0..5 {
                let lv = diff.lv(i);
                if lv != "?" {
                    let cc = lv.chars().filter(|c| c.is_numeric()).collect::<String>();
                    let mut cc = cc.parse::<ordered_float::OrderedFloat<f32>>().unwrap();
                    if cc >= 10.0.into() {
                        continue;
                    }
                    if lv.contains('+') {
                        cc += 0.5;
                    }
                    let actual_cc = diff.get_constant(i);
                    ensure!(
                        actual_cc.is_none() || actual_cc == Some(cc),
                        "failed to insert cc {} into chart {} with cc {:?}",
                        cc,
                        song.0,
                        actual_cc
                    );
                    diff.set_constant(i, cc.to_string());
                }
            }
        }
    }

    Ok(())
}

fn set_manual_constants(charts: &mut HashMap<String, ChuniInfo>) -> eyre::Result<()> {
    // Add manual constant info
    let file = File::open("data/chuni/chuni-manual-add.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let line = line.split('\t').collect_vec();
        assert_eq!(line.len(), 4);
        let (title, diff, region, cc) = (line[0], line[1], line[2], line[3]);
        let diff_idx = diff_to_idx(diff);

        let Some(chart) = charts.get_mut(title) else {
            bail!("{} <- title does not exist", title)
        };
        chart.deleted = false;
        let inner = if region == "JP" {
            chart.jp_lv.as_mut()
        } else if region == "IN" {
            if chart.intl_lv.is_none() {
                chart.intl_lv = Some(Difficulty::default());
            }
            chart.intl_lv.as_mut()
        } else {
            todo!()
        }
        .unwrap();
        if cc.contains('.') {
            // Add constant
            let cst = float_to_constant(cc);
            let cc = inner.get_constant(diff_idx);
            if cc.is_some() && cc != cst {
                eprintln!(
                    "Constant mismatch on manual constant line {:?}\n{:?}, {:?}",
                    line, cc, cst
                );
            } else if inner.mas_c == cst {
                eprintln!("{:?} exists on server", line);
            } else {
                // eprintln!("{:?} enter success", line);
            }
            inner.set_constant(diff_idx, cst.unwrap().to_string());
        } else {
            // Add level
            let diff_str = inner.lv(diff_idx);
            assert_eq!(diff_str, "?");
            inner.set_lv(diff_idx, cc.to_string());
        }
    }
    Ok(())
}

pub fn set_chuni_charts() -> Result<HashMap<String, ChuniInfo>, Error> {
    let mut charts = HashMap::new();
    let jp_and_intl_version_is_different = false;

    set_jp_difficulty(&mut charts)?;
    set_intl_difficulty(&mut charts)?;
    set_constants(&mut charts, jp_and_intl_version_is_different)?;
    set_intl_info(&mut charts, jp_and_intl_version_is_different)?;
    set_manual_constants(&mut charts)?;

    Ok(charts)
}
