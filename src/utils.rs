use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use strsim::jaro_winkler;

/////////////////////// General utils ///////////////////////

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub mai_charts: HashMap<String, MaiInfo>,
    pub mai_aliases: Aliases,
    pub mai_jacket_prefix: String,

    pub cooldown_server_ids: HashSet<String>,
    pub user_timestamp: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
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

pub struct Aliases {
    pub lowercased: HashMap<String, String>,
    pub lowercased_and_unspaced: HashMap<String, String>,
    pub alphanumeric_only: HashMap<String, String>,
    pub alphanumeric_and_ascii: HashMap<String, String>,
    pub nicknames: HashMap<String, String>,
}

pub fn serdest_to_string(st: &serde_json::Value) -> String {
    if let serde_json::Value::String(s) = st {
        s.to_string()
    } else {
        panic!()
    }
}

pub fn serdest_to_usize(st: &serde_json::Value) -> usize {
    if let serde_json::Value::Number(s) = st {
        s.as_u64().unwrap() as usize
    } else {
        panic!()
    }
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

pub fn get_title(title: &str, aliases: &Aliases) -> Option<String> {
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
    if let Some(a) = aliases.nicknames.get(&title2) {
        return Some(a.to_string());
    }
    None
}

pub fn get_closest_title(title: &str, aliases: &Aliases) -> (String, String) {
    let mut candidates = vec![];

    let f = |x: &HashMap<String, String>, title: &String| {
        let a = x
            .iter()
            .map(|x| (x, OrderedFloat(jaro_winkler(x.0, title))))
            .max_by_key(|x| x.1)
            .unwrap();
        ((a.0 .0.clone(), a.0 .1.clone()), a.1)
    };

    let titlem1 = title.to_lowercase();
    candidates.push(f(&aliases.lowercased, &titlem1));
    let title0 = titlem1.split_whitespace().collect::<String>();
    candidates.push(f(&aliases.lowercased_and_unspaced, &title0));
    let title1 = title0
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    candidates.push(f(&aliases.alphanumeric_only, &title1));
    let title2 = title1.chars().filter(|c| c.is_ascii()).collect::<String>();
    candidates.push(f(&aliases.alphanumeric_and_ascii, &title2));
    candidates.push(f(&aliases.nicknames, &title2));

    let a = &candidates.iter().max_by_key(|x| (*x).1).unwrap().0;
    (a.0.clone(), a.1.clone())
}

pub fn float_to_level(f: &str) -> String {
    let f = f.parse::<f32>().unwrap().abs();
    let decimal = f - f.floor();

    if decimal < 0.65 {
        f.floor().to_string()
    } else {
        format!("{}+", f.floor())
    }
}

pub fn float_to_constant(f: &str) -> Option<OrderedFloat<f32>> {
    let f = OrderedFloat::from(f.parse::<f32>().unwrap());

    if f <= (0.).into() {
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

/////////////////////// maimai utils ///////////////////////

#[derive(Debug, Eq, PartialEq, Default)]
pub struct MaiDifficulty {
    pub st: Option<Difficulty>,
    pub dx: Option<Difficulty>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct MaiInfo {
    pub jp_lv: Option<MaiDifficulty>,
    pub intl_lv: Option<MaiDifficulty>,
    pub jp_jacket: Option<String>,
    pub title: String,
    pub artist: String,
    pub bpm: Option<usize>,
    pub dx_sheets: Vec<MaiSheet>,
    pub st_sheets: Vec<MaiSheet>,
    pub version: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaiSheet {
    pub brk: usize,
    pub hold: usize,
    pub slide: usize,
    pub tap: usize,
    pub touch: usize,
}
