use gcm_bot::{maimai::*, utils::set_aliases};

fn main() {
    let m = set_mai_charts().unwrap();
    set_aliases(m.keys(), "maimai").unwrap();
    // println!("{:#?}", m);
}
