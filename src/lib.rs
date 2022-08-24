pub mod chuni;
pub mod maimai;
pub mod ongeki;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fs::{self, File},
        io::{BufRead, BufReader},
        sync::Arc,
    };
    use tokio::sync::Mutex;

    use poise::serenity_prelude::{GuildId, UserId};

    use crate::{
        chuni::set_chuni_charts,
        maimai::set_mai_charts,
        ongeki::set_ongeki_charts,
        utils::{get_curl, set_aliases},
    };

    #[test]
    fn full_test() {
        let charts = set_mai_charts().unwrap();
        set_aliases(charts.keys(), "maimai").unwrap();
        let charts = set_chuni_charts().unwrap();
        set_aliases(charts.keys(), "chuni").unwrap();
        let charts = set_ongeki_charts().unwrap();
        set_aliases(charts.keys(), "ongeki").unwrap();
        let cooldown_server_ids = {
            let file = File::open("data/cooldown-server-ids.txt").unwrap();
            BufReader::new(file)
                .lines()
                .map(|l| l.unwrap().parse::<u64>())
                .filter(|b| b.is_ok())
                .map(|l| GuildId(l.unwrap()))
                .collect::<HashSet<_>>()
        };
        let _: Arc<Mutex<HashMap<GuildId, HashMap<UserId, i64>>>> = Arc::new(Mutex::new(
            cooldown_server_ids
                .iter()
                .map(|k| (*k, HashMap::new()))
                .collect(),
        ));

        let files_not_urls = [
            "data/intl-add.txt",
            "data/intl-del.txt",
            "data/jp-del.txt",
            "data/maimai-jacket-prefix.txt",
            "data/maimai-manual-add.txt",
            "jp_lv.csv",
            "data/cooldown-server-ids.txt",
            "data/cooldown-channel-exception-ids.txt",
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
