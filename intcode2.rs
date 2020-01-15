use std::fs;
use std::io;
use std::num;
use std::convert::TryInto;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    InvalidOpcode(isize),
    BadValueAtPosition(usize),
    InvalidParameterMode(isize)
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

#[derive(Debug)]
pub enum Signal {
    Output(isize),
    ExpectingInput,
    Continue,
    Exiting
}

impl Signal {
    pub fn output(self) -> isize {
        match self {
            Signal::Output(v) => v,
            _ => panic!("expected an output, got: {:?}", self)
        }
    }
}

#[derive(Clone)]
pub struct IntcodeComputer {
    pc: usize,
    mem: Vec<isize>,
    program: Vec<isize>,
    relbase: isize,
    input: Vec<isize>
}

impl IntcodeComputer {
    pub fn new(mem: &[isize]) -> IntcodeComputer {
        IntcodeComputer {
            mem: mem.to_vec(),
            program: mem.to_vec(),
            pc: 0,
            relbase: 0,
            input: vec![]
        }
    }

    pub fn run(&mut self) -> Result<Signal, Error> {
        loop {
            let signal = match self.next() {
                Some(res) => res?,
                None => break
            };
            match signal {
                Signal::Continue => {
                    continue;
                },
                _ => return Ok(signal)
            }
        }
        Ok(Signal::Exiting)
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.relbase = 0;
        self.mem = self.program.clone();
    }

    fn get_pos(&self, pos: usize) -> Result<usize, Error> {
        let code = self.mem[self.pc];
        let mode = (code / 10_isize.pow(pos as u32 + 2)) % 10;
        if mode == 0 {
            Ok(self.mem[self.pc + pos + 1] as usize)
        } else if mode == 1 {
            Err(Error::InvalidParameterMode(mode))
        } else if mode == 2 {
            Ok((self.relbase + self.mem[self.pc + pos + 1]) as usize)
        } else {
            Err(Error::InvalidParameterMode(mode))
        }
    }

    fn get_param(&self, pos: usize) -> Result<isize, Error> {
        let code = self.mem[self.pc];
        let mode = (code / 10_isize.pow(pos as u32 + 2)) % 10;
        let pos = if mode == 0 {
            self.mem[self.pc + pos + 1] as usize
        } else if mode == 1 {
            return Ok(self.mem[self.pc + pos + 1]);
        } else if mode == 2 {
            (self.relbase + self.mem[self.pc + pos + 1]) as usize
        } else {
            return Err(Error::InvalidParameterMode(mode));
        };
        if pos >= self.mem.len() {
            Ok(0)
        } else {
            Ok(self.mem[pos])
        }
    }

    fn set_mem(&mut self, pos: usize, value: isize) {
        if pos >= self.mem.len() {
            self.mem.resize(pos + 1, 0);
        }
        self.mem[pos] = value;
    }

    pub fn send(&mut self, input: isize) {
        self.input.insert(0, input);
    }
}

macro_rules! opt_err {
    ($param:expr) => {
        match $param {
            Ok(v) => v,
            Err(e) => return Some(Err(e))
        }
    }
}

impl Iterator for IntcodeComputer {
    type Item = Result<Signal, Error>;

    fn next(&mut self) -> Option<Self::Item> {

        // parse opcode
        let code = self.mem[self.pc];
        let opcode = ((code / 10_isize.pow(0)) % 10) + 10 * ((code / 10_isize.pow(1)) % 10);

        match opcode {
            1 => {
                let out_pos = opt_err!(self.get_pos(2)) as usize;
                self.set_mem(out_pos, opt_err!(self.get_param(0)) + opt_err!(self.get_param(1)));
                self.pc += 4;
            },
            2 => {
                let out_pos = opt_err!(self.get_pos(2)) as usize;
                self.set_mem(out_pos, opt_err!(self.get_param(0)) * opt_err!(self.get_param(1)));
                self.pc += 4;
            },
            3 => {
                let out_pos = opt_err!(self.get_pos(0)) as usize;
                if let Some(value) = self.input.pop() {
                    self.set_mem(out_pos, value);
                    self.pc += 2;
                } else {
                    return Some(Ok(Signal::ExpectingInput));
                }
            },
            4 => {
                let output = opt_err!(self.get_param(0));
                self.pc += 2;
                return Some(Ok(Signal::Output(output)));
            },
            5 => {
                if opt_err!(self.get_param(0)) != 0 {
                    self.pc = match opt_err!(self.get_param(1)).try_into() {
                        Ok(v) => v,
                        Err(_) => return Some(Err(Error::BadValueAtPosition(self.pc)))
                    };
                } else {
                    self.pc += 3;
                }
            },
            6 => {
                if opt_err!(self.get_param(0)) == 0 {
                    self.pc = match opt_err!(self.get_param(1)).try_into() {
                        Ok(v) => v,
                        Err(_) => return Some(Err(Error::BadValueAtPosition(self.pc)))
                    };
                } else {
                    self.pc += 3;
                }
            },
            7 => {
                let out_pos = opt_err!(self.get_pos(2)) as usize;
                self.set_mem(out_pos, if opt_err!(self.get_param(0)) < opt_err!(self.get_param(1)) {
                    1
                }  else {
                    0
                });
                self.pc += 4;
            },
            8 => {
                let out_pos = opt_err!(self.get_pos(2)) as usize;
                self.set_mem(out_pos, if opt_err!(self.get_param(0)) == opt_err!(self.get_param(1)) {
                    1
                }  else {
                    0
                });
                self.pc += 4;
            },
            9 => {
                self.relbase += opt_err!(self.get_param(0));
                self.pc += 2;
            },
            99 => {
                return None;
            },
            _ => return Some(Err(Error::InvalidOpcode(opcode)))
        }

        Some(Ok(Signal::Continue))
    }
}
