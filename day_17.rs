use common::intcode::{IntcodeComputer, Signal, DataStream, Error, read_program};

fn main() -> Result<(), Error> {

    let program = read_program("day_17_input.txt")?;
    let mut comp = IntcodeComputer::new(&program);

    comp.run()?;

    let map = comp.output_iter()
        .map(|i| std::char::from_u32(i as _).unwrap())
        .collect::<String>()
        .split("\n")
        .map(|v| v.chars().collect::<Vec<_>>())
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();
    let x_len = map[0].len();
    let y_len = map.len();

    map.iter().for_each(|l| println!("{}", l.iter().collect::<String>()));

    let mut sum = 0;
    let mut pos = (0, 0);
    // start at 1, and end at -1 because we can't have an intersections at the boundaries
    for y in 1..(y_len - 1) {
        for x in 1..(x_len - 1) {
            if map[y][x] == '#' &&
                map[y - 1][x] == '#' &&
                map[y + 1][x] == '#' &&
                map[y][x - 1] == '#' &&
                map[y][x + 1] == '#' {
                    sum += x * y;
                }
            else if map[y][x] == '^' {
                pos = (x, y);
            }
        }
    }
    println!("Part 1: {}", sum);

    let mut dir = '^';
    let mut inputs = vec![];
    loop {
        // find direction to move
        if dir == '^' || dir == 'v' {
            if pos.0 + 1 < x_len && map[pos.1][pos.0 + 1] == '#' {
                if dir == '^' {
                    //print!("R,");
                    dir = '>';
                    inputs.push("R".to_string());
                } else {
                    //print!("L,");
                    dir = '>';
                    inputs.push("L".to_string());
                }
                let dist = map[pos.1].iter()
                    .skip(pos.0 + 1)
                    .take_while(|&&c| c == '#')
                    .count();
                //println!("{},", dist);
                inputs.push(format!("{}", dist));
                pos = (pos.0 + dist, pos.1);
            } else if pos.0 > 0 && map[pos.1][pos.0 - 1] == '#' {
                if dir == '^' {
                    //print!("L,");
                    dir = '<';
                    inputs.push("L".to_string());
                } else {
                    //print!("R,");
                    dir = '<';
                    inputs.push("R".to_string());
                }
                let dist = map[pos.1].iter()
                    .rev()
                    .skip(x_len - pos.0)
                    .take_while(|&&c| c == '#')
                    .count();
                //print!("{},", dist);
                inputs.push(format!("{}", dist));
                pos = (pos.0 - dist, pos.1);
            } else {
                break;
            }
        } else if dir == '>' || dir == '<' {
            if pos.1 + 1 < y_len && map[pos.1 + 1][pos.0] == '#' {
                if dir == '>' {
                    //print!("R,");
                    dir = 'v';
                    inputs.push("R".to_string());
                } else {
                    //print!("L,");
                    dir = 'v';
                    inputs.push("L".to_string());
                }
                let dist = map.iter()
                    .skip(pos.1 + 1)
                    .take_while(|v| v[pos.0] == '#')
                    .count();
                //print!("{},", dist);
                inputs.push(format!("{}", dist));
                pos = (pos.0, pos.1 + dist);
            } else if pos.1 > 0 && map[pos.1 - 1][pos.0] == '#' {
                if dir == '>' {
                    //print!("L,");
                    dir = '^';
                    inputs.push("L".to_string());
                } else {
                    //print!("R,");
                    dir = '^';
                    inputs.push("R".to_string());
                }
                let dist = map.iter()
                    .rev()
                    .skip(y_len - pos.1)
                    .take_while(|v| v[pos.0] == '#')
                    .count();
                //print!("{},", dist);
                inputs.push(format!("{}", dist));
                pos = (pos.0, pos.1 - dist);
            } else {
                break;
            }
        } else {
            panic!("dir: {}", dir);
        }
    }
    //println!("");
    let input_str = inputs
        .iter()
        .zip(std::iter::repeat(",".to_string()))
        .map(|(a, b)| vec![a.clone(), b.clone()])
        .flatten()
        .collect::<String>();
    println!("{}", input_str);

    // R,12,L,10,R,12,L,8,R,10,R,6,R,12,L,10,R,12,R,12,L,10,R,10,L,8,
    // L,8,R,10,R,6,R,12,L,10,R,10,L,8,L,8,R,10,R,6,R,12,L,10,R,10,
    // L,8,R,12,L,10,R,12,R,12,L,10,R,10,L,8,

    // DONE MANUALLY!

    // A = R,12,L,10,R,10,L,8
    let program_a = "R,12,L,10,R,10,L,8\n".chars().map(|c| c as isize).collect::<Vec<_>>();
    // B = L,8,R,10,R,6
    let program_b = "L,8,R,10,R,6\n".chars().map(|c| c as isize).collect::<Vec<_>>();
    // C = R,12,L,10,R,12
    let program_c = "R,12,L,10,R,12\n".chars().map(|c| c as isize).collect::<Vec<_>>();

    // C,B,C,A,B,A,B,A,C,A
    let program_main = "C,B,C,A,B,A,B,A,C,A\n".chars().map(|c| c as isize).collect::<Vec<_>>();


    let mut program = read_program("day_17_input.txt")?;
    program[0] = 2;
    let mut comp = IntcodeComputer::new(&program);
    [program_main, program_a, program_b, program_c].iter().for_each(|program| {
        program.iter().for_each(|&i| {
            comp.send(Signal::Value(i)).unwrap();
        });
    });

    comp.send(Signal::Value('n' as isize))?;
    comp.send(Signal::Value('\n' as isize))?;
    comp.run()?;

    let result = comp.output_iter().last().unwrap();
    println!("Part 2: {}", result);

    Ok(())
}
