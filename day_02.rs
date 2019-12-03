use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    InvalidOpcode(usize),
    IndexOutOfBounds(usize)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}

fn read_input() -> Result<Vec<usize>, Error> {
    // let file = fs::File::open("day_02_input.txt")?;
    // let reader = BufReader::new(file);
    // let result: Result<Vec<u64>, Error> = reader.lines().map(|line| {
    //     let line: String = line?;
    //     Ok(line.parse::<u64>()?)
    // }).collect();
    fs::read_to_string("day_02_input.txt")?.split(",").map(|x| {
        Ok(x.trim().parse::<usize>()?)
    }).collect()
}

fn run_program(memory: &mut Vec<usize>) -> Result<(), Error> {
    let mut pos = 0;
    loop {
        if memory[pos] == 1 {
            let out_pos = memory[pos + 3];
            let in_pos_1 = memory[pos + 1];
            let in_pos_2 = memory[pos + 2];
            if out_pos >= memory.len() {
                return Err(Error::IndexOutOfBounds(out_pos));
            } else if in_pos_1 >= memory.len() {
                return Err(Error::IndexOutOfBounds(in_pos_1));
            } else if in_pos_2 >= memory.len() {
                return Err(Error::IndexOutOfBounds(in_pos_2));
            }
            memory[out_pos] = memory[in_pos_1] + memory[in_pos_2];
        } else if memory[pos] == 2 {
            let out_pos = memory[pos + 3];
            let in_pos_1 = memory[pos + 1];
            let in_pos_2 = memory[pos + 2];
            if out_pos >= memory.len() {
                return Err(Error::IndexOutOfBounds(out_pos));
            } else if in_pos_1 >= memory.len() {
                return Err(Error::IndexOutOfBounds(in_pos_1));
            } else if in_pos_2 >= memory.len() {
                return Err(Error::IndexOutOfBounds(in_pos_2));
            }
            memory[out_pos] = memory[in_pos_1] * memory[in_pos_2];
        } else if memory[pos] == 99 {
            return Ok(())
        } else {
            return Err(Error::InvalidOpcode(memory[pos]));
        }
        pos += 4;
    }
}

fn find_inputs_matching(input: &Vec<usize>, expected_output: usize) -> Option<(usize, usize)> {
    //let input = read_input()?;
    for a in 0..100 {
        for b in 0..100 {
            let mut input = input.clone();
            assert_eq!(input[0], 1);
            input[1] = a;
            input[2] = b;
            let result = run_program(&mut input);
            if result.is_ok() && input[0] == expected_output {
                return Some((a, b));
            } else if result.is_err() {
                println!("{:?}", result.err());
            }
        }
    }
    return None;
}

fn main() -> Result<(), Error> {
    let mut input = read_input()?;
    input[1] = 12;
    input[2] = 2;
    run_program(&mut input).unwrap();
    println!("{}", input[0]);

    let input = read_input()?;
    if let Some((noun, verb)) = find_inputs_matching(&input, 19690720) {
        println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb)
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() {
        let mut t1 = vec![1,0,0,0,99];
        run_program(&mut t1).unwrap();
        assert_eq!(t1, vec![2,0,0,0,99]);

        let mut t2 = vec![2,3,0,3,99];
        run_program(&mut t2).unwrap();
        assert_eq!(t2, vec![2,3,0,6,99]);

        let mut t3 = vec![2,4,4,5,99,0];
        run_program(&mut t3).unwrap();
        assert_eq!(t3, vec![2,4,4,5,99,9801]);

        let mut t4 = vec![1,1,1,4,99,5,6,0,99];
        run_program(&mut t4).unwrap();
        assert_eq!(t4, vec![30,1,1,4,2,5,6,0,99]);
    }
}
