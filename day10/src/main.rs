#![allow(dead_code)]

use std::collections::{VecDeque, HashMap};
use std::convert::From;
use std::str::FromStr;
use std::io::BufRead;

use Instruction::*;
#[derive(Debug)]
enum Instruction {
    Assign {
        bot: usize,
        value: usize,
    },
    Give {
        from: usize,
        to: Transfer
    }
}

#[derive(Debug)]
struct Transfer {
    low: (String, usize),
    high: (String, usize)
}

#[derive(Debug, Default)]
struct Bot {
    values: VecDeque<usize>
}

impl Bot {
    fn add(&mut self, value: usize) {
        match self.values.len() {
            0 =>  self.values.push_back(value),
            1 if self.values[0] < value => self.values.push_back(value),
            1 => self.values.push_front(value),
            _ =>  panic!(format!("No space left for bot to add {}", value))
        }
    }

    fn extract(&mut self) -> (usize, usize) {
        let (low, high) = (self.values[0], self.values[1]);
        self.values.clear();
        (low, high)
    }
}

fn to_int(s: &str) -> usize {
    usize::from_str(s).unwrap()
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let words : Vec<&str> = s.split_whitespace().collect();
        if words[0] == "value" {
            Assign {
                bot: to_int(words[5]),
                value: to_int(words[1])
            }
        }
        else {
            Give {
                from: to_int(words[1]),
                to: Transfer {
                    low: (words[5].to_string(), to_int(words[6])),
                    high: (words[10].to_string(), to_int(words[11])),
                }
            }
        }
    }
}

fn init(instructions: Vec<Instruction>) -> (HashMap<usize, Bot>, HashMap<usize, Transfer>) {
    let mut bots = HashMap::new();
    let mut transfers = HashMap::new();

    for inst in instructions {
        match inst {
            Assign{bot, value} => {bots.entry(bot).or_insert(Bot::default()).add(value);},
            Give{from, to} => {transfers.insert(from, to);}
        }
    }
    (bots, transfers)
}

fn transfer(bots: &mut HashMap<usize, Bot>,
            ready: &mut VecDeque<usize>,
            output: &mut HashMap<usize, Vec<usize>>,
            to: &(String, usize),
            val: usize) {
    if to.0 == "bot" {
        let ref mut b = bots.entry(to.1).or_insert(Bot::default());
        b.add(val);
        if b.values.len() == 2 {
            ready.push_front(to.1);
        }
    }
    else {
        output.entry(to.1).or_insert(Vec::new()).push(val);
    }

}

fn main() {
    // println!("{:?}", Instruction::from("value 2 goes to bot 156".to_string()));
    // println!("{:?}", Instruction::from("bot 37 gives low to bot 114 and high to bot 150".to_string()));
    // println!("{:?}", Instruction::from("bot 32 gives low to output 1 and high to bot 180".to_string()));

    let f = std::fs::File::open("input").expect("Unable to open input file");
    let instructions = std::io::BufReader::new(f)
        .lines()
        .map(|l| Instruction::from(l.unwrap()))
        .collect();
    let (mut bots, transfers) = init(instructions);
    let mut ready : VecDeque<usize> = bots.iter()
        .filter(|&(_, b)| b.values.len() == 2)
        .map(|(&i, _)| i)
        .collect();
    let mut output = HashMap::new();

    loop {
        if let Some(i) = ready.pop_back() {
            let (low, high) = bots.get_mut(&i).expect("no bot found").extract();

            if low == 17 && high == 61 {
                println!("********* Response bot for 1: {}", i);
            }

            let ref to = transfers[&i];
            transfer(&mut bots, &mut ready, &mut output, &to.low, low);
            transfer(&mut bots, &mut ready, &mut output, &to.high, high);
        }
        else {
            println!("No bots left ready to move.");
            break;
        }
    }
    println!("********* Product for 2 : {}", output[&0][0] * output[&1][0] * output[&2][0]);
}
