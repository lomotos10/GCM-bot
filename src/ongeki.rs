use std::{
    collections::HashMap,
    fs::{self},
    io::Write,
    time::Duration,
};

use poise::{
    serenity_prelude::{
        self as serenity, AttachmentType, CreateActionRow, CreateButton, InteractionResponseType,
    },
    ReplyHandle,
};

use crate::utils::*;
use gcm_macro::{info_template, jacket_template};

fn get_ongeki_embed(title: String, ctx: Context<'_>) -> Result<(String, Option<String>), Error> {
    let song = ctx.data().ongeki_charts.get(&title);

    let song = song.unwrap();
    let date = song.date;
    let version = if date >= 20220303 {
        "bright MEMORY"
    } else if date >= 20211021 {
        "bright"
    } else if date >= 20210331 {
        "R.E.D. PLUS"
    } else if date >= 20200930 {
        "R.E.D."
    } else if date >= 20200220 {
        "SUMMER PLUS"
    } else if date >= 20190822 {
        "SUMMER"
    } else if date >= 20190207 {
        "PLUS"
    } else if date >= 20180726 {
        "オンゲキ"
    } else {
        unreachable!()
    };

    let description = format!(
        "**Artist:** {}
**Version**: {}
**VS**: {}

**Level:** {}",
        song.artist.replace('*', "\\*"),
        version,
        song.character,
        level_description(song.lv.as_ref().unwrap(), &title)
    );

    Ok((description, song.jp_jacket.clone()))
}

fn ongeki_duplicate_alias_to_title(title: &String) -> String {
    if title == "Singularity (Arcaea)" || title == "Singularity (MJ)" {
        "Singularity".to_string()
    } else if title == "Hand in Hand (deleted)" {
        "Hand in Hand".to_string()
    } else if title == "Perfect Shining!! (Location test)" {
        "Perfect Shining!!".to_string()
    } else {
        title.to_string()
    }
}

/// Get Ongeki song info
#[poise::command(slash_command, prefix_command, rename = "ongeki-info")]
pub async fn ongeki_info(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"w4\", \"Apollo\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    info_template!(
        "ongeki",
        "255, 127, 255",
        "\"https://ongeki-net.com/ongeki-mobile/img/music/\""
    );
    Ok(())
}

fn level_description(lv: &Difficulty, title: &str) -> String {
    let title = urlencoding::encode(title);
    format!(
        // "BAS **{}{}**/ADV **{}{}**/EXP **{}{}**/MAS **{}{}**{}",
        "[B](https://www.youtube.com/results?search_query=オンゲキ+{}+BASIC) **{}**{} / [A](https://www.youtube.com/results?search_query=オンゲキ+{}+ADVANCED) **{}**{} / [E](https://www.youtube.com/results?search_query=オンゲキ+{}+EXPERT) **{}**{} / [M](https://www.youtube.com/results?search_query=オンゲキ+{}+MASTER) **{}**{}{}",
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
            format!(" / [L](https://www.youtube.com/results?search_query=オンゲキ+{}+LUNATIC) **{}**{}", title, rem, constant_to_string(lv.extra_c))
        } else {
            "".to_string()
        }
    )
}

/// Get Ongeki song jacket
#[poise::command(slash_command, prefix_command, rename = "ongeki-jacket")]
pub async fn ongeki_jacket(
    ctx: Context<'_>,
    #[description = "Song title e.g. \"w4\", \"Apollo\", etc. You don't have to be exact; try things out!"]
    #[rest]
    title: String,
) -> Result<(), Error> {
    jacket_template!(
        "ongeki",
        "\"https://ongeki-net.com/ongeki-mobile/img/music/\""
    );
    Ok(())
}

