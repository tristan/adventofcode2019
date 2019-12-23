use std::thread;
use itertools::Itertools;
use common::intcode::{self, Signal};


fn main() -> Result<(), intcode::Error> {
    let mut program = intcode::read_program("day_13_input.txt")?;
    let mut comp = intcode::IntcodeComputer::new(&program);

    comp.run()?;

    let num_block_tiles = comp.output_iter().chunks(3).into_iter().filter_map(|mut c| {
        let _x = c.next().unwrap();
        let _y = c.next().unwrap();
        let id = c.next().unwrap();
        if id == 2 {
            Some(id)
        } else {
            None
        }
    }).count();
    println!("Part 1: {}", num_block_tiles);

    program[0] = 2;
    let input = intcode::DataStream::new();
    let output = intcode::DataStream::new();
    let mut comp = intcode::IntcodeComputer::new_with_streams(&program, input.clone(), output.clone());
    let thread: thread::JoinHandle<Result<(), intcode::Error>> = thread::spawn(move || {
        Ok(comp.run()?)
    });

    let mut paddle_location = 0;
    let mut score = 0;
    loop {
        let mut screen = [0isize; 42 * 25];

        let x = match output.recv()? {
            Signal::Value(v) => v,
            Signal::Exiting => break
        };
        let y = match output.recv()? {
            Signal::Value(v) => v,
            Signal::Exiting => break
        };
        let id = match output.recv()? {
            Signal::Value(v) => v,
            Signal::Exiting => break
        };
        if x == -1 && y == 0 {
            score = id;
            continue;
        }

        screen[(y * 42 + x) as usize] = id;
        if id == 3 { // paddle_location
            paddle_location = x;
        } else if id == 4 { // ball location
            // if the ball has moved, the program will be expecting some input
            if paddle_location == x {
                input.send(Signal::Value(0))?;
            } else if paddle_location > x {
                input.send(Signal::Value(-1))?;
            } else {
                input.send(Signal::Value(1))?;
            }
        }
    }

    println!("Part2: {}", score);
    thread.join().unwrap()?;
    Ok(())
}
