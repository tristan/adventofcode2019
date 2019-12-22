use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use intcode::{self, Signal};
use std::cmp::{min, max};

#[derive(Debug)]
enum Error {
    IntcodeError(intcode::Error)
}

impl From<intcode::Error> for Error {
    fn from(err: intcode::Error) -> Error {
        Error::IntcodeError(err)
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn turn(&self, int: isize) -> Direction {
        match int {
            1 => {
                match self {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left
                }
            },
            0 => {
                match self {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right
                }
            },
            _ => panic!("invalid direction")
        }
    }
}

struct Brain {
    input: intcode::DataStream,
    output: intcode::DataStream,
    thread: thread::JoinHandle<Result<(), intcode::Error>>
}

impl Brain {
    fn new(program: &[isize]) -> Brain {
        let input = intcode::DataStream::new();
        let output = intcode::DataStream::new();
        let mut computer = intcode::IntcodeComputer::new_with_streams(&program, input.clone(), output.clone());
        let thread = thread::spawn(move || Ok(computer.run()?));
        Brain {
            input,
            output,
            thread
        }
    }
}

struct EmergencyHullPaintingRobot {
    brain: Brain
}

impl EmergencyHullPaintingRobot {
    fn new(program: &[isize]) -> EmergencyHullPaintingRobot {
        let brain = Brain::new(program);
        EmergencyHullPaintingRobot {
            brain
        }
    }

    fn run(self, starting_colour: isize) -> Result<HashMap<(isize, isize), isize>, Error> {
        let hull: Arc<Mutex<HashMap<(isize, isize), isize>>> = Arc::new(Mutex::new(HashMap::new()));
        hull.lock().unwrap().insert((0, 0), starting_colour);
        let input = self.brain.input.clone();
        let output = self.brain.output.clone();
        let robot: thread::JoinHandle<Result<(), Error>> = {
            let hull = hull.clone();
            thread::spawn(move || {
                // @TODO: shutdown this thread ?....
                let mut direction = Direction::Up;
                let mut pos = (0, 0);
                loop {
                    let current_colour = *hull.lock().unwrap().get(&pos)
                        .unwrap_or(&0);
                    input.send(Signal::Value(current_colour))?;
                    let new_colour = match output.recv()? {
                        Signal::Value(v) => v,
                        Signal::Exiting => return Ok(())
                    };
                    hull.lock().unwrap().insert(pos, new_colour);
                    let turn = match output.recv()? {
                        Signal::Value(v) => v,
                        Signal::Exiting => return Ok(())
                    };
                    direction = direction.turn(turn);
                    pos = match direction {
                        Direction::Up => (pos.0 - 1, pos.1),
                        Direction::Down => (pos.0 + 1, pos.1),
                        Direction::Left => (pos.0, pos.1 - 1),
                        Direction::Right => (pos.0, pos.1 + 1),
                    }
                }
            })
        };

        self.brain.thread.join().unwrap()?;
        robot.join().unwrap()?;

        Ok(Arc::try_unwrap(hull).unwrap().into_inner().unwrap())
    }
}

fn main() -> Result<(), Error> {

    let program = intcode::read_program("day_11_input.txt")?;
    let hull = EmergencyHullPaintingRobot::new(&program.clone())
        .run(0).unwrap();
    println!("Part1: {}", hull.len());
    let hull = EmergencyHullPaintingRobot::new(&program.clone())
        .run(1).unwrap();

    let (min_x, min_y, max_x, max_y) = hull.keys().fold(
        (std::isize::MAX, std::isize::MAX, std::isize::MIN, std::isize::MIN),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            (min(min_x, *x), min(min_y, *y), max(max_x, *x), max(max_y, *y))
        });

    (min_x..=max_x).for_each(|x| {
        (min_y..=max_y).for_each(|y| {
            let colour = *hull.get(&(x, y)).unwrap_or(&0);
            if colour == 0 { print!(" ") } else { print!("█") }
        });
        println!("");
    });

    Ok(())
}
