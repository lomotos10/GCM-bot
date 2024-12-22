use eyre::bail;
use itertools::Itertools;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;
use poise::serenity_prelude::{
    interaction::InteractionResponseType, Color, CreateActionRow, CreateButton,
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
            "渦状銀河のシンフォニエッタ",
            "華の集落、秋のお届け",
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
            ("渦状銀河のシンフォニエッタ", "maimai BUDDiES"),
            ("華の集落、秋のお届け", "maimai BUDDiES PLUS"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    };
    static ref ADDITIONAL_REMAS: Vec<(String, String)> = {
        [
            ("Bad Apple!! feat nomico", "ORANGE PLUS"),
            ("Starlight Disco", "ORANGE PLUS"),
            ("マトリョシカ", "ORANGE PLUS"),
            ("Future", "ORANGE PLUS"),
            ("ワールズエンド・ダンスホール", "ORANGE PLUS"),
            ("SAVIOR OF SONG", "ORANGE PLUS"),
            ("明星ロケット", "PiNK"),
            ("いーあるふぁんくらぶ", "PiNK"),
            ("ナイト・オブ・ナイツ", "PiNK"),
            ("Blew Moon", "PiNK PLUS"),
            ("City Escape: Act1", "PiNK PLUS"),
            ("檄！帝国華撃団(改)", "PiNK PLUS"),
            ("ロミオとシンデレラ", "PiNK PLUS"),
            ("Tell Your World", "PiNK PLUS"),
            ("からくりピエロ", "PiNK PLUS"),
            ("Rooftop Run: Act1", "PiNK PLUS"),
            ("若い力 -SEGA HARD GIRLS MIX-", "PiNK PLUS"),
            ("ってゐ！ ～えいえんてゐVer～", "PiNK PLUS"),
            ("患部で止まってすぐ溶ける～狂気の優曇華院", "PiNK PLUS"),
            ("Save This World νMIX", "PiNK PLUS"),
            ("Living Universe", "PiNK PLUS"),
            ("ZIGG-ZAGG", "MURASAKi PLUS"),
            ("Burning Hearts ～炎のANGEL～", "MURASAKi PLUS"),
            ("Beat Of Mind", "MURASAKi PLUS"),
            ("Sun Dance", "MURASAKi PLUS"),
            ("Crush On You", "MURASAKi PLUS"),
            ("In Chaos", "MURASAKi PLUS"),
            ("だんだん早くなる", "MiLK"),
            ("ふ・れ・ん・ど・し・た・い", "MiLK"),
            ("かくしん的☆めたまるふぉ～ぜっ！", "MiLK"),
            ("カゲロウデイズ", "MiLK"),
            ("Panopticon", "MiLK PLUS"),
            ("Fragrance", "MiLK PLUS"),
            ("AMAZING MIGHTYYYY!!!!", "MiLK PLUS"),
            ("Garakuta Doll Play", "MiLK PLUS"),
            ("ガラテアの螺旋", "MiLK PLUS"),
            ("ようこそジャパリパークへ", "FiNALE"),
            ("ジングルベル", "FiNALE"),
            ("Endless World", "FiNALE"),
            ("Danza zandA", "FiNALE"),
            ("39", "FiNALE"),
            ("JACKY [Remix]", "Splash"),
            ("DADDY MULK -Groove remix-", "Splash"),
            ("We Gonna Party", "Splash"),
            ("LUCIA", "Splash"),
            ("air's gravity", "Splash"),
            ("Beat of getting entangled", "Splash"),
            ("Sky High [Reborn]", "Splash"),
            ("Death Scythe", "Splash"),
            ("Backyun! －悪い女－", "Splash"),
            ("Night Fly", "Splash"),
            ("泣き虫O'clock", "Splash"),
            ("FEEL the BEATS", "Splash"),
            ("Dragoon", "Splash"),
            ("アージェントシンメトリー", "Splash"),
            ("D✪N’T  ST✪P  R✪CKIN’", "Splash"),
            ("System “Z”", "Splash"),
            ("planet dancer", "Splash"),
            ("MAXRAGE", "UNiVERSE"),
            ("Now or Never", "UNiVERSE"),
            ("Secret Sleuth", "UNiVERSE"),
            ("バーチャルダム　ネーション", "UNiVERSE"),
            ("源平大戦絵巻テーマソング", "UNiVERSE PLUS"),
            ("記憶、記録", "UNiVERSE PLUS"),
            ("FLOWER", "UNiVERSE PLUS"),
            ("Revive The Rave", "UNiVERSE PLUS"),
            ("Limit Break", "UNiVERSE PLUS"),
            ("SPILL OVER COLORS", "UNiVERSE PLUS"),
            ("超常マイマイン", "UNiVERSE PLUS"),
            ("シエルブルーマルシェ", "UNiVERSE PLUS"),
            ("ぼくたちいつでも　しゅわっしゅわ！", "UNiVERSE PLUS"),
            ("星めぐり、果ての君へ。", "FESTiVAL"),
            ("みんなのマイマイマー", "FESTiVAL"),
            ("STEREOSCAPE", "BUDDiES"),
            ("一か罰", "BUDDiES"),
            ("Never Give Up!", "BUDDiES"),
            ("Link (maimai)", "BUDDiES"),
            ("STARTLINER", "BUDDiES"),
            ("レーイレーイ", "BUDDiES"),
            ("言ノ葉カルマ", "BUDDiES"),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    };
    static ref ADDITIONAL_DX: Vec<(String, String)> = {
        [
            ("ようこそジャパリパークへ", "maimaiでらっくす"),
            ("シュガーソングとビターステップ", "maimaiでらっくす"),
            ("かくしん的☆めたまるふぉ～ぜっ！", "maimaiでらっくす"),
            ("回レ！雪月花", "maimaiでらっくす"),
            ("六兆年と一夜物語", "maimaiでらっくす"),
            ("千本桜", "maimaiでらっくす"),
            ("脳漿炸裂ガール", "maimaiでらっくす"),
            ("39", "maimaiでらっくす"),
            ("いーあるふぁんくらぶ", "maimaiでらっくす"),
            ("シャルル", "maimaiでらっくす"),
            ("WARNING×WARNING×WARNING", "maimaiでらっくす"),
            ("月に叢雲華に風", "maimaiでらっくす"),
            (
                "チルノのパーフェクトさんすう教室　⑨周年バージョン",
                "maimaiでらっくす",
            ),
            (
                "患部で止まってすぐ溶ける～狂気の優曇華院",
                "maimaiでらっくす",
            ),
            ("Scream out! -maimai SONIC WASHER Edit-", "maimaiでらっくす"),
            ("幻想のサテライト", "maimaiでらっくす"),
            ("conflict", "maimaiでらっくす"),
            ("Oshama Scramble!", "maimaiでらっくす"),
            ("POP TEAM EPIC", "maimaiでらっくす"),
            ("ENERGY SYNERGY MATRIX", "maimaiでらっくす"),
            ("Calamity Fortune", "maimaiでらっくす"),
            ("Change Our MIRAI！", "maimaiでらっくす"),
            ("君の知らない物語", "maimaiでらっくす PLUS"),
            ("コネクト", "maimaiでらっくす PLUS"),
            ("Paradisus-Paradoxum", "maimaiでらっくす PLUS"),
            ("Daydream café", "Splash"),
            ("ダンスロボットダンス", "Splash"),
            ("天ノ弱", "Splash"),
            ("ゴーストルール", "UNiVERSE"),
            ("taboo tears you up", "UNiVERSE"),
            ("セツナトリップ", "UNiVERSE PLUS"),
            ("Grip & Break down !!", "UNiVERSE PLUS"),
            ("Starlight Disco", "UNiVERSE PLUS"),
            ("モザイクロール", "FESTiVAL"),
            ("M.S.S.Planet", "FESTiVAL"),
            ("響縁", "FESTiVAL"),
            ("火炎地獄", "FESTiVAL"),
            ("Maxi", "FESTiVAL"),
            ("ジングルベル", "FESTiVAL"),
            ("ケロ⑨destiny", "FESTiVAL"),
            ("深海少女", "FESTiVAL PLUS"),
            ("ナイト・オブ・ナイツ", "FESTiVAL PLUS"),
            ("Monochrome Rainbow", "FESTiVAL PLUS"),
            ("Selector", "FESTiVAL PLUS"),
            ("初音ミクの消失", "BUDDiES"),
            ("oboro", "BUDDiES"),
            ("色は匂へど散りぬるを", "BUDDiES"),
            ("ナミダと流星", "BUDDiES"),
            ("＊ハロー、プラネット。", "BUDDiES"),
            ("ハッピーシンセサイザ", "BUDDiES PLUS"),
            ("御旗のもとに", "BUDDiES PLUS"),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    };
}

fn get_mai_embed(title: String, ctx: &Context<'_>) -> eyre::Result<(String, Option<String>)> {
    let embed = get_mai_embed_inner(title.clone(), ctx, true)?;
    if embed.0.len() < 4096 {
        Ok(embed)
    } else {
        get_mai_embed_inner(title, ctx, false)
    }
}

fn get_mai_embed_inner(
    title: String,
    ctx: &Context<'_>,
    use_links: bool,
) -> eyre::Result<(String, Option<String>)> {
    let Some(song) = ctx.data().mai_charts.get(&title) else {
        bail!("No data for {title}");
    };

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
    if let Some(version) = &song.additional_remas_version {
        description = format!("{}\n**Version (Re:MASTER):** {}", description, version);
    }
    if let Some(version) = &song.additional_st_version {
        description = format!("{}\n**Version (ST):** {}", description, version);
    }
    if let Some(version) = &song.additional_dx_version {
        description = format!("{}\n**Version (DX):** {}", description, version);
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
                level_description(
                    song.jp_lv.as_ref().unwrap().dx.as_ref().unwrap(),
                    &title,
                    use_links
                )
            )
        }
        if st {
            description = format!(
                "{}\n\n**Level(ST):**\n{}",
                description,
                level_description(
                    song.jp_lv.as_ref().unwrap().st.as_ref().unwrap(),
                    &title,
                    use_links
                )
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
            level_description(
                jp_lv.as_ref().unwrap().dx.as_ref().unwrap(),
                &title,
                use_links,
            )
        } else {
            "**Unreleased**".to_string()
        };
        let in_dx_txt = if in_dx {
            level_description(
                in_lv.as_ref().unwrap().dx.as_ref().unwrap(),
                &title,
                use_links,
            )
        } else {
            "**Unreleased**".to_string()
        };
        if in_dx || jp_dx {
            if jp_dx_txt == in_dx_txt {
                description = format!(
                    "{}

**Level(DX)**
:flag_jp::globe_with_meridians: {}",
                    description, jp_dx_txt
                );
            } else {
                description = format!(
                    "{}

**Level(DX)**
:flag_jp: {}
:globe_with_meridians: {}",
                    description, jp_dx_txt, in_dx_txt
                );
            }
        };

        let jp_st_txt = if jp_st {
            level_description(
                jp_lv.as_ref().unwrap().st.as_ref().unwrap(),
                &title,
                use_links,
            )
        } else {
            "**Unreleased**".to_string()
        };
        let in_st_txt = if in_st {
            level_description(
                in_lv.as_ref().unwrap().st.as_ref().unwrap(),
                &title,
                use_links,
            )
        } else {
            "**Unreleased**".to_string()
        };
        if in_st || jp_st {
            if in_st_txt == jp_st_txt {
                description = format!(
                    "{}

**Level(ST)**
:flag_jp::globe_with_meridians: {}",
                    description, jp_st_txt
                );
            } else {
                description = format!(
                    "{}

**Level(ST)**
:flag_jp: {}
:globe_with_meridians: {}",
                    description, jp_st_txt, in_st_txt
                );
            }
        };
    }

    if !song.utages.is_empty() {
        let utage_info = song
            .utages
            .iter()
            .map(|utage| format!("{} **{}** *{}*", utage.kanji, utage.level, utage.comment))
            .join("\n");
        description = format!(
            "{description}

**U･TA･GE**
{utage_info}"
        );
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

fn level_description(lv: &Difficulty, title: &str, use_links: bool) -> String {
    let title = title.replace(" -", " ");
    let title = title.strip_prefix('-').unwrap_or(&title);
    let title = urlencoding::encode(title);
    if use_links {
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
    } else {
        format!(
            // "BAS **{}{}**/ADV **{}{}**/EXP **{}{}**/MAS **{}{}**{}",
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
                format!(" / R **{}**{}", rem, constant_to_string(lv.extra_c))
            } else {
                "".to_string()
            }
        )
    }
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

fn set_jp_difficulty(charts: &mut HashMap<String, MaiInfo>) -> eyre::Result<()> {
    // Get JP difficulty.
    let jp_url = fs::read_to_string("data/maimai/maimai-jp.txt")?;
    let jp_url = jp_url.trim();
    let s = get_curl(jp_url);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s)?;

    for song in songs.as_array().unwrap() {
        let song = song.as_object().unwrap();

        let title = song["title"].as_str().unwrap().to_string();
        // Edge case handling for duplicate title
        let title = if title == "Link" && song["catcode"].as_str().unwrap() == "maimai" {
            "Link (maimai)".to_string()
        } else {
            title
        };
        let title_kana = song["title_kana"].as_str().unwrap().to_string();
        let artist = song["artist"].as_str().unwrap().to_string();
        let order = song["sort"].as_str().unwrap().parse::<usize>()?;

        if song["catcode"].as_str().unwrap() == "宴会場" {
            let level = song["lev_utage"].as_str().unwrap().to_string();
            let kanji = song["kanji"].as_str().unwrap().to_string();
            let comment = song["comment"].as_str().unwrap().to_string();
            let expected_prefix = format!("[{kanji}]");
            let title = if let Some(title) = title.strip_prefix(&expected_prefix) {
                title
            } else if let Some(title) = title.strip_prefix("[宴]") {
                title
            } else {
                panic!(
                    "Illegal title on utage: expected prefix {expected_prefix} on title {title}"
                );
            };
            let info = Utage {
                level,
                kanji,
                comment,
            };
            match charts.entry(title.to_string()) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().utages.push(info);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(MaiInfo {
                        utages: vec![info],
                        title: title.to_string(),
                        artist: song["artist"].as_str().unwrap().to_string(),
                        order: Some(order),
                        title_kana,
                        ..Default::default()
                    });
                }
            }
        } else {
            let st_lv = song.contains_key("lev_bas").then(|| Difficulty {
                bas: song["lev_bas"].as_str().unwrap().to_string(),
                adv: song["lev_adv"].as_str().unwrap().to_string(),
                exp: song["lev_exp"].as_str().unwrap().to_string(),
                mas: song["lev_mas"].as_str().unwrap().to_string(),
                extra: song
                    .contains_key("lev_remas")
                    .then(|| song["lev_remas"].as_str().unwrap().to_string()),
                ..Default::default()
            });
            let dx_lv = song.contains_key("dx_lev_bas").then(|| Difficulty {
                bas: song["dx_lev_bas"].as_str().unwrap().to_string(),
                adv: song["dx_lev_adv"].as_str().unwrap().to_string(),
                exp: song["dx_lev_exp"].as_str().unwrap().to_string(),
                mas: song["dx_lev_mas"].as_str().unwrap().to_string(),
                extra: song
                    .contains_key("dx_lev_remas")
                    .then(|| song["dx_lev_remas"].as_str().unwrap().to_string()),
                ..Default::default()
            });

            let jp_lv = MaiDifficulty {
                st: st_lv,
                dx: dx_lv,
            };

            let r = charts.insert(
                title.clone(),
                MaiInfo {
                    jp_lv: Some(jp_lv),
                    title,
                    artist,
                    order: Some(order),
                    title_kana,
                    ..Default::default()
                },
            );
            assert_eq!(r, None);
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn set_jp_constants(charts: &mut HashMap<String, MaiInfo>) -> eyre::Result<()> {
    // Get jp constants
    let file = File::open("data/maimai/jp_lv.csv")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.split('\t').collect_vec();
        assert_eq!(line.len(), 7);
        let title = SONG_REPLACEMENT
            .get(line[6])
            .unwrap_or(&line[6].to_string())
            .to_string();

        if charts.contains_key(&title) {
            let entry = charts.get_mut(&title).unwrap();

            let l = &mut entry.jp_lv;
            match l {
                None => {
                    let difficulty = Difficulty {
                        bas: float_to_level(line[1], Game::Maimai),
                        bas_c: float_to_constant(line[1]),
                        adv: float_to_level(line[2], Game::Maimai),
                        adv_c: float_to_constant(line[2]),
                        exp: float_to_level(line[3], Game::Maimai),
                        exp_c: float_to_constant(line[3]),
                        mas: float_to_level(line[4], Game::Maimai),
                        mas_c: float_to_constant(line[4]),
                        extra: if line[5] == "0" {
                            None
                        } else {
                            Some(float_to_level(line[5], Game::Maimai))
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
                    if diff.bas == float_to_level(line[1], Game::Maimai) {
                        diff.bas_c = float_to_constant(line[1]);
                    } else {
                        eprintln!(
                            "Conflict on {} {} BAS: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.bas,
                            float_to_level(line[1], Game::Maimai)
                        );
                    }
                    if diff.adv == float_to_level(line[2], Game::Maimai) {
                        diff.adv_c = float_to_constant(line[2]);
                    } else {
                        eprintln!(
                            "Conflict on {} {} ADV: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.adv,
                            float_to_level(line[2], Game::Maimai)
                        );
                    }
                    if diff.exp == float_to_level(line[3], Game::Maimai) {
                        diff.exp_c = float_to_constant(line[3]);
                    } else {
                        eprintln!(
                            "Conflict on {} {} EXP: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.exp,
                            float_to_level(line[3], Game::Maimai)
                        );
                    }
                    if diff.mas == float_to_level(line[4], Game::Maimai) {
                        diff.mas_c = float_to_constant(line[4]);
                    } else {
                        eprintln!(
                            "Conflict on {} {} MAS: {} vs {}",
                            title,
                            if line[0] == "0" { "ST" } else { "DX" },
                            diff.mas,
                            float_to_level(line[4], Game::Maimai)
                        );
                    }
                    if line[5] != "0" {
                        if diff.extra == Some(float_to_level(line[5], Game::Maimai)) {
                            diff.extra_c = float_to_constant(line[5]);
                        } else {
                            eprintln!(
                                "Conflict on {} {} REM: {:?} vs {}",
                                title,
                                if line[0] == "0" { "ST" } else { "DX" },
                                diff.extra,
                                float_to_level(line[5], Game::Maimai)
                            );
                        }
                    }
                }
            }
        } else {
            panic!("chart does not contain title {title}");
        }
    }
    Ok(())
}

fn set_actual_constants(
    charts: &mut HashMap<String, MaiInfo>,
    filename: &str,
    is_jp: bool,
) -> eyre::Result<()> {
    // Get jp constants from second source.
    let file = File::open(filename)?;
    let songs: serde_json::Value = serde_json::from_reader(&file)?;
    let songs = songs.as_array().unwrap();
    for song in songs {
        let song = song.as_object().unwrap();
        if song.contains_key("Utage kanji") {
            continue;
        }

        let mut title = song["Song"].as_str().unwrap();
        let version = song["Version added"].as_str().unwrap();
        let mut dx = version.contains("でらっくす")
            || version.contains("スプラッシュ")
            || version.contains("UNiVERSE")
            || version.contains("FESTiVAL")
            || version.contains("BUDDiES")
            || version.contains("PRiSM");
        if EXCEPTIONS_ST_AFTER_DX.contains(&(title.to_string(), version.to_string())) {
            dx = !dx;
        }
        if (title, version) == ("Link", "maimai PLUS") {
            title = "Link (maimai)";
        }

        for (diff, chart) in song["Charts"].as_array().unwrap().iter().enumerate() {
            let cc = chart["Level Constant"].as_str().unwrap();
            let Some(region_diff) = charts.get_mut(title) else {
                bail!("No song of title `{title}` found")
            };
            let region_diff = if is_jp {
                region_diff.jp_lv.as_mut()
            } else {
                region_diff.intl_lv.as_mut()
            };
            let Some(region_diff) = region_diff else {
                bail!(
                    "No {} difficulty on song `{}`",
                    if is_jp { "jp" } else { "intl" },
                    title
                )
            };
            let dx_or_st_chart = if dx {
                &mut region_diff.dx
            } else {
                &mut region_diff.st
            };
            let Some(mai_diff) = dx_or_st_chart.as_mut() else {
                continue;
            };
            let current_cc = mai_diff.get_constant(diff);
            if current_cc.is_some() && format!("{:.1}", current_cc.unwrap()) != cc {
                eprintln!(
                    "{} constant sources different on song {} {} {} - {:.1} vs {}",
                    if is_jp { "JP" } else { "INTL" },
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
    Ok(())
}

fn set_intl_difficulty(charts: &mut HashMap<String, MaiInfo>) -> eyre::Result<()> {
    // Get intl difficulty.
    let jp_and_intl_version_is_different = true;
    if jp_and_intl_version_is_different {
        let file = File::open("data/maimai/in_lv.csv")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.split('\t').collect_vec();
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
                bas: float_to_level(line[1], Game::Maimai),
                bas_c: float_to_constant(line[1]),
                adv: float_to_level(line[2], Game::Maimai),
                adv_c: float_to_constant(line[2]),
                exp: float_to_level(line[3], Game::Maimai),
                exp_c: float_to_constant(line[3]),
                mas: float_to_level(line[4], Game::Maimai),
                mas_c: float_to_constant(line[4]),
                extra: if line[5] == "0" {
                    None
                } else {
                    Some(float_to_level(line[5], Game::Maimai))
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
                        intl_lv: Some(mai_difficulty),
                        title,
                        artist: "TODO".to_string(),
                        ..Default::default()
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
    Ok(())
}

fn set_song_info(charts: &mut HashMap<String, MaiInfo>) -> eyre::Result<()> {
    // Get info DB
    let info = fs::read_to_string("data/maimai/maimai-info.txt")?;
    let info = info.trim();
    let s = get_curl(info);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s)?;
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

        let exists_in_jp_func = || {
            let exists = song["sheets"].as_array()?[0].as_object()?["regions"].as_object()?["jp"]
                .as_bool()?;
            Some(exists)
        };
        let exists_in_jp = exists_in_jp_func().unwrap();

        let exists_in_intl_func = || {
            let exists = song["sheets"].as_array()?[0].as_object()?["regions"].as_object()?["intl"]
                .as_bool()?;
            Some(exists)
        };
        let mut exists_in_intl = exists_in_intl_func().unwrap();
        let mut manual_deletion = false;

        if [
            "夜に駆ける",
            "Shooting Stars",
            "MOON NIGHTのせいにして",
            "VOLTAGE",
            "veil",
            "うっせぇわ",
            "さんさーら！",
            "only my railgun",
            "お気に召すまま",
            "いつかいい感じにアレしよう",
            "二息歩行",
        ]
        .map(|c| c.to_string())
        .contains(&title)
        {
            exists_in_intl = false;
            manual_deletion = true;
        }

        let title = if title == "[宴]Oshama Scramble! (Cranky Remix)" {
            "Oshama Scramble! (Cranky Remix)".to_string()
        } else {
            title
        };

        if !charts.contains_key(&title) {
            // Is either Utage, deleted, or intl only
            if song["category"] == "宴会場" {
                // Utage info is already inserted on insert_jp_info
                continue;
            } else {
                // Deleted or intl only
                let title = song["title"].as_str().unwrap();
                charts.insert(
                    title.to_string(),
                    MaiInfo {
                        title: title.to_string(),
                        artist: song["artist"].as_str().unwrap().to_string(),
                        deleted: !exists_in_intl,
                        category: mai_get_category(song["category"].as_str().unwrap()),
                        ..Default::default()
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
            } else if dx_type == "utage" {
                // TODO
            } else {
                bail!("Unknown dx type {dx_type}");
            }

            // Get region info.
            let regions = sheet["regions"].as_object().unwrap();
            let jp_region = regions["jp"].as_bool().unwrap();
            let intl_region = regions["intl"].as_bool().unwrap();

            let diff_str = sheet["difficulty"].as_str().unwrap();
            // TODO FIX
            if diff_str.starts_with('【') {
                continue;
            }
            let diff_idx = diff_to_idx(sheet["difficulty"].as_str().unwrap());

            // We assume Basic~Master has same region availability
            if diff_idx == 0 {
                // Basic (and everything else except Remas)
                if (!jp_region && !intl_region) || manual_deletion {
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
        let version = if title == "Oshama Scramble! (Cranky Remix)" {
            Some("FiNALE".into())
        } else {
            version
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
    Ok(())
}

fn set_manual_constants(charts: &mut HashMap<String, MaiInfo>) -> eyre::Result<()> {
    // Add manual constant info
    let file = File::open("data/maimai/maimai-manual-add.txt")?;
    let lines = BufReader::new(file).lines();
    for line in lines.flatten() {
        let line = line.split('\t').collect_vec();
        assert_eq!(line.len(), 5);
        let title = line[0];
        let Some(chart) = charts.get_mut(title) else {
            bail!("{} <- title does not exist", title)
        };
        chart.deleted = false;

        let inner = if line[3] == "JP" {
            chart.jp_lv.as_mut()
        } else if line[3] == "IN" {
            if chart.intl_lv.is_none() {
                chart.intl_lv = Some(MaiDifficulty::default());
            }
            chart.intl_lv.as_mut()
        } else {
            panic!()
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
            let diff = diff_to_idx(line[2]);
            let constant = inner.get_constant(diff);
            if constant.is_some() && constant != cst {
                eprintln!(
                    "Constant mismatch on manual constant line {:?}\n{:?}, {:?}",
                    line, constant, cst
                );
            } else if constant == cst {
                eprintln!("{:?} exists on server", line);
            }
            inner.set_constant(diff, line[4].to_string());
        } else {
            // Add level
            let diff_idx = diff_to_idx(line[2]);
            let diff_str = inner.lv(diff_idx);
            assert_eq!(diff_str, "?");
            inner.set_lv(diff_idx, line[4].to_string());
        }
    }
    Ok(())
}

fn set_additional_info(charts: &mut HashMap<String, MaiInfo>) -> eyre::Result<()> {
    for (title, version) in ADDITIONAL_REMAS.iter() {
        charts
            .get_mut(title)
            .ok_or_else(|| eyre::eyre!("no song titled {title}"))?
            .additional_remas_version = Some(version.into());
    }
    for (title, version) in ADDITIONAL_DX.iter() {
        charts
            .get_mut(title)
            .ok_or_else(|| eyre::eyre!("no song titled {title}"))?
            .additional_dx_version = Some(version.into());
    }
    for (title, version) in EXCEPTIONS_ST_AFTER_DX.iter() {
        charts
            .get_mut(title)
            .ok_or_else(|| eyre::eyre!("no song titled {title}"))?
            .additional_st_version = Some(version.strip_prefix("maimai ").unwrap().into());
    }

    Ok(())
}

pub fn set_mai_charts() -> Result<HashMap<String, MaiInfo>, Error> {
    let mut charts = HashMap::new();

    set_jp_difficulty(&mut charts)?;
    // set_jp_constants(&mut charts)?;
    set_actual_constants(&mut charts, "data/maimai/prism 2024-12-12.json", true)?;
    set_intl_difficulty(&mut charts)?;
    set_song_info(&mut charts)?;
    set_actual_constants(
        &mut charts,
        "data/maimai/buddiesplus I051 2024-09-20.json",
        false,
    )?;
    // set_intl_difficulty(&mut charts)?; // TODO DELETE
    set_manual_constants(&mut charts)?;
    set_additional_info(&mut charts)?;

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
            let total = sheet.brk + sheet.tap + sheet.hold + sheet.slide + sheet.touch;
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
            let total = sheet.brk + sheet.tap + sheet.hold + sheet.slide + sheet.touch;
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

/// Get detailed info about song charts in maimai
#[poise::command(slash_command, prefix_command, rename = "detailed-mai-info")]
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
        let reply = sent.into_message().await?;
        if let Context::Application(poise_ctx) = ctx {
            let serenity_ctx = poise_ctx.serenity_context();
            let mci = match reply
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
            mci.create_interaction_response(&serenity_ctx.http, |r| {
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
