pub mod maimai;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::maimai::{set_mai_aliases, set_mai_charts};

    #[test]
    fn full_test() {
        set_mai_aliases().unwrap();
        set_mai_charts().unwrap();
        File::open("data/intl-del.txt").unwrap();
        File::open("data/jp-del.txt").unwrap();
        File::open("data/maimai-info.txt").unwrap();
        File::open("data/maimai-intl.txt").unwrap();
        File::open("data/maimai-jacket-prefix.txt").unwrap();
        File::open("data/maimai-jp.txt").unwrap();
        File::open("data/ongeki-url.txt").unwrap();
        File::open("in_lv.csv").unwrap();
    }
}