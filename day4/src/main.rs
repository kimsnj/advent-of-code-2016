#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;
use std::result::Result;
use regex::Regex;

#[derive(Debug)]
struct Room {
    cyphertext: String,
    sector_id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = String;
    fn from_str(s: &str) -> Result<Room, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("((?:[:alpha:]+-)+)(\\d+)\\[(\\w+)\\]").unwrap();
        }
        let caps = try!(RE.captures(s).ok_or(format!("Input {} doesn't match room regex.", s)));
        Ok(Room {
            cyphertext: caps.at(1).unwrap().to_string(),
            sector_id: u32::from_str(caps.at(2).unwrap()).unwrap(),
            checksum: caps.at(3).unwrap().to_string(),
        })
    }
}

impl Room {
    fn compute_checksum(&self) -> String {
        let mut freq : Vec<(i16, char)> = self.cyphertext.replace("-", "")
            .chars()
            .fold(HashMap::new(), |mut freq, c| {
                *freq.entry(c).or_insert(0_i16) += 1;
                freq
            })
            .iter().map(|(&c, &count)| (- count, c))
            .collect();

        freq.sort();
        freq.iter()
            .take(5)
            .map(|&(_, c)| c)
            .collect()
    }

    fn is_valid(&self) -> bool {
        self.checksum == self.compute_checksum()
    }

    fn decode_char(c: char, id: u32) -> char {
        match c {
            '-' => ' ',
            c if c.is_alphabetic() => {
                let a = 'a' as u32;
                let next = c as u32 + id;
                ((next - a + 26) % 26 + a) as u8 as char
            }
            c => c
        }
    }

    fn decode(&self) -> String {
        self.cyphertext.chars()
            .map(|c| Self::decode_char(c, self.sector_id))
            .collect()
    }
}

fn main() {
    let f = std::fs::File::open("input").expect("Unable to open input file");
    let valid_rooms: Vec<_> = std::io::BufReader::new(f)
        .lines()
        .map(|l| Room::from_str(l.unwrap().trim()).unwrap())
        .filter(Room::is_valid)
        .collect();

    let sector_sum : u32 = valid_rooms.iter().map(|r| r.sector_id).sum();
    println!("Total of valid sector ids: {:?}", sector_sum);

    let pole: Vec<_> = valid_rooms.iter()
        .map(|r| (r.decode(), r.sector_id))
        .filter(|&(ref name, _)| name.contains("pole"))
        .collect();

    println!("Room names containing north pole: {:?}", pole);
}
