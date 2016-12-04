#![feature(slice_patterns)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::str::FromStr;

fn parse_line(line: String) -> Vec<u16> {
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
        let valid: usize = BufReader::new(f)
            .lines()
            .map(|l| parse_line(l.unwrap()))
            .filter(is_valid_triangle)
            .count();
        println!("Possible triangles by row: {:?}", valid);
    }

    let f = File::open("input").expect("Unable to open input file");
    let grouped: Vec<Vec<_>> = BufReader::new(f)
        .lines()
        .map(|l| parse_line(l.unwrap()))
        .fold(Vec::new(), |mut acc, l| {
            if acc.last().is_some() && acc.last().unwrap().len() < 3 {
                acc.last_mut().unwrap().push(l);
            } else {
                acc.push(vec![l]);
            }
            acc
        });

    let mut valid = 0;
    for group in grouped {
        if let &[a1, b1, c1] = group[0].as_slice() {
            if let &[a2, b2, c2] = group[1].as_slice() {
                if let &[a3, b3, c3] = group[2].as_slice() {
                    valid += [vec![a1, a2, a3], vec![b1, b2, b3], vec![c1, c2, c3]]
                        .iter()
                        .filter(|t| is_valid_triangle(t))
                        .count();
                }
            }
        }
    }
    println!("Possible triangles by column: {:?}", valid);
}
