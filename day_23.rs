use common::intcode2::{IntcodeComputer, Error, Signal, read_program};
use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let program = read_program("day_23_input.txt")?;

    let mut nics = (0..50).map(|address| {
        let mut comp = IntcodeComputer::new(&program);
        comp.send(address);
        comp
    }).collect::<Vec<_>>();

    // router
    let s = std::time::Instant::now();
    let mut nat_memory = None;
    let mut delivered_ys = HashSet::new();
    'outer: loop {
        let outputs = nics.iter_mut().map(|comp| {
            match comp.run()? {
                Signal::Output(address) => {
                    let x = comp.run()?.output();
                    let y = comp.run()?.output();
                    Ok(Some((address, x, y)))
                },
                Signal::ExpectingInput => {
                    Ok(None)
                },
                x => panic!("unexpected signal: {:?}", x)
            }
        }).collect::<Result<Vec<Option<(isize, isize, isize)>>, Error>>()?
            .into_iter()
            .filter_map(|o| o)
            .collect::<Vec<(isize, isize, isize)>>();

        if outputs.is_empty() {
            // all the nics are waiting for input
            if let Some((x, y)) = &nat_memory {
                if !delivered_ys.insert(*y) {
                    println!("Part2: {} ({:?})", y, s.elapsed());
                    break 'outer;
                }
                nics[0].send(*x);
                nics[0].send(*y);
            } else {
                nics.iter_mut().for_each(|comp| {
                    comp.send(-1);
                });
            }
        } else {
            for (address, x, y) in outputs {
                if address == 255 {
                    if nat_memory.is_none() {
                        println!("Part1: {} ({:?})", y, s.elapsed());
                    }
                    nat_memory = Some((x, y));
                } else {
                    nics[address as usize].send(x);
                    nics[address as usize].send(y);
                }
            }
        }
    }

    Ok(())
}
