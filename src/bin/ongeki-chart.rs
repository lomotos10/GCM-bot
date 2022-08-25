use gcm_bot::{ongeki::set_ongeki_charts, utils::set_aliases};

fn main() {
    let m = set_ongeki_charts().unwrap();
    set_aliases(m.keys(), "chuni").unwrap();
}
