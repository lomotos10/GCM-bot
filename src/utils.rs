use ordered_float::OrderedFloat;
use poise::serenity_prelude::{
    model::application::interaction::InteractionResponseType, AttachmentType, Color,
    CreateActionRow, CreateButton, GuildId,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use walkdir::WalkDir;

/////////////////////// General utils ///////////////////////

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// pub const USER_COOLDOWN: i64 = 1800;
// pub const CHANNEL_COOLDOWN: i64 = 300;

// type Maps = (HashMap<UserId, i64>, HashMap<ChannelId, i64>);

#[derive(Debug, poise::ChoiceParameter, Copy, Clone, PartialEq)]
pub enum Game {
    #[name = "maimai"]
    Maimai,
    #[name = "CHUNITHM"]
    Chunithm,
    #[name = "O.N.G.E.K.I."]
    Ongeki,
}

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub mai_charts: HashMap<String, MaiInfo>,
    pub mai_aliases: Aliases,
    pub mai_jacket_prefix: String,

    pub chuni_charts: HashMap<String, ChuniInfo>,
    pub chuni_aliases: Aliases,
    pub chuni_jacket_prefix: String,

    pub ongeki_charts: HashMap<String, OngekiInfo>,
    pub ongeki_aliases: Aliases,
    pub ongeki_jacket_prefix: String,

    pub manual_alias_file_maimai: Arc<Mutex<File>>,
    pub manual_alias_file_chuni: Arc<Mutex<File>>,
    pub manual_alias_file_ongeki: Arc<Mutex<File>>,

    // pub cooldown_server_ids: HashSet<GuildId>,
    // pub cooldown_channel_exception_ids: HashSet<ChannelId>,
    // pub timestamps: Arc<Mutex<HashMap<GuildId, Maps>>>,
    pub alias_log: Arc<Mutex<File>>,
}

#[allow(dead_code)]
pub enum Cooldown {
    User(i64),
    Channel(i64),
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Difficulty {
    pub bas: String,
    pub adv: String,
    pub exp: String,
    pub mas: String,
    pub extra: Option<String>,

    pub bas_c: Option<OrderedFloat<f32>>,
    pub adv_c: Option<OrderedFloat<f32>>,
    pub exp_c: Option<OrderedFloat<f32>>,
    pub mas_c: Option<OrderedFloat<f32>>,
    pub extra_c: Option<OrderedFloat<f32>>,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            bas: "?".to_string(),
            adv: "?".to_string(),
            exp: "?".to_string(),
            mas: "?".to_string(),
            extra: None,
            bas_c: None,
            adv_c: None,
            exp_c: None,
            mas_c: None,
            extra_c: None,
        }
    }
}

impl Difficulty {
    pub fn get_const_mut(&mut self, idx: usize) -> &mut Option<OrderedFloat<f32>> {
        if idx == 0 {
            &mut self.bas_c
        } else if idx == 1 {
            &mut self.adv_c
        } else if idx == 2 {
            &mut self.exp_c
        } else if idx == 3 {
            &mut self.mas_c
        } else if idx == 4 {
            &mut self.extra_c
        } else {
            panic!()
        }
    }

    pub fn lv(&self, idx: usize) -> String {
        if idx == 0 {
            self.bas.clone()
        } else if idx == 1 {
            self.adv.clone()
        } else if idx == 2 {
            self.exp.clone()
        } else if idx == 3 {
            self.mas.clone()
        } else if idx == 4 {
            self.extra.as_ref().unwrap_or(&"?".to_string()).clone()
        } else {
            panic!()
        }
    }

    #[allow(dead_code)]
    pub fn get_constant(&self, idx: usize) -> Option<OrderedFloat<f32>> {
        if idx == 0 {
            self.bas_c
        } else if idx == 1 {
            self.adv_c
        } else if idx == 2 {
            self.exp_c
        } else if idx == 3 {
            self.mas_c
        } else if idx == 4 {
            self.extra_c
        } else {
            panic!()
        }
    }

    pub fn set_lv(&mut self, idx: usize, lv: String) -> String {
        if idx == 0 {
            let s = self.bas.clone();
            self.bas = lv;
            s
        } else if idx == 1 {
            let s = self.adv.clone();
            self.adv = lv;
            s
        } else if idx == 2 {
            let s = self.exp.clone();
            self.exp = lv;
            s
        } else if idx == 3 {
            let s = self.mas.clone();
            self.mas = lv;
            s
        } else if idx == 4 {
            let s = self.lv(4);
            self.extra = Some(lv);
            s
        } else {
            panic!()
        }
    }

