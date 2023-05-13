use gcm_bot::{
    chuni::set_chuni_charts,
    maimai::set_mai_charts,
    ongeki::set_ongeki_charts,
    utils::{ChuniCategory, MaiCategory, OngekiCategory},
};

fn main() {
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

    gl.sort_by_key(|a| a.to_lowercase());
    cl.sort_by_key(|a| a.to_lowercase());
    ml.sort_by_key(|a| a.to_lowercase());
    println!("ONGEKI: ({})", gl.len());
    for t in gl {
        println!("{}", t);
    }
    println!("\nCHUNITHM: ({})", cl.len());
    for t in cl {
        println!("{}", t);
    }
    println!("\nMAIMAI: ({})", ml.len());
    for t in ml {
        println!("{}", t);
    }
}
