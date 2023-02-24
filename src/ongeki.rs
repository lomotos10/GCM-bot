use std::{
    collections::HashMap,
    fs::{self},
    io::Write,
    time::Duration,
};

use poise::{
    serenity_prelude::{
        self as serenity, model::interactions::InteractionResponseType, AttachmentType,
        CreateActionRow, CreateButton,
    },
    ReplyHandle,
};

use crate::utils::*;
use gcm_macro::{info_template, jacket_template};

lazy_static::lazy_static! {
    static ref LV_SOURCE_REPLACEMENT: HashMap<String, String> = {
        [
            ("Let’s Starry Party！", "Let's Starry Party！"),
            ("Let’s Starry Party！ -結城 莉玖ソロver.-", "Let's Starry Party！ -結城 莉玖ソロver.-"),
            ("Let’s Starry Party！ -藍原 椿ソロver.-", "Let's Starry Party！ -藍原 椿ソロver.-"),
            ("Let’s Starry Party！ -高瀬 梨緒ソロver.-", "Let's Starry Party！ -高瀬 梨緒ソロver.-"),
            ("Dement ~after legend~", "Dement ～after legend～"),
            ("YURUSHITE（LeaF Remix）", "YURUSHITE (LeaF Remix)"),
            ("Memories of O.N.G.E.K.I.（楽曲）", "Memories of O.N.G.E.K.I."),
            ("P！P！P！P！がおー！！", "P！P！P！P！がおー!!"),
            ("Bad Apple！！ feat．nomico", "Bad Apple!! feat.nomico"),
            ("Bad Apple！！（Camellia’s “Bad Psy” Remix）", "Bad Apple!! feat.nomico (Camellia’s “Bad Psy” Remix)"),
            ("Grip ＆ Break Down！！", "Grip & Break down !!"),
            ("メーベル（self cover）", "メーベル (self cover)"),
            ("Singularity - technoplanet", "Singularity"),
            ("Singularity - ETIA.", "Singularity (Arcaea)"),
            ("Singularity - SEGA SOUND STAFF「セガNET麻雀 MJ」", "Singularity (MJ)"),
            ("Oshama Scramble！（Cranky Remix）", "Oshama Scramble! (Cranky Remix)"),
            ("シアワセうさぎ・ぺこみこマリン（兎田ぺこら、さくらみこ、宝鐘マリン）", "シアワセうさぎ・ぺこみこマリン"),
            ("めんどーい！やっほーい！ともだち！ -井之原 小星ソロver.-", "めんどーい！やっほーい！ともだち！  -井之原 小星ソロver.-"),
            ("めんどーい！やっほーい！ともだち！ -柏木 咲姫ソロver.-", "めんどーい！やっほーい！ともだち！  -柏木 咲姫ソロver.-"),
            ("とびだせ！TO THE COSMIC！！", "とびだせ！TO THE COSMIC!!"),
            ("Change Our MIRAI！ （Our 7 Lights）", "Change Our MIRAI！ (Our 7 Lights)"),
            ("My Soul，Your Beats！", "My Soul,Your Beats!"),
            ("PinqPiq（xovevox Remix）", "PinqPiq (xovevox Remix)"),
            ("ナイト・オブ・ナイツ（xi Remix）", "ナイト・オブ・ナイツ (xi Remix)"),
            ("MEGATON BLAST（tpz Overcute Remix）", "MEGATON BLAST (tpz Overcute Remix)"),
            ("Party 4U ”holy nite mix”", "Party 4U ''holy nite mix''"),
            ("HELLO，SOFMAP WORLD", "HELLO,SOFMAP WORLD"),
            ("妖々跋扈 ～ Who done it！", "妖々跋扈 ～ Who done it！！！"),
            ("Hand in Hand - livetune", "Hand in Hand (deleted)"),
            ("３９", "39"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>()
    };

    static ref CHARACTER_REPLACEMENT: HashMap<String, String> = {
        [
            ("藤沢 柚子", "Fujisawa Yuzu"),
            ("星咲 あかり", "Hoshizaki Akari"),
            ("三角 葵", "Misumi Aoi"),
            ("高瀬 梨緒", "Takase Rio"),
            ("結城 莉玖", "Yuuki Riku"),
            ("藍原 椿", "Aihara Tsubaki"),
            ("桜井 春菜", "Sakurai Haruna"),
            ("早乙女 彩華", "Saotome Ayaka"),
            ("柏木 咲姫", "Kashiwagi Saki"),
            ("井之原 小星", "Inohara Koboshi"),
            ("九條 楓", "Kujo Kaede"),
            ("逢坂 茜", "Ousaka Akane"),
            ("珠洲島 有栖", "Suzushima Arisu"),
            ("柏木 美亜", "Kashiwagi Mia"),
            ("日向 千夏", "Hinata Chinatsu"),
            ("東雲 つむぎ", "Shinonome Tsumugi"),
            ("皇城 セツナ", "Sumeragi Setsuna"),

            ("あかニャン", "Akanyan"),
            ("みどニャン", "Midonyan"),
            ("こんじきニャン", "Konjikinyan"),
            ("あおニャン", "Aonyan"),

            ("明坂 芹菜", "Akesaka Serina"),
            ("ユメ", "Yume"),
            ("リンカ", "Linka"),
            ("セイネ", "Seine"),
            ("初音ミク", "Hatsune Miku"),
            ("巡音ルカ", "Megurine Luka"),
            ("鏡音リン", "Kagamine Rin"),
            ("光", "Hikari"),

            ("古明地 こいし", "Komeiji Koshi"),
            ("古明地 さとり", "Komeiji Satori"),
            ("風見 幽香", "Kazami Yuuka"),
            ("洩矢 諏訪子", "Moriya Suwako"),
            ("霧雨 魔理沙", "Kirisame Marisa"),
            ("射命丸 文", "Shameimaru Aya"),
            ("魂魄 妖夢", "Konpaku Youmu"),
            ("レミリア・スカーレット", "Remilia Scarlet"),
            ("フランドール・スカーレット", "Flandre Scarlet"),
            ("八雲 紫", "Yakumo Yukari"),
            ("八雲 藍", "Yakumo Ran"),
            ("蓬莱山 輝夜", "Houraisan Kaguya"),
            ("八意 永琳", "Yagokoro Eirin"),
            ("鈴仙・優曇華院・イナバ", "Reisen Udongein Inaba"),
            ("因幡 てゐ", "Inaba Tewi"),
            ("ユキ", "Yuki"),
            ("アリス・マーガトロイド", "Alice Margatroid"),
            ("紅 美鈴", "Hong Meiling"),
            ("パチュリー・ノーレッジ", "Patchouli Knowledge"),
            ("藤原 妹紅", "Fujiwara no Mokou"),
            ("西行寺 幽々子", "Saigyouji Yuyuko"),
            ("十六夜 咲夜", "Izayoi Sakuya"),
            ("チルノ", "Cirno"),
            ("博麗 霊夢", "Hakurei Reimu"),
            ("橙", "Chen"),
            ("東風谷 早苗", "Kochiya Sanae"),

            ("名取さな", "Natori Sana"),

            ("ハチロク", "Hachiroku"),
            ("ティアラ", "Tiara"),
            ("メアリーベリー", "Maryberry"),
            ("ペコリーヌ", "Pecorine"),
            ("キャル", "Kyaru"),
            ("赤座 あかり", "Akaza Akari"),
            ("歳納 京子", "Toshinou Kyouko"),
            ("杉浦 綾乃", "Sugiura Ayano"),
            ("由比ヶ浜 結衣", "Yuigahama Yui"),
            ("雪ノ下 雪乃", "Yukinoshita Yukino"),
            ("上葉 みあ", "Ageha Mia"),
            ("エミリア", "Emilia"),
            ("白雪姫リボン", "Shirayukihime Ribbon"),
            ("吉田 優子", "Yoshida Yuko"),
            ("水原 千鶴", "Mizuhara Chizuru"),
            ("チノ", "Chino"),
            ("ジャンヌ・ダルク", "Jeanne d'Arc"),
            ("双挽 乃保", "Soubiji Noho"),
            ("魔法少女リリカ", "Magical girl Lyrica"),
            ("メグメグ", "Megumegu"),
            ("コクリコット ブランシュ", "Coquelicot Blanche"),
            ("ロザリー", "Rosalie"),
            ("星月 みき", "Hoshitsuki Miki"),
            ("春日部 ハル", "Kasukabe Haru"),
            ("かなで", "Kanade"),
            ("リムル＝テンペスト[人型]", "Rimuru Tempest"),
            ("ミリム・ナーヴァ", "Milim Nava"),
            ("アリサ", "Arisa"),
            ("桃山 みらい", "Momoyama Mirai"),
            ("草津 結衣奈", "Kusatsu Yuina"),
            ("アイリス・ディセンバー・アンクライ", "Iris December Uncry"),
            ("為栗 メロ", "Shiteguri Mero"),
            ("鹿目 まどか", "Kaname Madoka"),
            ("暁美 ほむら", "Akemi Homura"),
            ("天海 春香", "Amami Haruka"),
            ("芹沢 あさひ", "Serizawa Asahi"),
            ("サンドリヨン", "Cendrillion"),
            ("式宮 舞菜", "Shikimiya Mana"),
            ("鳶沢 みさき", "Tobisawa Misaki"),
            ("十条 姫和", "Jujo Hiyori"),
            ("ココア", "Cocoa"),
            ("滝本 ひふみ", "Takimoto Hifumi"),
            ("中野 五月", "Nakano Itsuki"),
            ("八神 コウ", "Yagami Kou"),
            ("山手響子", "Yamate Kyoko"),
            ("衛藤 可奈美", "Eto Kanami"),
            ("レム", "Rem"),
            ("絢辻 詞", "Ayatsuji Tsukasa"),
            ("六石 陽菜", "Mutsuishi Haruna"),
            ("涼風 青葉", "Suzukaze Aoba"),
            ("青柳 椿", "Aoyagi Tsubaki"),
            ("イレイナ", "Elaina"),
            ("日向 美海", "Hinata Miumi"),
            ("真中 らぁら", "Manaka Laala"),

            // ("Two for all", ""),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>()
    };

    static ref DELETED_SONGS_20230210: Vec<String> = [
        "Believe",
        "Deep-Connect",
        "Snow in \"I love you\"",
        "NATSUKAGE-夏陰-",
        "INDETERMINATE UNIVERSE",
        "お願いマッスル",
        "リトルソルジャー",
        "どどんぱち大音頭",
        "100%ちゅ～学生",
        "いぇす!ゆゆゆ☆ゆるゆり♪♪",
        "ピースサイン",
        "ヒャダインのカカカタ☆カタオモイ-C",
        "深海のリトルクライ feat. 土岐麻子",
        "Catch the Moment",
        "わたしたち魔法乙女です☆",
        "エブリデイワールド",
        "春擬き",
        "Contrail ～軌跡～",
        "Nameless Story",
    ].iter().map(|s| s.to_string()).collect();
}

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

    let mut vs = song.character.clone();
    if CHARACTER_REPLACEMENT.contains_key(&song.character) {
        vs = format!("{} ({})", vs, CHARACTER_REPLACEMENT[&song.character]);
    }
    if !song.element.is_empty() {
        vs = format!(
            "{} :{}: Lv. {}",
            vs,
            if song.element == "FIRE" {
                "fire"
            } else if song.element == "LEAF" {
                "leaves"
            } else if song.element == "AQUA" {
                "droplet"
            } else {
                unreachable!()
            },
            song.char_lv
        );
    }

    let vs_text = if song.deleted {
        "".to_string()
    } else {
        format!("**VS**: {}\n", vs)
    };

    let description = if song.deleted {
        "**THIS SONG IS DELETED**\n\n"
    } else {
        ""
    }
    .to_string();

    let description = format!(
        "{}**Artist:** {}
**Version**: {}
{}
**Level:** {}",
        description,
        song.artist.replace('*', "\\*"),
        version,
        vs_text,
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
        "ctx.data().ongeki_jacket_prefix"
    );
    Ok(())
}

fn level_description(lv: &Difficulty, title: &str) -> String {
    let title = urlencoding::encode(title);
    if !lv.bas.is_empty() {
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
    } else {
        // Assume level has LUNATIC
        if let Some(rem) = &lv.extra {
            format!(
                "[L](https://www.youtube.com/results?search_query=オンゲキ+{}+LUNATIC) **{}**{}",
                title,
                rem,
                constant_to_string(lv.extra_c)
            )
        } else {
            "".to_string()
        }
    }
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
        "ctx.data().ongeki_jacket_prefix"
    );
    Ok(())
}

fn set_jp_difficulty(charts: &mut HashMap<String, OngekiInfo>) {
    let url = fs::read_to_string("data/ongeki/ongeki-url.txt").unwrap();
    let url = url.trim();
    let s = get_curl(url);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();

    let songs = songs.as_array().unwrap();

    for song in songs {
        let song = if let serde_json::Value::Object(m) = song {
            m
        } else {
            panic!()
        };

        let mut title = song["title"].as_str().unwrap().to_string();
        let artist = song["artist"].as_str().unwrap().to_string();
        // let jacket = song["image_url"].as_str().unwrap().to_string();
        let date = song["date"].as_str().unwrap().to_string();
        let character = song["character"].as_str().unwrap().to_string();
        let category = song["category"].as_str().unwrap().to_string();
        let lv = Difficulty {
            bas: song["lev_bas"].as_str().unwrap().to_string(),
            adv: song["lev_adv"].as_str().unwrap().to_string(),
            exp: song["lev_exc"].as_str().unwrap().to_string(),
            mas: song["lev_mas"].as_str().unwrap().to_string(),
            extra: if song.contains_key("lev_lnt") {
                let lnt = song.get("lev_lnt").unwrap();
                if lnt == "" {
                    None
                } else {
                    Some(lnt.as_str().unwrap().to_string())
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
            if song["lev_bas"].as_str().unwrap().to_string().is_empty() {
                // 1. LUNATIC added to normal chart - items have empty level items
                assert_eq!(song["lev_bas"].as_str().unwrap().to_string(), "");
                let diff = charts.get_mut(&title).unwrap().lv.as_mut().unwrap();
                diff.extra = Some(song["lev_lnt"].as_str().unwrap().to_string())
            } else {
                // 2. Normal chart added to lunatic - items have empty lunatic item
                assert_eq!(song["lev_lnt"].as_str().unwrap().to_string(), "");
                let diff = charts.get_mut(&title).unwrap().lv.as_mut().unwrap();
                diff.bas = song["lev_bas"].as_str().unwrap().to_string();
                diff.adv = song["lev_adv"].as_str().unwrap().to_string();
                diff.exp = song["lev_exc"].as_str().unwrap().to_string();
                diff.mas = song["lev_mas"].as_str().unwrap().to_string();
            }
        } else {
            charts.insert(
                title.clone(),
                OngekiInfo {
                    lv: Some(lv),
                    jp_jacket: None,
                    title,
                    artist,
                    date,
                    character,
                    category: ongeki_get_category(&category),
                    element: "".to_string(),
                    char_lv: 9999,
                    deleted: false,
                },
            );
        }
    }
}

fn set_deleted_songs(charts: &mut HashMap<String, OngekiInfo>) {
    let url = fs::read_to_string("data/ongeki/ongeki-deleted.txt").unwrap();
    let url = url.trim();
    let s = get_curl(url);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();
    let songs = songs.as_object().unwrap()["songs"].as_array().unwrap();

    for song in songs {
        let song = song.as_object().unwrap();

        let title = song["title"].as_str().unwrap();
        let id = song["songId"].as_str().unwrap();
        // Edge case handling for duplicate title
        let title = if id == "Singularity (2)" {
            "Singularity (Arcaea)".to_string()
        } else if id == "Singularity (3)" {
            "Singularity (MJ)".to_string()
        } else {
            title.to_string()
        };

        let jp_jacket = Some(song["imageName"].as_str().unwrap().to_string());
        let artist = song["artist"].as_str().unwrap().to_string();
        let date = song["releaseDate"]
            .as_str()
            .unwrap()
            .replace('-', "")
            .parse::<usize>()
            .unwrap();

        if !charts.contains_key(&title) {
            // Song is deleted, add element to charts
            let sheets = song["sheets"].as_array().unwrap();
            let mut lv = Difficulty::default();
            for i in 0..4 {
                // Default value of lv is "?" - change it to "" for ongeki processing
                lv.set_lv(i, "".to_string());
            }

            let category_str = song["category"].as_str().unwrap();
            let category;
            if category_str == "LUNATIC" {
                category = OngekiCategory::Error;
                let sheet = sheets[0].as_object().unwrap();

                let level = sheet["level"].as_str().unwrap();
                lv.set_lv(4, level.to_string());

                let cc = &sheet["internalLevel"];
                if !cc.is_null() {
                    let cc = cc.as_str().unwrap();
                    lv.set_constant(4, cc.to_string());
                }
            } else {
                category = if category_str == "ボーナストラック" {
                    OngekiCategory::Ongeki
                } else {
                    ongeki_get_category(category_str)
                };
                for (idx, sheet) in sheets.iter().enumerate() {
                    let sheet = sheet.as_object().unwrap();

                    let level = sheet["level"].as_str().unwrap();
                    lv.set_lv(idx, level.to_string());

                    let cc = &sheet["internalLevel"];
                    if cc.is_null() {
                        continue;
                    }
                    let cc = cc.as_str().unwrap();
                    lv.set_constant(idx, cc.to_string());
                }
            }

            charts.insert(
                title.to_string(),
                OngekiInfo {
                    lv: Some(lv),
                    jp_jacket,
                    title,
                    artist,
                    date,
                    character: "".to_string(),
                    category,
                    element: "".to_string(),
                    char_lv: 9999,
                    deleted: true,
                },
            );
        } else {
            // just add the jacket
            charts.get_mut(&title).unwrap().jp_jacket = jp_jacket;
        }
    }
}

fn set_constants(charts: &mut HashMap<String, OngekiInfo>) {
    let url = fs::read_to_string("data/ongeki/ongeki-info.txt").unwrap();
    let url = url.trim();
    let s = get_curl(url);

    // Get table element from entire html
    let json = html_parser::Dom::parse(&s)
        .unwrap()
        .to_json_pretty()
        .unwrap();
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
    assert_eq!(m.len(), 3);

    let m = m.get(2).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 1);

    let m = m.get(0).unwrap().as_object().unwrap();
    let m = m.get("children").unwrap().as_array().unwrap();
    assert_eq!(m.len(), 3);

    let m = m.get(2).unwrap().as_object().unwrap();
    let songs = m.get("children").unwrap().as_array().unwrap();

    // Parse table to earn cc
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
            let lv = song.lv.as_mut().unwrap();
            assert_eq!(lv.lv(diff_idx), float_to_level(cst));
            if cst_exists {
                lv.set_constant(diff_idx, cst.to_string());
            }
        } else {
            // eprintln!("{}", title);
        }
    }
}

