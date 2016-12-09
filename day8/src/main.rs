use std::ops::IndexMut;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y
}

#[derive(Debug)]
enum Instruction {
    Rectangle(i8, i8),
    Rotate(Axis, i8, i8)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord(i8, i8);

impl Coord {
    fn get(& self, idx: Axis) -> i8 {
        match idx {
            Axis::X => self.0,
            Axis::Y => self.1
        }
    }

    fn rotate(&self, idx: Axis, dist: i8) -> Coord {
        let mut x = self.0;
        let mut y = self.1;
        match idx {
            Axis::X => y = (y + dist + 6) % 6,
            Axis::Y => x = (x + dist + 50) % 50,
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
            i8::from_str(splitted.next().unwrap()).unwrap(),
            i8::from_str(splitted.next().unwrap()).unwrap());
    }
    else {
        let axis = if words.next() == Some("row") {Axis::Y} else {Axis::X};
        let coord = words.next().unwrap();
        let (_, c) = coord.split_at(coord.find('=').unwrap() + 1);
        words.next();
        let dist = words.next().unwrap();
        return Instruction::Rotate(
            axis,
            i8::from_str(c).unwrap(),
            i8::from_str(dist).unwrap()
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
            println!("To be removed {:?}", to_be_removed);
            let to_be_added : Vec<Coord> = to_be_removed.iter()
                .map(|c| c.rotate(axis, distance))
                .collect();
            println!("To be added   {:?}", to_be_added);
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
    let rect = parse("rect 3x2");
    println!("{:?}", rect);
    let mut lights = HashSet::new();
    process(&rect,  &mut lights);
    println!("{:?}", lights);
    let rot =  parse("rotate column x=1 by 1");
    println!("{:?}", rot);
    process(&rot,  &mut lights);
    println!("{:?}", lights);
    let rot =  parse("rotate row y=0 by 4");
    println!("{:?}", rot);
    process(&rot,  &mut lights);
    println!("{:?}", lights);
}
