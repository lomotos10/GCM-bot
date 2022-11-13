use gcm_bot::{maimai::*};

fn main() {
    let m = set_mai_charts().unwrap();
    let mut m = m.iter().map(|i| (i.0, i.1.order)).collect::<Vec<_>>();
    m.sort_by(|a, b| a.1.cmp(&b.1));
    for i in m {
        println!("{}", i.0);
    }
}
