use gcm_bot::{chuni::*, utils::set_aliases};

fn main() {
    let m = set_chuni_charts().unwrap();
    set_aliases(m.keys(), "chuni").unwrap();
}
