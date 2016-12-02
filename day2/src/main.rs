use std::cmp::min;
use std::cmp::max;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


// Quizz 1
static KEYPAD_WIDTH : i8 = 3;

#[derive(Debug)]
struct Position(i8, i8);

fn key(p: &Position) -> i8 {
    return p.0 + p.1 * KEYPAD_WIDTH + 1
}

fn next_pos(p: &Position, m: char, max_width: i8) -> Position {
    let mut x = p.0;
    let mut y = p.1;
    match m {
        'U' => y = max(0, p.1 - 1),
        'D' => y = min(max_width - 1, p.1 + 1),
        'L' => x = max(0, p.0 - 1),
        'R' => x = min(max_width - 1, p.0 + 1),
        m => println!("Unknown char: {:?}", m),
    }
    Position(x, y)
}


// Quizz 2
static DIGIPAD : [[std::option::Option<i8>; 5]; 5] =
                        [[None, None, Some(1), None, None],
                        [None, Some(2), Some(3), Some(4), None],
                        [Some(5), Some(6), Some(7), Some(8), Some(9)],
                        [None, Some(10), Some(11), Some(12), None],
                        [None, None, Some(13), None, None]];

fn key2(p: &Position) -> Option<String> {
    if let Some(&Some(i)) = DIGIPAD.get(p.1 as usize).and_then(|l| l.get(p.0 as usize)) {
        return Some(format!("{:X}", i));
    }
    None
}

fn next_pos2(p: Position, m: char, max_width: i8) -> Position {
    let next1 = next_pos(&p, m, max_width);
    match key2(&next1) {
        Some(_) => next1,
        _ => p
    }
}

fn main() {
    if let Ok(f) = File::open("input.txt") {
        for l in BufReader::new(f).lines() {
            let mut pos = Position(1, 1);
            for c in l.unwrap().chars() {
                pos = next_pos(&mut pos, c, KEYPAD_WIDTH);
            }
            print!("{:?}", key(&pos));
        }
        println!("\n-----");
    }

    if let Ok(f) = File::open("input.txt") {
        for l in BufReader::new(f).lines() {
            let mut pos = Position(0, 2);
            for c in l.unwrap().chars() {
                pos = next_pos2(pos, c, 5);
            }
            print!("{}", key2(&pos).unwrap());
        }
        println!("\n-----");
    }
}
