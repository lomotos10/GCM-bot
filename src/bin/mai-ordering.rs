use gcm_bot::maimai::*;

fn main() {
    let m = set_mai_charts().unwrap();
    let mut m = m
        .iter()
        .map(|i| (i.0, i.1.order.unwrap_or(999)))
        .collect::<Vec<_>>();
    m.sort_by(|a, b| a.1.cmp(&b.1));
    for i in m {
        println!("{}\t{}", i.0, i.1);
    }
}

// use gcm_bot::maimai::*;

// fn main() {
//     let m = set_mai_charts().unwrap();
//     let lv = "14+".to_string();
//     let mut v = vec![];
//     for song in m {
//         if song.1.deleted {
//             continue;
//         }
//         if let Some(a) = song.1.intl_lv {
//             if let Some(b) = a.dx {
//                 if b.mas == lv {
//                     v.push((song.0.clone(), song.1.order, "DX", "MASTER"));
//                 }
//                 if b.extra == Some(lv.clone()) {
//                     v.push((song.0.clone(), song.1.order, "DX", "Re:MASTER"));
//                 }
//             }
//             if let Some(b) = a.st {
//                 if b.mas == lv {
//                     v.push((song.0.clone(), song.1.order, "ST", "MASTER"));
//                 }
//                 if b.extra == Some(lv.clone()) {
//                     v.push((song.0.clone(), song.1.order, "ST", "Re:MASTER"));
//                 }
//             }
//         }
//     }
//     v.sort_by(|a, b| a.1.cmp(&b.1));
//     for i in v {
//         let title = urlencoding::encode(&i.0);
//         println!("{}\t{}\t{}\thttps://www.youtube.com/results?search_query=maimai+{}+{}", i.0, i.2, i.3, title, i.3);
//     }
// }
