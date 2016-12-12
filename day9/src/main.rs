#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::str::FromStr;
use std::io::Read;

#[derive(Debug)]
enum IngestionState {
    Take,
    Duplicate {
        occ: usize,
        rem: usize,
        buf: String
    }
}

impl IngestionState {
    fn from_marker(s: &str) -> IngestionState {
        let (occ, rep) = s.split_at(s.find('x').unwrap());
        Duplicate {
            rem: usize::from_str(&occ[1..]).unwrap(),
            occ: usize::from_str(&rep[1..(rep.len()-1)]).unwrap(),
            buf: String::new()
        }
    }
}

#[derive(Debug)]
struct Decompressor {
    decompressed: String,
    state: IngestionState,
}

#[derive(Debug)]
enum BlockType {
    Word,
    Marker
}
fn split(compressed: &str) -> Vec<(BlockType, &str)> {
    let mut blocks = Vec::new();
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("(\\w+)?(\\(\\w+\\))?").unwrap();
    }
    for cap in RE.captures_iter(compressed) {
        if let Some(s) = cap.at(1) {
            blocks.push((BlockType::Word, s));
        }
        if let Some(s) = cap.at(2) {
            blocks.push((BlockType::Marker, s));
        }
    }
    blocks
}


use IngestionState::*;
use BlockType::*;

fn ingest_block(decomp:  &mut Decompressor, block: (BlockType, &str)) {
    let mut switch_to_take = false;
    match (&mut decomp.state, block.0) {
        (&mut Take, Word) =>
            decomp.decompressed += block.1,

        (s @ &mut Take, Marker) =>
            *s = IngestionState::from_marker(block.1),

        (&mut Duplicate {ref occ, ref mut rem, ref mut buf}, _) => {
            if block.1.len() >= *rem {
                let (dupe, simple) = block.1.split_at(*rem);
                *buf += dupe;
                for _ in 0..*occ {
                    decomp.decompressed += buf;
                }
                decomp.decompressed += simple;
                switch_to_take = true;
            } else {
                *buf += block.1;
                *rem -= block.1.len();
            }
        }
    }
    if switch_to_take {
        decomp.state = Take;
    }
}

fn decompress(s: &str) -> String {
    let mut decompressor = Decompressor {decompressed: String::new(), state: Take};
    for b in split(s) {
        ingest_block(&mut decompressor, b)
    }
    decompressor.decompressed
}

fn main() {
    let mut f = std::fs::File::open("input").expect("Unable to open input file");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("Unable to read file");

    println!("{:?}", decompress(&input).len());
}