    pub fn set_constant(&mut self, idx: usize, lv: String) -> String {
        let lv = float_to_constant(&lv).unwrap();
        if idx == 0 {
            let s = self.bas.clone();
            self.bas_c = Some(lv);
            s
        } else if idx == 1 {
            let s = self.adv.clone();
            self.adv_c = Some(lv);
            s
        } else if idx == 2 {
            let s = self.exp.clone();
            self.exp_c = Some(lv);
            s
        } else if idx == 3 {
            let s = self.mas.clone();
            self.mas_c = Some(lv);
            s
        } else if idx == 4 {
            let s = self.lv(4);
            self.extra_c = Some(lv);
            s
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
pub struct Aliases {
    pub main: MainAliases<String>,
    // Outer hashmap: Maps from guild id to inner hashmap.
    // Inner hashmap: Maps from alias to (song title, user that uploaded alias)
    pub manual: HashMap<GuildId, MainAliases<(String, String)>>,
}

#[derive(Debug, Default)]
pub struct MainAliases<V> {
    pub original: HashMap<String, V>,
    pub lowercased: HashMap<String, V>,
    pub lowercased_and_unspaced: HashMap<String, V>,
    pub alphanumeric_only: HashMap<String, V>,
    pub alphanumeric_and_ascii: HashMap<String, V>,
    pub nicknames_lowercased_and_unspaced: HashMap<String, V>,
    pub nicknames_alphanumeric_only: HashMap<String, V>,
    pub nicknames_alphanumeric_and_ascii: HashMap<String, V>,
}

pub fn get_curl(url: &str) -> String {
    let mut data = Vec::new();
    let mut handle = curl::easy::Easy::new();
    handle.url(url.trim()).unwrap();
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

// TODO: NEEDS REFACTOR
pub fn get_title(title: &str, all_aliases: &Aliases, server_id: GuildId) -> Option<String> {
    let aliases = &all_aliases.main;
    if let Some(a) = aliases.original.get(title) {
        return Some(a.to_string());
    }
    let titlem1 = title.to_lowercase();
    if let Some(a) = aliases.lowercased.get(&titlem1) {
        return Some(a.to_string());
    }
    let title0 = titlem1.split_whitespace().collect::<String>();
    if let Some(a) = aliases.lowercased_and_unspaced.get(&title0) {
        return Some(a.to_string());
    }
    let title1 = title0
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    if let Some(a) = aliases.alphanumeric_only.get(&title1) {
        return Some(a.to_string());
    }
    let title2 = title1.chars().filter(|c| c.is_ascii()).collect::<String>();
    if let Some(a) = aliases.alphanumeric_and_ascii.get(&title2) {
        return Some(a.to_string());
    }
    if let Some(a) = aliases.nicknames_lowercased_and_unspaced.get(&title0) {
        return Some(a.to_string());
    }
    if let Some(a) = aliases.nicknames_alphanumeric_only.get(&title1) {
        return Some(a.to_string());
    }
    if let Some(a) = aliases.nicknames_alphanumeric_and_ascii.get(&title2) {
        return Some(a.to_string());
    }

    let server_aliases = all_aliases.manual.get(&server_id)?;
    let aliases = server_aliases;
    let titlem1 = title.to_lowercase();
    if let Some(a) = aliases.lowercased.get(&titlem1) {
        return Some(a.1.to_string());
    }
    let title0 = titlem1.split_whitespace().collect::<String>();
    if let Some(a) = aliases.lowercased_and_unspaced.get(&title0) {
        return Some(a.1.to_string());
    }
    let title1 = title0
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    if let Some(a) = aliases.alphanumeric_only.get(&title1) {
        return Some(a.1.to_string());
    }
    let title2 = title1.chars().filter(|c| c.is_ascii()).collect::<String>();
    if let Some(a) = aliases.alphanumeric_and_ascii.get(&title2) {
        return Some(a.1.to_string());
    }
    if let Some(a) = aliases.nicknames_lowercased_and_unspaced.get(&title0) {
        return Some(a.1.to_string());
    }
    if let Some(a) = aliases.nicknames_alphanumeric_only.get(&title1) {
        return Some(a.1.to_string());
    }
    if let Some(a) = aliases.nicknames_alphanumeric_and_ascii.get(&title2) {
        return Some(a.1.to_string());
    }

    None
}

pub fn set_aliases<'a, I>(titles: I, game: &str) -> Result<Aliases, Error>
where
    I: Iterator<Item = &'a String>,
{
    let mut original = HashMap::new();
    let mut lowercased = HashMap::new();
    let mut lowercased_and_unspaced = HashMap::new();
    let mut alphanumeric_only = HashMap::new();
    let mut alphanumeric_and_ascii = HashMap::new();
    let mut nicknames_lowercased_and_unspaced = HashMap::new();
    let mut nicknames_alphanumeric_only = HashMap::new();
    let mut nicknames_alphanumeric_and_ascii = HashMap::new();
    // Oh god what is this trainwreck
    for title in titles {
        original.insert(title.to_string(), title.to_string());

        let namem1 = title.to_lowercase();
        let a = lowercased.insert(namem1.to_string(), title.to_string());
        if let Some(a) = a {
            eprintln!(
                "Alias-1 {} (for {}) shadowed by same alias-1 for {}",
                namem1, a, title
            );
        }

        let name0 = title.to_lowercase().split_whitespace().collect::<String>();
        let a = lowercased_and_unspaced.insert(name0.to_string(), title.to_string());
        if let Some(a) = a {
            eprintln!(
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
                eprintln!(
                    "Alias1 {} (for {}) shadowed by same alias1 for {}",
                    name1, a, title
                );
            }
        }

        let name2 = name1.chars().filter(|c| c.is_ascii()).collect::<String>();
        if !name2.is_empty() {
            let a = alphanumeric_and_ascii.insert(name2.to_string(), title.to_string());
            if let Some(a) = a {
                eprintln!(
                    "Alias2 {} (for {}) shadowed by same alias2 for {}",
                    name2, a, title
                );
            }
        }
    }

    // Set aliases
    let files = WalkDir::new("./data/aliases")
        .into_iter()
        .filter_map(|file| file.ok())
        // filter files with correct filename
        .filter(|file| {
            file.path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .eq(&format!("{}.tsv", game))
        })
        // Filter out `manual/game.tsv`
        .filter(|file| {
            !file
                .path()
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .eq("manual")
        })
        .map(|path| File::open(path.path()).unwrap());
    for file in files {
        let lines = BufReader::new(file).lines();
        for line in lines.map_while(Result::ok) {
            let split = line.split('\t');
            let split = split.collect::<Vec<_>>();
            let title = split[0];

            let nickname_slice = &split[1..];
            for nickname in nickname_slice {
                let nick = nickname
                    .to_lowercase()
                    .split_whitespace()
                    .collect::<String>();
                if !nick.is_empty() {
                    let a = nicknames_lowercased_and_unspaced
                        .insert(nick.to_string(), title.to_string());
                    if let Some(a) = a {
                        if a != title {
                            eprintln!(
                                "Alias2 {} (for {}) shadowed by same alias2 for {}",
                                nick, a, title
                            );
                        }
                    }
                }
                let nick = nick
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>();
                if !nick.is_empty() {
                    let a = nicknames_alphanumeric_only.insert(nick.to_string(), title.to_string());
                    if let Some(a) = a {
                        if a != title {
                            eprintln!(
                                "Alias3 {} (for {}) shadowed by same alias3 for {}",
                                nick, a, title
                            );
                        }
                    }
                }
                let nick = nick.chars().filter(|c| c.is_ascii()).collect::<String>();
                if !nick.is_empty() {
                    let a = nicknames_alphanumeric_and_ascii
                        .insert(nick.to_string(), title.to_string());
                    if let Some(a) = a {
                        if a != title {
                            eprintln!(
                                "Alias4 {} (for {}) shadowed by same alias4 for {}",
                                nick, a, title
                            );
                        }
                    }
                }
            }
        }
    }

    // Set community aliases
    let mut community_aliases = HashMap::<GuildId, MainAliases<(String, String)>>::new();
    let file = File::open(format!("./data/aliases/manual/{}.tsv", game)).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines.map_while(Result::ok) {
        let split = line.split('\t');
        let split = split.collect::<Vec<_>>();
        assert!(
            split.len() == 5 || split.len() == 1,
            "Community alias parse fail for line `{}`",
            line
        );
        if split.len() == 1 {
            continue;
        }
        let title = split[0];
        let nickname = split[1];
        let uploader_id = split[2];
        let uploader_dscrm = split[3];
        let server_id = GuildId(split[4].parse::<u64>().unwrap());

        let server_aliases_map = community_aliases.get_mut(&server_id);
        let server_aliases_map = if let Some(m) = server_aliases_map {
            m
        } else {
            let inserted = community_aliases.insert(server_id, MainAliases::default());
            assert!(inserted.is_none());
            community_aliases.get_mut(&server_id).unwrap()
        };

        let uploader_title_pair = (
            format!("{}#{}", uploader_id, uploader_dscrm),
            title.to_string(),
        );
        let nick = nickname
            .to_lowercase()
            .split_whitespace()
            .collect::<String>();
        if !nick.is_empty() {
            let a = server_aliases_map
                .nicknames_lowercased_and_unspaced
                .insert(nick.to_string(), uploader_title_pair.clone());
            if let Some(a) = a {
                if a.1 != title {
                    eprintln!(
                        "Alias2 {} (for {}) shadowed by same alias2 for {}",
                        nick, a.1, title
                    );
                }
            }
        }
        let nick = nick
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        if !nick.is_empty() {
            let a = server_aliases_map
                .nicknames_alphanumeric_only
                .insert(nick.to_string(), uploader_title_pair.clone());
            if let Some(a) = a {
                if a.1 != title {
                    eprintln!(
                        "Alias3 {} (for {}) shadowed by same alias3 for {}",
                        nick, a.1, title
                    );
                }
            }
        }
        let nick = nick.chars().filter(|c| c.is_ascii()).collect::<String>();
        if !nick.is_empty() {
            let a = server_aliases_map
                .nicknames_alphanumeric_and_ascii
                .insert(nick.to_string(), uploader_title_pair);
            if let Some(a) = a {
                if a.1 != title {
                    eprintln!(
                        "Alias4 {} (for {}) shadowed by same alias4 for {}",
                        nick, a.1, title
                    );
                }
            }
        }
    }

    // // I fucking hate myself but I don't have the energy to fix this
    // for (name0, title) in lowercased_and_unspaced.iter() {
    //     if lowercased.contains_key(name0) {
    //         // Don't delete this; it's for actual debugging!
    //         if title != &lowercased_and_unspaced[name0] {
    //             eprintln!(
    //                 "Alias0 {} (for {}) shadowed by same alias-1 for {}",
    //                 name0, title, lowercased_and_unspaced[name0]
    //             );
    //         }
    //     }
    // }
    // for (name1, title) in alphanumeric_only.iter() {
    //     if lowercased_and_unspaced.contains_key(name1) {
    //         // Don't delete this; it's for actual debugging!
    //         if title != &lowercased_and_unspaced[name1] {
    //             eprintln!(
    //                 "Alias1 {} (for {}) shadowed by same alias0 for {}",
    //                 name1, title, lowercased_and_unspaced[name1]
    //             );
    //         }
    //     }
    // }
    // for (name2, title) in alphanumeric_and_ascii.iter() {
    //     if alphanumeric_only.contains_key(name2) {
    //         // Don't delete this; it's for actual debugging!
    //         if title != &alphanumeric_only[name2] {
    //             eprintln!(
    //                 "Alias2 {} (for {}) shadowed by same alias1 for {}",
    //                 name2, title, alphanumeric_only[name2]
    //             );
    //         }
    //     }
    // }
    // for (nick, title) in nicknames_alphanumeric_and_ascii.iter() {
    //     if alphanumeric_and_ascii.contains_key(nick) {
    //         // Don't delete this; it's for actual debugging!
    //         if title != &alphanumeric_and_ascii[nick] {
    //             eprintln!(
    //                 "Alias3 {} (for {}) shadowed by same alias2 for {}",
    //                 nick, title, alphanumeric_and_ascii[nick]
    //             );
    //         }
    //     }
    // }
    // for (nick, title) in nicknames_alphanumeric_only.iter() {
    //     if alphanumeric_only.contains_key(nick) {
    //         // Don't delete this; it's for actual debugging!
    //         if title != &alphanumeric_only[nick] {
    //             eprintln!(
    //                 "Alias3 {} (for {}) shadowed by same alias2 for {}",
    //                 nick, title, alphanumeric_only[nick]
    //             );
    //         }
    //     }
    // }

    Ok(Aliases {
        main: MainAliases {
            original,
            lowercased,
            lowercased_and_unspaced,
            alphanumeric_only,
            alphanumeric_and_ascii,
            nicknames_lowercased_and_unspaced,
            nicknames_alphanumeric_only,
            nicknames_alphanumeric_and_ascii,
        },
        manual: community_aliases,
    })
}

/// TODO: REFACTOR PLEASE PLEASE PLEASE PLEASE PLEASE PLEASE PLEASE PLEASE
/// Returns (closest entry in aliases, original title of that entry)
pub fn get_closest_title(
    title: &str,
    all_aliases: &Aliases,
    server_id: GuildId,
) -> (String, String) {
    let mut candidates = vec![];
    let aliases = &all_aliases.main;
    let comm_aliases = all_aliases.manual.get(&server_id);

    // Returns ((closest entry in aliases, original title of that entry), closeness score)
    let f = |x: &HashMap<String, String>, title: &String| {
        let a = x
            .iter()
            .map(|x| (x, strsim::jaro_winkler(x.0, title)))
            .max_by_key(|x| OrderedFloat::from(x.1))?;
        Some(((a.0 .0.clone(), a.0 .1.clone()), a.1))
    };

    // Returns ((closest entry in aliases, original title of that entry), closeness score)
    let g = |alias_map: &HashMap<String, (String, String)>, title: &String| {
        let a = alias_map
            .iter()
            .map(|x| (x, strsim::jaro_winkler(x.0, title)))
            .max_by_key(|x| OrderedFloat::from(x.1))?;
        Some(((a.0 .0.clone(), a.0 .1 .1.clone()), a.1))
    };

    let push = |candidates: &mut Vec<((String, String), f64)>,
                close_entry: Option<((String, String), f64)>| {
        if let Some(close_entry) = close_entry {
            candidates.push(close_entry);
        }
    };

    push(&mut candidates, f(&aliases.original, &title.to_string()));
    let titlem1 = title.to_lowercase();
    push(&mut candidates, f(&aliases.lowercased, &titlem1));
    let title0 = titlem1.split_whitespace().collect::<String>();
    push(
        &mut candidates,
        f(&aliases.lowercased_and_unspaced, &title0),
    );
    let title1 = title0
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    push(&mut candidates, f(&aliases.alphanumeric_only, &title1));
    // push(&mut candidates, f(&aliases.nicknames_alphanumeric_only, &title1));
    let title2 = title1.chars().filter(|c| c.is_ascii()).collect::<String>();
    push(&mut candidates, f(&aliases.alphanumeric_and_ascii, &title2));
    // push(&mut candidates, f(&aliases.nicknames_alphanumeric_and_ascii, &title2));

    if let Some(comm_aliases) = comm_aliases {
        let titlem1 = title.to_lowercase();
        // push(&mut candidates, g(&comm_aliases.lowercased, &titlem1));
        let title0 = titlem1.split_whitespace().collect::<String>();
        // push(&mut candidates, g(&comm_aliases.lowercased_and_unspaced, &title0));
        push(
            &mut candidates,
            g(&comm_aliases.nicknames_lowercased_and_unspaced, &title0),
        );
        let title1 = title0
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        // push(&mut candidates, g(&comm_aliases.alphanumeric_only, &title1));
        push(
            &mut candidates,
            g(&comm_aliases.nicknames_alphanumeric_only, &title1),
        );
        let title2 = title1.chars().filter(|c| c.is_ascii()).collect::<String>();
        // push(&mut candidates, g(&comm_aliases.alphanumeric_and_ascii, &title2));
        push(
            &mut candidates,
            g(&comm_aliases.nicknames_alphanumeric_and_ascii, &title2),
        );
    }

    let a = &candidates
        .iter()
        .max_by_key(|x| OrderedFloat::from(x.1))
        .unwrap()
        .0;
    (a.0.clone(), a.1.clone())
}

pub fn float_to_level(f: &str, game: Game) -> String {
    let f = f.parse::<f32>().unwrap().abs();
    let decimal = f - f.floor();

    let plus_border = match game {
        Game::Maimai => 6,
        Game::Chunithm => 5,
        Game::Ongeki => 7,
    } as f32
        * 0.1
        - 0.05;

    if game == Game::Maimai && f.floor() as usize <= 6 {
        return f.floor().to_string();
    }

    if decimal < plus_border {
        f.floor().to_string()
    } else {
        format!("{}+", f.floor())
    }
}

pub fn float_to_constant(f: &str) -> Option<OrderedFloat<f32>> {
    let f = OrderedFloat::from(
        f.parse::<f32>()
            .unwrap_or_else(|_| panic!("Failed parse on supposed float: \"{}\"", f)),
    );

    if f < (0.).into() {
        None
    } else {
        Some(f)
    }
}

pub fn constant_to_string(c: Option<OrderedFloat<f32>>) -> String {
    if let Some(s) = c {
        format!(" ({:.1})", s)
    } else {
        "".to_string()
    }
}

/// Returns true if guild id is registered in `data/cooldown-server-ids.txt`
/// and user cooldown has not yet passed.
pub async fn check_cooldown(_ctx: &Context<'_>) -> Cooldown {
    Cooldown::None

    // let guild_id = match ctx.guild_id() {
    //     Some(id) => id,
    //     None => return Cooldown::None,
    // };
    // let channel_id = ctx.channel_id();

    // if !ctx.data().cooldown_server_ids.contains(&guild_id) {
    //     return Cooldown::None;
    // }
    // if ctx
    //     .data()
    //     .cooldown_channel_exception_ids
    //     .contains(&channel_id)
    // {
    //     return Cooldown::None;
    // }

    // let mut map = ctx.data().timestamps.lock().await;
    // let (user_map, channel_map) = map.get_mut(&guild_id).unwrap();

    // let now = Timestamp::now().unix_timestamp();
    // let user_id = ctx.author().id;
    // let channel_id = ctx.channel_id();
    // let then = user_map.get(&user_id);
    // if let Some(then) = then {
    //     if now - then < USER_COOLDOWN {
    //         return Cooldown::User(USER_COOLDOWN - (now - then));
    //     }
    // }
    // let then = channel_map.get(&channel_id);
    // if let Some(then) = then {
    //     if now - then < CHANNEL_COOLDOWN {
    //         return Cooldown::Channel(CHANNEL_COOLDOWN - (now - then));
    //     }
    // }
    // user_map.insert(user_id, now);
    // channel_map.insert(channel_id, now);
    // Cooldown::None
}

/// Return corresponding index to difficulty - BASIC = 0, ADVANCED = 1, ...
pub fn diff_to_idx(diff: &str) -> usize {
    let strs = [
        vec!["BAS", "Basic", "basic"],
        vec!["ADV", "Advanced", "advanced"],
        vec!["EXP", "Expert", "expert"],
        vec!["MAS", "Master", "master"],
        vec!["REM", "Lunatic", "ULT", "ultima", "remaster"],
    ];
    for (i, st) in strs.iter().enumerate() {
        if st.contains(&diff) {
            return i;
        }
    }
    panic!("{}", diff);
}

/////////////////////// maimai utils ///////////////////////

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct MaiDifficulty {
    pub st: Option<Difficulty>,
    pub dx: Option<Difficulty>,
}

impl MaiDifficulty {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.st.is_some() || self.dx.is_some()
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub enum MaiCategory {
    PopAnime,
    NicoVoca,
    TouhouProject,
    GameVariety,
    Maimai,
    OngekiChuni,
    Utage,
    #[default]
    Error,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct MaiInfo {
    pub jp_lv: Option<MaiDifficulty>,
    pub intl_lv: Option<MaiDifficulty>,
    pub utages: Vec<Utage>,
    pub jp_jacket: Option<String>,
    pub title: String,
    pub artist: String,
    pub bpm: Option<OrderedFloat<f64>>,
    pub dx_sheets: Vec<MaiSheet>,
    pub st_sheets: Vec<MaiSheet>,
    pub version: Option<String>,
    pub deleted: bool,
    pub order: Option<usize>,
    pub category: MaiCategory,
    pub title_kana: String,
    pub additional_remas_version: Option<String>,
    pub additional_st_version: Option<String>,
    pub additional_dx_version: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Utage {
    pub level: String,
    pub kanji: String,
    pub comment: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaiSheet {
    pub designer: Option<String>,
    pub brk: usize,
    pub hold: usize,
    pub slide: usize,
    pub tap: usize,
    pub touch: usize,
}

#[allow(dead_code)]
fn mai_diff_string(dx: bool, diff: usize) -> String {
    let st_str = ["BASIC", "ADVANCED", "EXPERT", "MASTER", "Re:MAS"];
    let dx_str = ["DXBAS", "DXADV", "DXEXP", "DXMAS", "DXREM"];
    if dx { dx_str[diff] } else { st_str[diff] }.to_string()
}

/// Get category string from maimai-info
pub fn mai_get_category(s: &str) -> MaiCategory {
    if s == "POPS＆アニメ" {
        MaiCategory::PopAnime
    } else if s == "niconico＆ボーカロイド" {
        MaiCategory::NicoVoca
    } else if s == "東方Project" {
        MaiCategory::TouhouProject
    } else if s == "ゲーム＆バラエティ" {
        MaiCategory::GameVariety
    } else if s == "maimai" {
        MaiCategory::Maimai
    } else if s == "オンゲキ＆CHUNITHM" {
        MaiCategory::OngekiChuni
    } else if s == "宴会場" {
        MaiCategory::Utage
    } else {
        panic!("Invalid maimai song category: {}", s)
    }
}

/////////////////////// chuni utils ///////////////////////

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ChuniInfo {
    pub jp_lv: Option<Difficulty>,
    pub intl_lv: Option<Difficulty>,
    pub jp_jacket: Option<String>,
    pub title: String,
    pub artist: String,
    pub bpm: Option<usize>,
    pub version: Option<String>,
    pub deleted: bool,
    pub category: ChuniCategory,
    /// Some("01, "01155") stands for https://sdvx.in/chunithm/01/01155mst.htm
    pub sdvxin: Option<(String, String)>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub enum ChuniCategory {
    PopsAnime,
    Niconico,
    TouhouProject,
    Variety,
    Irodori,
    GekiMai,
    Original,
    #[default]
    Error,
}

pub fn chuni_get_category(s: &str) -> ChuniCategory {
    if s == "POPS & ANIME" {
        ChuniCategory::PopsAnime
    } else if s == "niconico" {
        ChuniCategory::Niconico
    } else if s == "東方Project" {
        ChuniCategory::TouhouProject
    } else if s == "VARIETY" {
        ChuniCategory::Variety
    } else if s == "イロドリミドリ" {
        ChuniCategory::Irodori
    } else if s == "ゲキマイ" {
        ChuniCategory::GekiMai
    } else if s == "ORIGINAL" {
        ChuniCategory::Original
    } else {
        panic!("Invalid chuni song category")
    }
}

pub fn float_to_chuni_level(f: &str) -> String {
    let f = f.parse::<f32>().unwrap().abs();
    let decimal = f - f.floor();

    if decimal < 0.45 {
        f.floor().to_string()
    } else {
        format!("{}+", f.floor())
    }
}

/////////////////////// ongeki utils ///////////////////////

#[derive(Debug, Eq, PartialEq, Default)]
pub struct OngekiInfo {
    pub lv: Option<Difficulty>,
    pub jp_jacket: Option<String>,
    pub title: String,
    pub artist: String,
    pub date: usize,
    pub character: String,
    pub category: OngekiCategory,
    pub element: String,
    pub char_lv: usize,
    pub deleted: bool,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub enum OngekiCategory {
    Ongeki,
    PopsAnime,
    Niconico,
    TouhouProject,
    Variety,
    ChuMai,
    // BonusTrack,
    // Lunatic,
    #[default]
    Error,
}

pub fn ongeki_get_category(s: &str) -> OngekiCategory {
    if s == "オンゲキ" {
        OngekiCategory::Ongeki
    } else if s == "POPS＆ANIME" {
        OngekiCategory::PopsAnime
    } else if s == "niconico" {
        OngekiCategory::Niconico
    } else if s == "東方Project" {
        OngekiCategory::TouhouProject
    } else if s == "VARIETY" {
        OngekiCategory::Variety
    } else if s == "チュウマイ" {
        OngekiCategory::ChuMai
    } else {
        panic!("Invalid ongeki song category {}", s)
    }
}

fn get_jp_jacket(ctx: Context<'_>, game: Game, title: &str) -> Option<String> {
    match game {
        Game::Maimai => ctx.data().mai_charts[title].jp_jacket.clone(),
        Game::Chunithm => ctx.data().chuni_charts[title].jp_jacket.clone(),
        Game::Ongeki => ctx.data().ongeki_charts[title].jp_jacket.clone(),
    }
}

fn get_url_prefix(ctx: Context<'_>, game: Game) -> String {
    match game {
        Game::Maimai => ctx.data().mai_jacket_prefix.clone(),
        Game::Chunithm => ctx.data().chuni_jacket_prefix.clone(),
        Game::Ongeki => ctx.data().ongeki_jacket_prefix.clone(),
    }
}

fn get_aliases(ctx: Context<'_>, game: Game) -> &Aliases {
    match game {
        Game::Maimai => &ctx.data().mai_aliases,
        Game::Chunithm => &ctx.data().chuni_aliases,
        Game::Ongeki => &ctx.data().ongeki_aliases,
    }
}

pub async fn jacket_template(ctx: Context<'_>, title: String, game: Game) -> eyre::Result<()> {
    // Get alias corresponding to game.
    let aliases_template = get_aliases(ctx, game);

    // Check if title is in alias list.
    let actual_title = get_title(
        &title,
        aliases_template,
        ctx.guild_id()
            .unwrap_or(poise::serenity_prelude::GuildId(0)),
    );

    // If title is not in alias list, get closest alias to title and show button.
    if actual_title.is_none() {
        let mut log = ctx.data().alias_log.lock().await;
        let closest = get_closest_title(
            &title,
            aliases_template,
            ctx.guild_id()
                .unwrap_or(poise::serenity_prelude::GuildId(0)),
        );
        writeln!(log, "{}\t{:?}\t{}\t{}", title, game, closest.0, closest.1)?;
        log.sync_all()?;
        drop(log);
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
                aliases_template,
                ctx.guild_id()
                    .unwrap_or(poise::serenity_prelude::GuildId(0)),
            )
            .unwrap();
            mci.create_interaction_response(&serenity_ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        let jacket = get_jp_jacket(ctx, game, &actual_title);
                        if let Some(jacket) = jacket {
                            d.content(format!("Query by <@{}>", ctx.author().id))
                                .add_file(AttachmentType::Image(
                                    url::Url::parse(&format!(
                                        "{}{}",
                                        get_url_prefix(ctx, game),
                                        jacket
                                    ))
                                    .unwrap(),
                                ));
                        }
                        d
                    })
            })
            .await?;
        }
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
                aliases_template,
                ctx.guild_id()
                    .unwrap_or(poise::serenity_prelude::GuildId(0)),
            )
            .unwrap();
            mci.create_interaction_response(&serenity_ctx.http, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        let jacket = get_jp_jacket(ctx, game, &actual_title);
                        if let Some(jacket) = jacket {
                            d.content(format!("Query by <@{}>", ctx.author().id))
                                .add_file(AttachmentType::Image(
                                    url::Url::parse(&format!(
                                        "{}{}",
                                        get_url_prefix(ctx, game),
                                        jacket
                                    ))
                                    .unwrap(),
                                ));
                        }
                        d
                    })
            })
            .await?;
        }
        return Ok(());
    }
    let title = actual_title.unwrap();
    let jacket = get_jp_jacket(ctx, game, &title);
    if let Some(jacket) = jacket {
        ctx.send(|f| {
            f.attachment(AttachmentType::Image(
                url::Url::parse(&format!("{}{}", get_url_prefix(ctx, game), jacket)).unwrap(),
            ))
        })
        .await?;
    }
    Ok(())
}

