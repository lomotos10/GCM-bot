use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use gcm_bot::ongeki::set_ongeki_charts;

fn main() {
    let m = set_ongeki_charts().unwrap();
    // set_aliases(m.keys(), "chuni").unwrap();

    let mut chuni = HashMap::new();
    let mut mai = HashMap::new();

    let file = File::open("data/aliases/en/chuni.tsv").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let split = line.unwrap();
        let split = split.split('\t');
        let split = split.collect::<Vec<_>>();
        let title = split[0];

        let nickname_slice = split[1..].iter().map(|c| c.to_string()).collect::<Vec<_>>();
        chuni.insert(title.to_string(), nickname_slice);
    }
    let file = File::open("data/aliases/en/maimai.tsv").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let split = line.unwrap();
        let split = split.split('\t');
        let split = split.collect::<Vec<_>>();
        let title = split[0];

        let nickname_slice = split[1..].iter().map(|c| c.to_string()).collect::<Vec<_>>();
        mai.insert(title.to_string(), nickname_slice);
    }

    let mut m = m.iter().collect::<Vec<_>>();
    m.sort_by(|a, b| {
        let c = a.1.date.cmp(&b.1.date);
        if c == Ordering::Equal {
            a.0.cmp(b.0)
        } else {
            c
        }
    });

    for (title, _) in m {
        print!("{}", title);
        let mut v = vec![];
        if let Some(a) = mai.get(title) {
            for alias in a {
                if !v.contains(alias) {
                    v.push(alias.clone());
                }
            }
        }
        if let Some(a) = chuni.get(title) {
            for alias in a {
                if !v.contains(alias) {
                    v.push(alias.clone());
                }
            }
        }
        for a in v {
            print!("\t{}", a);
        }
        println!();
    }
}
