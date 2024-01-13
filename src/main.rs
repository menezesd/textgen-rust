use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

use lazy_static::lazy_static;
use std::sync::Mutex;

const NPREF: usize = 2;
const NONWORD: &str = "\n";
const MAXGEN: usize = 10000;

type Prefix = VecDeque<String>;

lazy_static! {
    static ref STATETAB: Mutex<HashMap<Prefix, Vec<String>>> = Mutex::new(HashMap::new());
}

fn build(prefix: &mut Prefix, reader: impl BufRead) {
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            add(prefix, word.to_string());
        }
    }
}

fn add(prefix: &mut Prefix, s: String) {
    if prefix.len() == NPREF {
        STATETAB
            .lock()
            .unwrap()
            .entry(prefix.clone())
            .or_insert_with(Vec::new)
            .push(s.clone());
        prefix.pop_front();
    }
    prefix.push_back(s);
}

fn generate(nwords: usize) {
    let mut prefix = Prefix::new();

    for _ in 0..NPREF {
        add(&mut prefix, NONWORD.to_string());
    }

    for _ in 0..nwords {
        if let Some(suf) = STATETAB.lock().unwrap().get(&prefix) {
            let w = suf[rand::thread_rng().gen_range(0..suf.len())].clone();
            if w == NONWORD {
                break;
            }
            println!("{}", w);
            prefix.pop_front();
            prefix.push_back(w);
        }
    }
}

fn main() {
    let nwords = MAXGEN;
    let mut prefix = Prefix::new();

    for _ in 0..NPREF {
        add(&mut prefix, NONWORD.to_string());
    }

    let stdin = io::stdin();
    let reader = stdin.lock();
    build(&mut prefix, reader);

    add(&mut prefix, NONWORD.to_string());
    generate(nwords);
}
