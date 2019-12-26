use std::thread;
use std::fmt;
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use common::intcode::{read_program, Signal, DataStream, IntcodeComputer, Error};

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {

    const ALL: [Direction; 4] = [
        Direction::North, Direction::South, Direction::East, Direction::West
    ];

    fn int(&self) -> isize {
        match *self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4
        }
    }

    fn reverse(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }

    fn move_position(&self, position: (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::North => (position.0, position.1 - 1),
            Direction::South => (position.0, position.1 + 1),
            Direction::West => (position.0 - 1, position.1),
            Direction::East => (position.0 + 1, position.1)
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Direction::North => write!(f, "north"),
            Direction::South => write!(f, "south"),
            Direction::West => write!(f, "west"),
            Direction::East => write!(f, "east"),
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

enum State {
    Discovery,
    Backtrack
}

#[allow(unused)]
fn print_map(map: &HashMap<(isize, isize), Tile>, bot_pos: (isize, isize)) {
    let (min_x, min_y, max_x, max_y) = map.keys().fold((0, 0, 0, 0), |(p_min_x, p_min_y, p_max_x, p_max_y), (x, y)| {
        (min(p_min_x, *x), min(p_min_y, *y), max(p_max_x, *x), max(p_max_y, *y))
    });

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x, y) == bot_pos {
                print!("D");
            } else if (x, y) == (0, 0) {
                print!("X");
            } else {
                match map.get(&(x, y)) {
                    Some(tile) => match tile {
                        Tile::Wall => print!("#"),
                        Tile::Empty => print!("."),
                        Tile::Oxygen => print!("O")
                    },
                    None => print!(" ")
                }
            }
        }
        println!("");
    }
}

fn main() -> Result<(), Error> {
    let program = read_program("day_15_input.txt")?;
    let input = DataStream::new();
    let output = DataStream::new();
    let mut comp = IntcodeComputer::new_with_streams(&program, input.clone(), output.clone());
    let thread = thread::spawn(move || comp.run());

    let mut current_direction = Direction::South;
    let mut current_position = (0, 0);
    let mut map = HashMap::new();
    let mut path: Vec<Direction> = Vec::new();
    let mut state = State::Discovery;
    let mut shortest = None;
    //let mut oxygen_pos = None;
    map.insert(current_position, Tile::Empty);
    'outer: loop {
        //println!("CURRENT DIRECTION: {}", current_direction);
        //print_map(&map, current_position);
        input.send(Signal::Value(current_direction.int()))?;
        let v = match output.recv()? {
            Signal::Value(v) => v,
            Signal::Exiting => break 'outer
        };
        if v == 0 {
            let wall_pos = current_direction.move_position(current_position);
            map.insert(wall_pos, Tile::Wall);
            // try find new direction to try
            for check_dir in &[Direction::West, Direction::South, Direction::East, Direction::North] {
                if *check_dir == current_direction {
                    continue;
                }
                let check_pos = check_dir.move_position(current_position);
                match map.get(&check_pos) {
                    Some(Tile::Oxygen) | None => {
                        current_direction = *check_dir;
                        continue 'outer
                    },
                    _ => ()
                }
            }

            // backtrack
            state = State::Backtrack;
            current_direction = path.pop().unwrap().reverse();
        } else if v == 1 {
            current_position = current_direction.move_position(current_position);
            map.insert(current_position, Tile::Empty);
            match state {
                State::Discovery => {
                    path.push(current_direction);
                },
                State::Backtrack => {
                    // check if there is some empty position
                    for check_dir in &[Direction::West, Direction::South, Direction::East, Direction::North] {
                        let check_pos = check_dir.move_position(current_position);
                        if map.get(&check_pos).is_none() {
                            state = State::Discovery;
                            current_direction = *check_dir;
                            continue 'outer;
                        }
                    }

                    // otherwise go back down the path
                    match path.pop() {
                        Some(dir) => {
                            current_direction = dir.reverse();
                        },
                        None => {
                            // no paths left, exit!
                            input.send(Signal::Exiting)?;
                            break 'outer;
                        }
                    }
                }
            }
        } else if v == 2 {
            // found oxygen
            current_position = current_direction.move_position(current_position);
            map.insert(current_position, Tile::Oxygen);
            let dist = path.len() + 1;
            shortest = match shortest {
                Some(prev) => Some(min(prev, dist)),
                None => Some(dist)
            };
            state = State::Backtrack;
            current_direction = current_direction.reverse();
        }
    }

    println!("Part 1: {}", shortest.unwrap());
    //print_map(&map, current_position);
    thread.join().unwrap()?;

    let mut minutes = 0;
    let mut positions = map.iter().filter_map(|(k, v)| {
        if *v == Tile::Oxygen {
            Some(*k)
        } else {
            None
        }
    }).collect::<HashSet<_>>();

    loop {
        positions = positions.iter().flat_map(|pos| {
            Direction::ALL
                .iter()
                .filter_map(|dir| {
                    let npos = dir.move_position(*pos);
                    match map.get(&npos) {
                        Some(Tile::Oxygen) | Some(Tile::Wall) => None,
                        _ => Some(npos)
                    }
                })
                .collect::<Vec<_>>()
        }).collect::<HashSet<_>>();
        if positions.is_empty() {
            break;
        }
        positions.iter().for_each(|pos| { map.insert(*pos, Tile::Oxygen); });
        minutes += 1;
    }
    println!("Part 2: {}", minutes);

    Ok(())
}
