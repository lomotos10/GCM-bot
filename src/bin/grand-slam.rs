use std::collections::HashMap;

use gcm_bot::{
    chuni::set_chuni_charts,
    maimai::set_mai_charts,
    ongeki::set_ongeki_charts,
    utils::{ChuniCategory, MaiCategory, OngekiCategory},
};
use itertools::Itertools;

fn main() {
    let substitutes = [
        ("Catch Me If You Can", "Catch Me If You Can(SEGA)"),
        ("Climax", "Climax(CHUNITHM)"),
        ("TEmPTaTiON", "TEmPTaTiON(maimai 시리즈)"),
        ("ジングルベル", "Jingle Bells#s-6.2"),
        ("夢花火", "夢花火(maimai 시리즈)"),
        ("Ring", "Ring(CHUNITHM)"),
        ("Regulus", "Regulus(SEGA)"),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect::<HashMap<_, _>>();

    let print = |name: &str| -> String {
        substitutes
            .get(name)
            .map(|value| format!("{}|{}", value, name))
            .unwrap_or_else(|| name.to_string())
    };

    let g = set_ongeki_charts().unwrap();
    let c = set_chuni_charts().unwrap();
    let m = set_mai_charts().unwrap();

    let mut gl = vec![];
    let mut cl = vec![];
    let mut ml = vec![];
    for (title, _) in g.iter() {
        if title == "Jörqer" {
            continue;
        }
        if c.contains_key(title) && m.contains_key(title) {
            // not using `else if` is intended - we don't want another jorqer situation
            if g[title].category == OngekiCategory::Ongeki {
                gl.push(title);
            }
            if c[title].category == ChuniCategory::Original
                || c[title].category == ChuniCategory::Irodori
            {
                cl.push(title);
            }
            if m[title].category == MaiCategory::Maimai {
                ml.push(title);
            }
        }
    }

    let tiamat = "TiamaT:F minor".to_string();
    cl.push(&tiamat);

    gl.sort_by_key(|&a| &m[a].title_kana);
    cl.sort_by_key(|&a| &m[a].title_kana);
    ml.sort_by_key(|&a| &m[a].title_kana);
    const COLUMNS: usize = 4;

    let print_chunk = |list: Vec<&String>| {
        for (idx, chunk) in list.chunks(COLUMNS).enumerate() {
            let st = Itertools::intersperse(
                chunk.iter().map(|title| {
                    if idx == 0 {
                        format!("<width=25%> [[{}]] ", print(title))
                    } else {
                        format!(" [[{}]] ", print(title))
                    }
                }),
                "||".into(),
            )
            .join("");

            println!("||{st}||");
        }
    };

    println!("ONGEKI: ({})", gl.len());
    print_chunk(gl);
    println!("\nCHUNITHM: ({})", cl.len());
    print_chunk(cl);
    println!("\nMAIMAI: ({})", ml.len());
    print_chunk(ml);
}
