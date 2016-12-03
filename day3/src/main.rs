#![feature(slice_patterns)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::str::FromStr;

fn parse_line(line:String) -> Vec<u16> {
    line.split_whitespace()
        .map(|s| u16::from_str(s).unwrap())
        .collect()
}

fn is_valid_triangle(sides: &Vec<u16>) -> bool {
    if let [a, b, c] = *sides.as_slice() {
        return (a + b) > c && (a + c) > b && (b + c) > a;
    }
    false
}

fn main() {
    if let Ok(f) = File::open("input") {
        let valid: usize = BufReader::new(f).lines()
                           .map(|l| parse_line(l.unwrap()))
                           .filter(is_valid_triangle)
                           .count();
       println!("Possible triangles: {:?}", valid);
    }
}
