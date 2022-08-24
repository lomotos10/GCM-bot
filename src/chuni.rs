use std::{
    collections::HashMap,
    fs::{self},
    io::Write,
    time::Duration,
};

use lazy_static::lazy_static;
use poise::{
    serenity_prelude::{
        self as serenity, AttachmentType, CreateActionRow, CreateButton, InteractionResponseType,
    },
    ReplyHandle,
};

use crate::utils::*;
use gcm_macro::{info_template, jacket_template};

lazy_static! {
    static ref INTL_VIEWER_REPLACEMENT: HashMap<String, String> = {
        [
            ("セイクリッド ルイン", "セイクリッド　ルイン"),
            (
                "Bad Apple!! feat.nomico(REDALiCE Remix)",
                "Bad Apple!! feat.nomico (REDALiCE Remix)",
            ),
            (
                "Satellite System ft. Diana Chiaki",
                "Satellite System ft.Diana Chiaki",
            ),
            ("妖々跋扈～Who done it!!!", "妖々跋扈　～ Who done it！！！"),
            ("DAZZLING♡SEAZON", "DAZZLING♡SEASON"),
            ("The EmpErroR", "the EmpErroR"),
            ("SQUAD-phvntom-", "SQUAD-Phvntom-"),
            ("ピアノ協奏曲第1番\"蠍火\"", "ピアノ協奏曲第１番”蠍火”"),
            ("Iudicium \"Apocalypsis Mix\"", "Iudicium “Apocalypsis Mix”"),
            (
                "ナイト・オブ・ナイツ(かめりあ's \"ワンス・アポン・ア・ナイト\"Remix)",
                "ナイト・オブ・ナイツ (かめりあ’s“ワンス・アポン・ア・ナイト”Remix)",
            ),
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

    // let in_lv = &song.intl_lv;
    let jp_lv = &song.jp_lv;

    let jp_txt = if let Some(jp_lv) = jp_lv {
        level_description(jp_lv)
    } else {
        "**Unreleased**".to_string()
    };
//     let in_txt = if let Some(in_lv) = in_lv {
//         level_description(in_lv)
//     } else {
//         "**Unreleased**".to_string()
//     };
//     if in_txt == jp_txt {
//         description = format!(
//             "{}

// **Level:**
// :flag_jp::globe_with_meridians: {}",
//             description, jp_txt
//         );
//     } else {
//         description = format!(
//             "{}

// **Level:**
// :flag_jp: {}
// :globe_with_meridians: {}",
//             description, jp_txt, in_txt
//         );
//     }
    description = format!("{}

**Level:** {}"
    , description, jp_txt);

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

fn level_description(lv: &Difficulty) -> String {
    format!(
        "B **{}**{} / A **{}**{} / E **{}**{} / M **{}**{}{}",
        lv.bas,
        constant_to_string(lv.bas_c),
        lv.adv,
        constant_to_string(lv.adv_c),
        lv.exp,
        constant_to_string(lv.exp_c),
        lv.mas,
        constant_to_string(lv.mas_c),
        if let Some(rem) = &lv.extra {
            format!(" / U **{}**{}", rem, constant_to_string(lv.extra_c))
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
    let url = fs::read_to_string("data/chuni-url.txt")?;
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
    let url = fs::read_to_string("data/chuni-intl.txt")?;
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

            let data = charts.get_mut(&title).unwrap();
            if let Some(intl_lv) = &mut (*data).intl_lv {
                if let Some(ult) = song.get("lev_ul") {
                    assert_eq!((*intl_lv).extra, None);
                    (*intl_lv).extra = Some(serdest_to_string(ult));
                }
            } else {
                (*data).intl_lv = Some(intl_lv);
            }
        } else {
            // WORLD'S END item; TODO implement
        }
    }

    // Get constants
    let intl_viewer = fs::read_to_string("chuni_intl_viewer/chartConstant.json")?;
    let songs: serde_json::Value = serde_json::from_str(&intl_viewer).unwrap();
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

        let title = serdest_to_string(song.get("name").unwrap());
        let diff = serdest_to_string(song.get("difficulty").unwrap());

        let elem = charts.get_mut(&title);
        let elem = if let Some(v) = elem {
            v
        } else {
            charts.get_mut(&INTL_VIEWER_REPLACEMENT[&title]).unwrap()
        };
        if diff == "EXP" {
            if let Some(lv) = song.get("constant") {
                let lv = &serdest_to_string(lv);
                if let Some(intl_lv) = (*elem).intl_lv.as_mut() {
                    intl_lv.exp_c = float_to_constant(lv);
                    if intl_lv.exp == "?" {
                        intl_lv.exp = float_to_chuni_level(lv);
                    } else {
                        assert_eq!(intl_lv.exp, float_to_chuni_level(lv));
                    }
                } else {
                    (*elem).intl_lv = Some(Difficulty {
                        exp: float_to_chuni_level(lv),
                        exp_c: float_to_constant(lv),
                        ..Default::default()
                    });
                }
            }
            if let Some(lv) = song.get("constant_new_plus") {
                (*elem).jp_lv.as_mut().unwrap().exp_c = float_to_constant(&serdest_to_string(lv));
            }
        } else if diff == "MAS" {
            if let Some(lv) = song.get("constant") {
                let lv = &serdest_to_string(lv);
                if let Some(intl_lv) = (*elem).intl_lv.as_mut() {
                    intl_lv.mas_c = float_to_constant(lv);
                    if intl_lv.mas == "?" {
                        intl_lv.mas = float_to_chuni_level(lv);
                    } else {
                        assert_eq!(intl_lv.mas, float_to_chuni_level(lv));
                    }
                } else {
                    (*elem).intl_lv = Some(Difficulty {
                        mas: float_to_chuni_level(lv),
                        mas_c: float_to_constant(lv),
                        ..Default::default()
                    });
                }
            }
            if let Some(lv) = song.get("constant_new_plus") {
                (*elem).jp_lv.as_mut().unwrap().mas_c = float_to_constant(&serdest_to_string(lv));
            }
        } else if diff == "ULT" {
            if let Some(lv) = song.get("constant") {
                let lv = &serdest_to_string(lv);
                if let Some(intl_lv) = (*elem).intl_lv.as_mut() {
                    intl_lv.extra_c = float_to_constant(lv);
                    if intl_lv.extra == None {
                        intl_lv.extra = Some(float_to_chuni_level(lv));
                    } else {
                        assert_eq!(intl_lv.extra, Some(float_to_chuni_level(lv)));
                    }
                } else {
                    (*elem).intl_lv = Some(Difficulty {
                        extra: Some(float_to_chuni_level(lv)),
                        extra_c: float_to_constant(lv),
                        ..Default::default()
                    });
                }
            }
            if let Some(lv) = song.get("constant_new_plus") {
                (*elem).jp_lv.as_mut().unwrap().extra_c = float_to_constant(&serdest_to_string(lv));
            }
        } else {
            unimplemented!()
        }
    }
    Ok(charts)
}
