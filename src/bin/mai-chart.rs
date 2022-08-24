use gcm_bot::{maimai::*, utils::set_aliases};

fn main() {
    let m = set_mai_charts().unwrap();
    // println!("{:#?}", m);
    // for song in m {
    //     if song.1.deleted {
    //         println!("{}", song.0);
    //     }
    // }
    set_aliases(m.keys(), "maimai").unwrap();
}
