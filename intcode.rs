use std::fs;
use std::io;
use std::num;
use std::convert::TryInto;
use crossbeam::crossbeam_channel::{Receiver, Sender, RecvError, SendError};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    InvalidOpcode(isize),
    RecvError(RecvError),
    SendError(SendError<isize>),
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

pub fn read_program(program: &str) -> Result<Vec<isize>, Error> {
    fs::read_to_string(&program)?.split(",").map(|x| {
        Ok(x.trim().parse::<isize>()?)
    }).collect()
}

pub struct IntcodeComputer {
    pc: usize,
    mem: Vec<isize>,
    pub input: Receiver<isize>,
    pub output: Sender<isize>
}

impl IntcodeComputer {
    pub fn new(mem: &[isize], input: Receiver<isize>, output: Sender<isize>) -> IntcodeComputer {
        IntcodeComputer {
            mem: mem.to_vec(),
            pc: 0,
            input,
            output
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.next() {
                Some(res) => res?,
                None => break
            }
        }
        Ok(())
    }

    pub fn get_param(&self, pos: usize) -> isize {
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
                self.mem[out_pos] = match self.input.recv() {
                    Ok(v) => v,
                    Err(e) => return Some(Err(Error::RecvError(e)))
                };
                self.pc += 2;
            },
            4 => {
                match self.output.send(self.get_param(0)) {
                    Ok(_) => (),
                    Err(e) => return Some(Err(Error::SendError(e)))
                }
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
