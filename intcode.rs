use std::fs;
use std::io;
use std::num;
use std::convert::TryInto;
use crossbeam::crossbeam_channel::{Receiver, Sender, RecvError, SendError, TryIter, unbounded as channel};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    InvalidOpcode(isize),
    RecvError(RecvError),
    SendError(SendError<Signal>),
    BadValueAtPosition(usize),
    InvalidParameterMode(isize),
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

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Error {
        Error::RecvError(err)
    }
}

impl From<SendError<Signal>> for Error {
    fn from(err: SendError<Signal>) -> Error {
        Error::SendError(err)
    }
}

pub fn read_program(program: &str) -> Result<Vec<isize>, Error> {
    fs::read_to_string(&program)?.split(",").map(|x| {
        Ok(x.trim().parse::<isize>()?)
    }).collect()
}

#[derive(Clone)]
pub struct DataStream(Sender<Signal>, Receiver<Signal>);

pub enum Signal {
    Value(isize),
    Exiting
}

impl Signal {
    pub fn value(self) -> isize {
        match self {
            Signal::Value(v) => v,
            _ => panic!("not a value")
        }
    }
}


impl DataStream {
    pub fn new() -> DataStream {
        let (sender, receiver) = channel();
        DataStream(sender, receiver)
    }

    pub fn send(&self, input: Signal) -> Result<(), Error> {
        Ok(self.0.send(input)?)
    }

    pub fn recv(&self) -> Result<Signal, Error> {
        Ok(self.1.recv()?)
    }

    pub fn try_iter(&self) -> TryIter<Signal> {
        self.1.try_iter()
    }
}

#[derive(Clone)]
pub struct IntcodeComputer {
    pc: usize,
    mem: Vec<isize>,
    program: Vec<isize>,
    relbase: isize,
    input: DataStream,
    output: DataStream
}

impl IntcodeComputer {
    pub fn new_with_streams(mem: &[isize], input: DataStream, output: DataStream) -> IntcodeComputer {
        IntcodeComputer {
            mem: mem.to_vec(),
            program: mem.to_vec(),
            pc: 0,
            relbase: 0,
            input,
            output
        }
    }

    pub fn new(mem: &[isize]) -> IntcodeComputer {
        let input = DataStream::new();
        let output = DataStream::new();
        IntcodeComputer::new_with_streams(mem, input, output)
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

    pub fn reset(&mut self) {
        let mut i = self.input.1.try_iter();
        while let Some(_) = i.next() {}
        let mut i = self.output.1.try_iter();
        while let Some(_) = i.next() {}
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

    pub fn send(&self, input: Signal) -> Result<(), Error> {
        self.input.send(input)
    }

    pub fn recv(&self) -> Result<Signal, Error> {
        self.output.recv()
    }

    pub fn output_iter<'a>(&'a self) -> impl Iterator<Item = isize> + 'a {
        self.output.try_iter().filter_map(|x| match x {
            Signal::Value(v) => Some(v),
            _ => None
        })
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
    type Item = Result<(), Error>;

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
                self.set_mem(out_pos, match self.input.recv() {
                    Ok(v) => match v {
                        Signal::Value(v) => v,
                        Signal::Exiting => return None
                    },
                    Err(e) => return Some(Err(e))
                });
                self.pc += 2;
            },
            4 => {
                let output = opt_err!(self.get_param(0));
                opt_err!(self.output.send(Signal::Value(output)));
                self.pc += 2;
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
                opt_err!(self.output.send(Signal::Exiting));
                return None;
            },
            _ => return Some(Err(Error::InvalidOpcode(opcode)))
        }

        Some(Ok(()))
    }
}
