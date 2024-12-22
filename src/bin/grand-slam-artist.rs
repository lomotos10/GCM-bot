use std::collections::BTreeSet;

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

    let gl: BTreeSet<_> = g
        .iter()
        .filter(|(_, info)| info.category == OngekiCategory::Ongeki)
        .map(|(_, info)| &info.artist)
        .collect();
    let cl: BTreeSet<_> = c
        .iter()
        .filter(|(_, info)| {
            info.category == ChuniCategory::Original || info.category == ChuniCategory::Irodori
        })
        .map(|(_, info)| &info.artist)
        .collect();
    let ml: BTreeSet<_> = m
        .iter()
        .filter(|(_, info)| info.category == MaiCategory::Maimai)
        .map(|(_, info)| &info.artist)
        .collect();

    let mut l = gl.clone();
    l.append(&mut cl.clone());
    l.append(&mut ml.clone());

    let v = l.iter().map(|artist| {
        (
            artist,
            (gl.contains(artist) as usize),
            (cl.contains(artist) as usize),
            (ml.contains(artist) as usize),
        )
    });
    for i in v {
        println!("{}\t{}\t{}\t{}", i.0, i.1, i.2, i.3);
    }
}
