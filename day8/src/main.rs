use std::str::FromStr;
use std::collections::HashSet;

use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y
}

static HEIGHT: u8 = 6;
static WIDTH: u8 = 50;

#[derive(Debug)]
enum Instruction {
    Rectangle(u8, u8),
    Rotate(Axis, u8, u8)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord(u8, u8);

impl Coord {
    fn get(& self, idx: Axis) -> u8 {
        match idx {
            Axis::X => self.0,
            Axis::Y => self.1
        }
    }

    fn rotate(&self, idx: Axis, dist: u8) -> Coord {
        let mut x = self.0;
        let mut y = self.1;
        match idx {
            Axis::X => y = (y + dist + HEIGHT) % HEIGHT,
            Axis::Y => x = (x + dist + WIDTH) % WIDTH,
        }
        Coord(x,y)
    }
}

fn parse(s: &str) -> Instruction {
    let mut words = s.split_whitespace();
    let action = words.next().expect("No actions in instruction");
    if action == "rect" {
        let rest = words.next().unwrap();
        let mut splitted = rest.split('x');
        return Instruction::Rectangle(
            u8::from_str(splitted.next().unwrap()).unwrap(),
            u8::from_str(splitted.next().unwrap()).unwrap());
    }
    else {
        let axis = if words.next() == Some("row") {Axis::Y} else {Axis::X};
        let coord = words.next().unwrap();
        let (_, c) = coord.split_at(coord.find('=').unwrap() + 1);
        words.next();
        let dist = words.next().unwrap();
        return Instruction::Rotate(
            axis,
            u8::from_str(c).unwrap(),
            u8::from_str(dist).unwrap()
        );
    }
}

fn process(inst: &Instruction, lights: &mut HashSet<Coord>) {
    match *inst {
        Instruction::Rectangle(w, h) => {
            for x in 0..w {
                for y in 0..h {
                    lights.insert(Coord(x, y));
                }
            }
        },

        Instruction::Rotate(axis, coord, distance) => {
            let to_be_removed : Vec<Coord> = lights.iter()
                .filter(|c| c.get(axis) == coord)
                .map(|c| c.clone())
                .collect();
            let to_be_added : Vec<Coord> = to_be_removed.iter()
                .map(|c| c.rotate(axis, distance))
                .collect();
            for r in to_be_removed {
                lights.remove(&r);
            }
            for a in to_be_added {
                lights.insert(a);
            }
        },
    }
}

fn main() {
    let f = std::fs::File::open("input").expect("Unable to open input file");
    let mut lights = HashSet::new();
    let mut lines = std::io::BufReader::new(f).lines();
    while let Some(Ok(l)) = lines.next() {
        let inst = parse(&l);
        process(&inst, &mut lights);
    }
    println!("Number of lights on: {:?}", lights.len());

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if lights.contains(&Coord(x, y)) { '*' } else { ' ' } );
        }
        println!("");
    }

}