pub fn set_ongeki_charts() -> Result<HashMap<String, OngekiInfo>, Error> {
    let mut charts: HashMap<String, OngekiInfo> = HashMap::new();

    // Get JP difficulty.
    let url = fs::read_to_string("data/ongeki-url.txt")?;
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

        let mut title = serdest_to_string(song.get("title").unwrap());
        let artist = serdest_to_string(song.get("artist").unwrap());
        let jacket = serdest_to_string(song.get("image_url").unwrap());
        let date = serdest_to_string(song.get("date").unwrap());
        let character = serdest_to_string(song.get("character").unwrap());
        let category = serdest_to_string(song.get("category").unwrap());
        let lv = Difficulty {
            bas: serdest_to_string(song.get("lev_bas").unwrap()),
            adv: serdest_to_string(song.get("lev_adv").unwrap()),
            exp: serdest_to_string(song.get("lev_exc").unwrap()),
            mas: serdest_to_string(song.get("lev_mas").unwrap()),
            extra: if song.contains_key("lev_lnt") {
                let lnt = song.get("lev_lnt").unwrap();
                if lnt == "" {
                    None
                } else {
                    Some(serdest_to_string(lnt))
                }
            } else {
                None
            },
            ..Default::default()
        };
        // Get duplicate title aliases
        if title == "Singularity" {
            if date == "20201217" {
                title = "Singularity (Arcaea)".to_string();
            } else if date == "20210401" {
                title = "Singularity (MJ)".to_string();
            }
        } else if title == "Perfect Shining!!" && date == "20220804" {
            title = "Perfect Shining!! (Location test)".to_string();
        }

        let date = date.parse::<usize>().unwrap();
        assert!(date >= 20180726);

        if charts.get(&title).is_some() {
            // LUNATIC items have empty level items
            assert_eq!(serdest_to_string(song.get("lev_bas").unwrap()), "");
            let diff = (*charts.get_mut(&title).unwrap()).lv.as_mut().unwrap();
            (*diff).extra = Some(serdest_to_string(song.get("lev_lnt").unwrap()))
        } else {
            charts.insert(
                title.clone(),
                OngekiInfo {
                    lv: Some(lv),
                    jp_jacket: Some(jacket),
                    title,
                    artist,
                    date,
                    character,
                    category,
                },
            );
        }
    }

    // Get constants
    let url = fs::read_to_string("data/ongeki-info.txt")?;
    let url = url.trim();
    let s = get_curl(url);

    let json = html_parser::Dom::parse(&s)?.to_json_pretty()?;
    let songs: serde_json::Value = serde_json::from_str(&json).unwrap();
    let song = songs.as_object().unwrap();
    let m = song.get("children").unwrap();
    let m = m.as_array().unwrap();
    assert_eq!(m.len(), 1);

    let m = m.get(0).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 2);

    let m = m.get(1).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 11);

    let m = m.get(3).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 2);

    let m = m.get(0).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 2);

    let m = m.get(1).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 5);

    let m = m.get(4).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 1);

    let m = m.get(0).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 3);

    let m = m.get(2).unwrap().as_object().unwrap();
    let songs = m.get("children").unwrap().as_array().unwrap();

    for song in songs {
        let m = song.as_object().unwrap();
        let children = m.get("children").unwrap().as_array().unwrap();
        assert_eq!(children.len(), 6);

        // Get title and id
        let title_child = children.get(0).unwrap().as_object().unwrap();
        let title_child = title_child.get("children").unwrap().as_array().unwrap();
        assert_eq!(title_child.len(), 2);
        let id_child = title_child.get(1).unwrap().as_object().unwrap();
        let id_child = id_child.get("attributes").unwrap().as_object().unwrap();
        let href = id_child.get("href").unwrap().as_str().unwrap();
        let split = href.split('/').collect::<Vec<_>>();
        assert_eq!(split.len(), 4);
        let id = split[2].parse::<usize>().unwrap();

        let title_child = title_child.get(0).unwrap().as_object().unwrap();
        let title_child = title_child.get("children").unwrap().as_array().unwrap();
        assert_eq!(title_child.len(), 1);
        let title = title_child.get(0).unwrap().as_str().unwrap();

        // Get diff
        let diff_child = children.get(1).unwrap().as_object().unwrap();
        let diff_child = diff_child.get("children").unwrap().as_array().unwrap();
        assert_eq!(diff_child.len(), 1);
        let diff = diff_child.get(0).unwrap().as_str().unwrap();

        // Get const
        let const_child = children.get(3).unwrap().as_object().unwrap();
        let const_child = const_child.get("children").unwrap().as_array().unwrap();
        assert_eq!(const_child.len(), 1);
        let const_child = const_child.get(0).unwrap(); //.as_object().unwrap();
        let (cst_exists, cst) = if !const_child.is_string() {
            let const_child = const_child.as_object().unwrap();
            assert_eq!(const_child.get("name").unwrap().as_str(), Some("i"));
            let const_child = const_child.get("children").unwrap().as_array().unwrap();
            assert_eq!(const_child.len(), 1);
            let const_child = const_child.get(0).unwrap().as_object().unwrap();
            let const_child = const_child.get("children").unwrap().as_array().unwrap();
            assert_eq!(const_child.len(), 1);
            let cst = const_child.get(0).unwrap().as_str().unwrap();
            (false, cst)
        } else {
            (true, const_child.as_str().unwrap())
        };

        let title = title.replace("&amp;", "&");
        let title = title.replace("&#039;", "'");
        let title = title.replace("&quot;", "\"");

        // Fix duplicates
        let title = if title == "Singularity" {
            vec![
                (362, title),
                (425, "Singularity (Arcaea)".to_string()),
                (487, "Singularity (MJ)".to_string()),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>()[&id]
                .clone()
        } else if title == "Hand in Hand" {
            vec![(337, title), (185, "Hand in Hand (deleted)".to_string())]
                .into_iter()
                .collect::<HashMap<_, _>>()[&id]
                .clone()
        } else if title == "Perfect Shining!!(ロケテスト譜面)" {
            "Perfect Shining!! (Location test)".to_string()
        } else {
            title
        };

        if let Some(song) = charts.get_mut(&title) {
            let diff_idx = diff_to_idx(diff);
            let lv = (*song).lv.as_mut().unwrap();
            assert_eq!(lv.lv(diff_idx), float_to_level(cst));
            if cst_exists {
                lv.set_constant(diff_idx, cst.to_string());
            }
        } else {
            // println!("{}", title);
        }
    }
    Ok(charts)
}