fn set_vs_character_level_element(charts: &mut HashMap<String, OngekiInfo>) {
    // Get VS character level and element.
    let s = fs::read_to_string("data/ongeki/ongeki-curl.html").unwrap();
    let dom = tl::parse(&s, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .nodes()
        .iter()
        .filter(|a| a.as_tag().is_some())
        .map(|a| {
            (
                a,
                a.as_tag()
                    .unwrap()
                    .attributes()
                    .class()
                    .map(|x| x.as_utf8_str()),
            )
        })
        .filter(|a| a.1.is_some())
        .map(|(a, b)| (a, b.unwrap()))
        .filter(|a| a.1.contains("mu__table--scroll_inside"))
        .map(|a| a.0)
        .collect::<Vec<_>>();

    for (idx, node) in element.iter().enumerate() {
        let children = node.children().unwrap();
        let node = children.top()[0].get(parser).unwrap();
        let children = node.children().unwrap();
        let node = children.top()[3].get(parser).unwrap();

        let children = node.children().unwrap();
        let children = children
            .top()
            .iter()
            .filter_map(|t| t.get(parser).unwrap().as_tag())
            .collect::<Vec<_>>();

        for node in children {
            let children = node
                .children()
                .top()
                .iter()
                .map(|t| t.get(parser).unwrap())
                .collect::<Vec<_>>();

            let mut title_node = children[1];
            while title_node.children().is_some() {
                let top = title_node.children().unwrap();
                let top = top.top();
                if top.len() > 2 {
                    eprintln!("{:#?}", top[0].get(parser));
                    eprintln!("{:#?}", top[1].get(parser));
                    panic!();
                }
                title_node = top[0].get(parser).unwrap();
            }
            let title_lv = title_node
                .as_raw()
                .unwrap()
                .try_as_utf8_str()
                .unwrap()
                .trim();
            // eprintln!("{:#?}", title_lv);

            let mut character_node = children[3];
            while character_node.children().is_some() {
                let top = character_node.children().unwrap();
                let top = top.top();
                if top.len() > 2 {
                    eprintln!("{:#?}", top[0].get(parser));
                    eprintln!("{:#?}", top[1].get(parser));
                    panic!();
                }
                // assert_eq!(top.len(), 1);
                character_node = top[0].get(parser).unwrap();
            }
            let character = character_node
                .as_raw()
                .unwrap()
                .try_as_utf8_str()
                .unwrap()
                .trim();
            // eprintln!("{:#?}", character);

            let mut lv_node = children[4];
            while lv_node.children().is_some() {
                let top = lv_node.children().unwrap();
                let top = top.top();
                if top.len() > 2 {
                    eprintln!("{:#?}", top[0].get(parser));
                    eprintln!("{:#?}", top[1].get(parser));
                    panic!();
                }
                // assert_eq!(top.len(), 1);

                lv_node = top[top.len() - 1].get(parser).unwrap();
            }
            let lv = lv_node.as_raw().unwrap().try_as_utf8_str().unwrap().trim();

            let title_lv2 = if !charts.contains_key(title_lv) {
                let title_lv = title_lv.replace('！', "!");
                let title_lv = title_lv.replace('’', "'");
                let title_lv = title_lv.replace('（', "(");
                let title_lv = title_lv.replace('）', ")");
                let title_lv = title_lv.replace('＋', "+");
                let title_lv = title_lv.replace('，', ", ");
                let title_lv = title_lv.replace('＆', "&");
                let title_lv = title_lv.replace('”', "\"");
                let title_lv = title_lv.replace('［', "[");
                let title_lv = title_lv.replace('］', "]");
                let title_lv = title_lv.replace('：', ":");
                title_lv.replace('％', "%")
            } else {
                title_lv.to_string()
            };

            // Deleted songs that are still in cc info db
            if DELETED_SONGS_20230210.contains(&title_lv2) {
                continue;
            }

            let title_lv3 = if !charts.contains_key(&title_lv2) {
                LV_SOURCE_REPLACEMENT[title_lv].to_string()
            } else {
                title_lv2
            };

            let title_lv = title_lv3;
            if title_lv == "Hand in Hand (deleted)" {
                continue;
            }

            assert!(charts.contains_key(&title_lv));

            let c2: String = character.split_whitespace().collect();
            let c3: String = charts[&title_lv].character.split_whitespace().collect();
            if c2.contains(&c3) || character == "リムル＝テンペスト［人型］" {
                charts.get_mut(&title_lv).unwrap().element = if idx / 2 == 0 {
                    "FIRE"
                } else if idx / 2 == 1 {
                    "LEAF"
                } else {
                    "AQUA"
                }
                .to_string();
                charts.get_mut(&title_lv).unwrap().char_lv = lv
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            }
        }
    }
}

pub fn set_ongeki_charts() -> Result<HashMap<String, OngekiInfo>, Error> {
    let mut charts: HashMap<String, OngekiInfo> = HashMap::new();

    set_jp_difficulty(&mut charts);
    set_deleted_songs(&mut charts);
    set_constants(&mut charts);
    set_vs_character_level_element(&mut charts);

    Ok(charts)
}
