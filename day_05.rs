use std::fs;
use std::io;
use std::num;
use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    InvalidOpcode(isize),
    ExpectedInput,
    BadValueAtPosition(usize),
    //IndexOutOfBounds(isize)
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

fn read_program(program: &str) -> Result<Vec<isize>, Error> {
    fs::read_to_string(&program)?.split(",").map(|x| {
        Ok(x.trim().parse::<isize>()?)
    }).collect()
}

struct IntcodeComputer {
    pc: usize,
    mem: Vec<isize>,
    pub input: VecDeque<isize>,
    pub output: Vec<isize>
}

impl IntcodeComputer {
    fn new_from_source(program: &str) -> Result<IntcodeComputer, Error> {
        Ok(IntcodeComputer {
            mem: read_program(program)?,
            pc: 0,
            input: VecDeque::new(),
            output: vec![]
        })
    }

    #[allow(dead_code)]
    fn new_from_buffer(mem: &[isize]) -> IntcodeComputer {
        IntcodeComputer {
            mem: mem.to_vec(),
            pc: 0,
            input: VecDeque::new(),
            output: vec![]
        }
    }

    fn run_with_input(&mut self, input: &[isize]) -> Result<(), Error> {
        self.input = input.iter().map(|x| *x).collect::<VecDeque<isize>>();
        self.run()
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.next() {
                Some(res) => res?,
                None => break
            }
        }
        Ok(())
    }

    fn get_param(&self, pos: usize) -> isize {
        let code = self.mem[self.pc];
        let mode = (code / 10_isize.pow(pos as u32 + 2)) % 10;
        if mode == 0 {
            let in_pos = self.mem[self.pc + pos + 1] as usize;
            self.mem[in_pos]
        } else {
            self.mem[self.pc + pos + 1]
        }
    }
}

impl Iterator for IntcodeComputer {
    type Item = Result<(), Error>;

    fn next(&mut self) -> Option<Self::Item> {

        // parse opcode
        let code = self.mem[self.pc];
        let opcode = ((code / 10_isize.pow(0)) % 10) + 10 * ((code / 10_isize.pow(1)) % 10);

        match opcode {
            1 => {
                let out_pos = self.mem[self.pc + 3] as usize;
                self.mem[out_pos] = self.get_param(0) + self.get_param(1);
                self.pc += 4;
            },
            2 => {
                let out_pos = self.mem[self.pc + 3] as usize;
                self.mem[out_pos] = self.get_param(0) * self.get_param(1);
                self.pc += 4;
            },
            3 => {
                let out_pos = self.mem[self.pc + 1] as usize;
                self.mem[out_pos] = match self.input.pop_front() {
                    Some(v) => v,
                    None => return Some(Err(Error::ExpectedInput))
                };
                self.pc += 2;
            },
            4 => {
                self.output.push(self.get_param(0));
                self.pc += 2;
            },
            5 => {
                if self.get_param(0) != 0 {
                    self.pc = match self.get_param(1).try_into() {
                        Ok(v) => v,
                        Err(_) => return Some(Err(Error::BadValueAtPosition(self.pc)))
                    };
                } else {
                    self.pc += 3;
                }
            },
            6 => {
                if self.get_param(0) == 0 {
                    self.pc = match self.get_param(1).try_into() {
                        Ok(v) => v,
                        Err(_) => return Some(Err(Error::BadValueAtPosition(self.pc)))
                    };
                } else {
                    self.pc += 3;
                }
            },
            7 => {
                let out_pos = self.mem[self.pc + 3] as usize;
                self.mem[out_pos] = if self.get_param(0) < self.get_param(1) {
                    1
                }  else {
                    0
                };
                self.pc += 4;
            },
            8 => {
                let out_pos = self.mem[self.pc + 3] as usize;
                self.mem[out_pos] = if self.get_param(0) == self.get_param(1) {
                    1
                }  else {
                    0
                };
                self.pc += 4;
            },
            99 => return None,
            _ => return Some(Err(Error::InvalidOpcode(opcode)))
        }

        Some(Ok(()))
    }
}

fn main() -> Result<(), Error>  {
    let mut comp = IntcodeComputer::new_from_source("day_05_input.txt")?;
    comp.run_with_input(&[1])?;
    println!("{:?}", comp.output);

    let mut comp = IntcodeComputer::new_from_source("day_05_input.txt")?;
    comp.run_with_input(&[5])?;
    println!("{:?}", comp.output);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() -> Result<(), Error> {
        let mut comp = IntcodeComputer::new_from_buffer(&[1002,4,3,4,33]);
        comp.run()?;
        assert_eq!(comp.mem, [1002, 4, 3, 4, 99]);
        Ok(())
    }

    #[test]
    fn test_run_program_2() -> Result<(), Error> {
        let mut comp = IntcodeComputer::new_from_buffer(&[3,9,8,9,10,9,4,9,99,-1,8]);
        comp.run_with_input(&[1])?;
        assert_eq!(comp.output, [0]);

        let mut comp = IntcodeComputer::new_from_buffer(&[3,9,8,9,10,9,4,9,99,-1,8]);
        comp.run_with_input(&[8])?;
        assert_eq!(comp.output, [1]);

        Ok(())
    }

    #[test]
    fn test_run_program_3() -> Result<(), Error> {
        let mut comp = IntcodeComputer::new_from_buffer(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                                                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                                                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        comp.run_with_input(&[7])?;
        assert_eq!(comp.output, [999]);

        let mut comp = IntcodeComputer::new_from_buffer(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                                                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                                                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        comp.run_with_input(&[8])?;
        assert_eq!(comp.output, [1000]);

        let mut comp = IntcodeComputer::new_from_buffer(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                                                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                                                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        comp.run_with_input(&[9])?;
        assert_eq!(comp.output, [1001]);
        Ok(())
    }


}
