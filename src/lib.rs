pub mod maimai;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::fs::{self, File};

    use crate::{
        maimai::{set_mai_aliases, set_mai_charts},
        utils::get_curl,
    };

    #[test]
    fn full_test() {
        let charts = set_mai_charts().unwrap();
        set_mai_aliases(&charts).unwrap();

        let files_not_urls = [
            "data/intl-del.txt",
            "data/jp-del.txt",
            "data/maimai-jacket-prefix.txt",
            "in_lv.csv",
            "jp_lv.csv",
        ];
        let files_urls = [
            "data/maimai-info.txt",
            "data/maimai-intl.txt",
            "data/maimai-jp.txt",
            "data/ongeki-url.txt",
            "data/chuni-url.txt",
        ];

        for s in files_not_urls {
            File::open(s).unwrap();
        }
        for s in files_urls {
            let url = fs::read_to_string(s).unwrap();
            get_curl(&url);
        }
    }
}
