use std::str::FromStr;

use std::io::Read;
use std::fs::File;

// Input types and decoding
//
#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left = 1,
    Forward = 0,
    Right = -1,
}

#[derive(Debug)]
struct Step {
    rotation: Rotation,
    distance: i32,
}

impl FromStr for Rotation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => Ok(Rotation::Left),
            Some('R') => Ok(Rotation::Right),
            c => Err(format!("Unknown entry: {:?}", c)),
        }
    }
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);
        Ok(Step {
            rotation: Rotation::from_str(dir).unwrap(),
            distance: i32::from_str(dist.trim()).unwrap(),
        })
    }
}

// Common helpers
//
#[derive(Debug)]
struct Position {
    direction: i8,
    x: i32, // Distance in the East - West direction
    y: i32, // Distance in the South - North direction
}

impl Position {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn walk(pos: Position, step: Step) -> Position {
    let int_dir = (pos.direction + step.rotation as i8 + 4) % 4;
    let (x, y) = match int_dir {
        0 => (pos.x, pos.y + step.distance),
        1 => (pos.x - step.distance, pos.y),
        2 => (pos.x, pos.y - step.distance),
        3 => (pos.x + step.distance, pos.y),
        _ => panic!("Unknown direction {}", int_dir),
    };

    Position {direction: int_dir, x: x, y: y}
}


// Puzzle 1
//
fn process(input: &String) -> Position {
    let start = Position {direction: 0, x: 0, y: 0};
    input.split(", ")
         .map(|s| Step::from_str(s).unwrap())
         .fold(start, walk)
}

// Puzzle 2
//
fn split_step(step: Step) -> Vec<Step> {
    let mut v = Vec::new();
    v.push(Step {rotation: step.rotation, distance: 1});
    for _ in 1..step.distance {
        v.push(Step {rotation: Rotation::Forward, distance: 1});
    }
    v
}

fn find_crossing(input: &String) -> Position {
    let steps = input.split(", ")
        .map(|s| Step::from_str(s).unwrap())
        .flat_map(split_step);
    let mut pos = Position {direction: 0, x: 0, y:0};
    let mut visited : std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    visited.insert((0,0));
    for s in steps {
        pos = walk(pos, s);
        if visited.contains(&(pos.x, pos.y)) {
            return pos;
        }
        visited.insert((pos.x, pos.y));
    }
    panic!("No crossing found after visiting: {:?}", visited);
}

fn main() {
    if let Ok(mut f) = File::open("input.txt") {
        let mut input = String::new();
        if let Ok(_) = f.read_to_string(&mut input) {
            let end =  process(&input);
            println!("Final distance: {:?} at {:?}", end.distance(), end);
            let crossing = find_crossing(&input);
            println!("First crossing: {:?} at {:?}", crossing.distance(), crossing);
        }
    }
}
