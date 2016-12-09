
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate itertools;

use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct IP<>(Vec<String>, Vec<String>);
impl FromStr for IP {
    type Err = String;
    fn from_str<>(s: &str) -> Result<IP, String> {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new("(?:([a-z]+)(?:\\[([a-z]+)\\])?)").unwrap();
        }
        let mut ip = IP(Vec::new(), Vec::new());
        for cap in RE.captures_iter(s) {
            if let Some(s) = cap.at(1) {
                ip.0.push(s.to_string());
            }
            if let Some(s) = cap.at(2) {
                ip.1.push(s.to_string());
            }
        }
        Ok(ip)
    }
}

fn is_abba((&a, &b, &c, &d): (&char, &char, &char, &char)) -> bool {
    a == d && b == c  && a != b
}

fn has_abba_block(s: &String) -> bool {
    let v : Vec<_> = s.chars().collect();
    for c in v.windows(4) {
        if is_abba(c.iter().next_tuple().unwrap()) {
            return true;
        }
    }
    false
}

fn supports_tls(s: &str) -> bool {
    let IP(v1, v2) = IP::from_str(s).expect(&format!("Invalid IP: {}", s));
    v1.iter().any(has_abba_block) && !v2.iter().any(has_abba_block)
}

fn aba_blocks(s: &str, rev: bool, blocks: &mut HashSet<(char, char)>) {
    let v : Vec<_> = s.chars().collect();
    for c in v.windows(3) {
        let (&a, &b, &c) = c.iter().next_tuple().unwrap();
        if a == c && a != b {
            blocks.insert(if rev {(a, b)} else {(b, a)});
        }
    }
}

fn supports_ssl(s: &str) -> bool{
    let IP(v1, v2) = IP::from_str(s).expect(&format!("Invalid IP: {}", s));
    let mut supernet = HashSet::new();
    for w in v1 {
        aba_blocks(&w, false, &mut supernet);
    }

    let mut hypernet = HashSet::new();
    for w in v2 {
        aba_blocks(&w, true, &mut hypernet);
    }
    !supernet.is_disjoint(&hypernet)
}

fn main() {
    let f = std::fs::File::open("input").expect("Unable to open input file");
    let lines: Vec<_> =  std::io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();
    let count_tls = lines.iter().filter(|l| supports_tls(l)).count();
    println!("Valid TLS count: {}", count_tls);

    let count_ssl = lines.iter().filter(|l| supports_ssl(l)).count();
    println!("Valid SSL count: {}", count_ssl);
}
