// use std::thread;
// use std::io::Write;
// use std::io::stdout;
use common::intcode::{IntcodeComputer, Signal, DataStream, Error, read_program};

fn find_square_of_size(
    mut comp: IntcodeComputer,
    input: DataStream,
    output: DataStream,
    size: isize
) -> (isize, isize) {
    let mut y = size;
    let mut x = size;
    'outer: loop {
        y += 1;
        x = (x..).filter(|&dx| {
            input.send(Signal::Value(dx)).unwrap();
            input.send(Signal::Value(y)).unwrap();
            comp.run().unwrap();
            let v = output.recv().unwrap().value();
            comp.reset();
            v == 1
        }).next().unwrap();
        input.send(Signal::Value(x + size - 1)).unwrap();
        input.send(Signal::Value(y)).unwrap();
        comp.run().unwrap();
        let v = output.recv().unwrap().value();
        comp.reset();
        if v == 0 {
            continue;
        }
        input.send(Signal::Value(x)).unwrap();
        input.send(Signal::Value(y - (size - 1))).unwrap();
        comp.run().unwrap();
        let v = output.recv().unwrap().value();
        comp.reset();
        if v == 0 {
            continue 'outer;
        }
        input.send(Signal::Value(x + size - 1)).unwrap();
        input.send(Signal::Value(y - (size - 1))).unwrap();
        comp.run().unwrap();
        let v = output.recv().unwrap().value();
        comp.reset();
        if v == 0 {
            continue 'outer;
        }

        return (x, y - (size - 1));
    }
}

fn main() -> Result<(), Error> {
    let program = read_program("day_19_input.txt")?;
    let input = DataStream::new();
    let output = DataStream::new();
    let mut comp = IntcodeComputer::new_with_streams(&program, input.clone(), output.clone());

    let mut sum = 0;
    for y in 0..150 {
        //print!("{:4}: ", y);
        for x in 0..150 {
            input.send(Signal::Value(x))?;
            input.send(Signal::Value(y))?;
            comp.run()?;

            let v = output.recv()?.value();
            if v == 0 {
                //print!(".");
            } else {
                sum += 1;
                //print!("#");
            }
            comp.reset();
        }
        //println!("");
    }

    println!("part1: {}", sum);

    let (x, y) = find_square_of_size(comp.clone(), input.clone(), output.clone(), 100);
    //dbg!(x, y);

    // for sy in (y-50)..(y+100) {
    //     print!("{:4}: ", sy);
    //     for sx in (x-50)..(x+100) {
    //         input.send(Signal::Value(sx))?;
    //         input.send(Signal::Value(sy))?;
    //         comp.run()?;

    //         let v = output.recv()?.value();
    //         if v == 0 {
    //             print!(".");
    //         } else {
    //             if sx >= x && sx < x+100 && sy >= y && sy < y+100 {
    //                 print!("O");
    //             } else {
    //                 print!("#");
    //             }
    //         }
    //         comp.reset();
    //     }
    //     println!("");
    // }

    // assert_eq!(x, 129);
    // assert_eq!(y, 72);


    println!("part2: {}", x * 10000 + y);

    Ok(())
}
