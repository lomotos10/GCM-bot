use std::fs;

use gcm_bot::{ongeki::set_ongeki_charts, utils::get_curl};

fn main() {
    // let m = set_ongeki_charts().unwrap();
    // println!("{:#?}", m);
    // set_aliases(m.keys(), "chuni").unwrap();

    let url = fs::read_to_string("data/ongeki-lv.txt").unwrap();
    let url = url.trim();
    println!("{}", url);
    let s = get_curl(url);
    println!("{}", s);
}