/// (description, jacket)
type GetEmbed =
    Arc<dyn Fn(String, &Context<'_>) -> eyre::Result<(String, Option<String>)> + Sync + Send>;

pub async fn info_template(
    ctx: Context<'_>,
    title: String,
    game: Game,
    get_embed: GetEmbed,
    color: (u8, u8, u8),
    duplicate_alias_to_title: Arc<dyn Fn(&String) -> String + Sync + Send>,
) -> eyre::Result<()> {
    let aliases = get_aliases(ctx, game);
    // let actual_title = get_title(
    //     &title,
    //     aliases_template,
    //     ctx.guild_id()
    //         .unwrap_or(poise::serenity_prelude::GuildId(0)),
    // );
    let actual_title = get_title(
        &title,
        aliases,
        ctx.guild_id()
            .unwrap_or(poise::serenity_prelude::GuildId(0)),
    );
    if actual_title.is_none() {
        let mut log = ctx.data().alias_log.lock().await;
        writeln!(log, "{}\t{:?}", title, game)?;
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
                                    get_embed(actual_title.to_string(), &ctx).unwrap();

                                let mut f = f
                                    .title(duplicate_alias_to_title(&actual_title))
                                    .description(description)
                                    .color(Color::from_rgb(color.0, color.1, color.2));
                                if let Some(jacket) = jacket {
                                    f = f.thumbnail(format!(
                                        "{}{}",
                                        get_url_prefix(ctx, game),
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

    match check_cooldown(&ctx).await {
        Cooldown::Channel(t) => {
            let is_slash_command = matches!(&ctx, poise::Context::Application(_));
            ctx.send(|f| {
                f.ephemeral(is_slash_command).content(format!(
                    "Channel cooldown: please wait {} seconds and try again, or try the #bot-commands channel for no cooldown.",
                    t
                ))
            })
            .await?;
            return Ok(());
        }
        Cooldown::User(t) => {
            if let poise::Context::Application(_) = &ctx {
                ctx.send(|f| {
                    f.ephemeral(true).content(format!(
                        "Channel cooldown: please wait {} seconds and try again, or try the #bot-commands channel for no cooldown.",
                        t
                    ))
                })
                .await?;
            }
            return Ok(());
        }
        Cooldown::None => (),
    }
    let title = actual_title.unwrap();
    let (description, jacket) = get_embed(title.clone(), &ctx)?;

    ctx.send(|f| {
        f.embed(|f| {
            let mut f = f
                .title(duplicate_alias_to_title(&title).replace('*', "\\*"))
                .description(description)
                .color(Color::from_rgb(color.0, color.1, color.2));
            if let Some(jacket) = jacket {
                f = f.thumbnail(format!("{}{}", get_url_prefix(ctx, game), jacket));
            }

            f
        })
    })
    .await?;
    Ok(())
}
