use gcm_bot::{chuni::set_chuni_charts, maimai::set_mai_charts, ongeki::set_ongeki_charts};

fn main() {
    let g = set_ongeki_charts().unwrap();
    let c = set_chuni_charts().unwrap();
    let m = set_mai_charts().unwrap();

    let mut l = vec![];
    for (title, song) in g.iter() {
        if c.contains_key(title)
            && m.contains_key(title)
            && (song.category == "オンゲキ" || song.category == "チュウマイ")
        {
            l.push(title);
        }
    }

    l.sort_by_key(|a| a.to_lowercase());
    for t in l {
        println!("{}", t);
    }
}
