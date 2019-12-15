use itertools::Itertools;
use crossbeam::crossbeam_channel::{RecvError, SendError, unbounded as channel};
use std::thread;

use intcode;

#[derive(Debug)]
enum Error {
    IntCodeError(intcode::Error),
    RecvError(RecvError),
    SendError(SendError<isize>),
    NoResults
}

impl From<intcode::Error> for Error {
    fn from(err: intcode::Error) -> Error {
        Error::IntCodeError(err)
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Error {
        Error::RecvError(err)
    }
}

impl From<SendError<isize>> for Error {
    fn from(err: SendError<isize>) -> Error {
        Error::SendError(err)
    }
}

fn run_amplifiers(program: &[isize], phases: &[isize]) -> Result<isize, Error> {
    let (sender_a, receiver_a) = channel();
    let (sender_b, receiver_b) = channel();
    let (sender_c, receiver_c) = channel();
    let (sender_d, receiver_d) = channel();
    let (sender_e, receiver_e) = channel();
    let (sender_f, receiver_f) = channel();

    sender_a.send(phases[0])?;
    sender_a.send(0)?;
    sender_b.send(phases[1])?;
    sender_c.send(phases[2])?;
    sender_d.send(phases[3])?;
    sender_e.send(phases[4])?;

    let mut amp_a = intcode::IntcodeComputer::new(&program, receiver_a.clone(), sender_b);
    let mut amp_b = intcode::IntcodeComputer::new(&program, receiver_b, sender_c);
    let mut amp_c = intcode::IntcodeComputer::new(&program, receiver_c, sender_d);
    let mut amp_d = intcode::IntcodeComputer::new(&program, receiver_d, sender_e);
    let mut amp_e = intcode::IntcodeComputer::new(&program, receiver_e, sender_f.clone());

    let thread_a: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || Ok(amp_a.run()?));
    let thread_b: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || Ok(amp_b.run()?));
    let thread_c: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || Ok(amp_c.run()?));
    let thread_d: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || Ok(amp_d.run()?));
    let thread_e: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || Ok(amp_e.run()?));
    let _feedback: thread::JoinHandle<Result<(), Error>>  = thread::spawn(move || {
        // @TODO: shutdown this thread ?....
        loop {
            let result = receiver_f.recv()?;
            sender_a.send(result)?;
        }
    });

    thread_a.join().unwrap()?;
    thread_b.join().unwrap()?;
    thread_c.join().unwrap()?;
    thread_d.join().unwrap()?;
    thread_e.join().unwrap()?;

    // feedback thread sent this to a, but a should be halted
    // so we should be able to recv it
    let result = receiver_a.recv()?;
    Ok(result)
}

fn find_max_thruster_signal(program: &[isize]) -> Result<isize, Error> {
    (0..5)
        .permutations(5)
        .map(|phases| run_amplifiers(&program, &phases))
        .collect::<Result<Vec<isize>, Error>>()?
        .into_iter()
        .max()
        .ok_or(Error::NoResults)
}

fn find_max_thruster_signal_with_feedback(program: &[isize]) -> Result<isize, Error> {
    (5..10)
        .permutations(5)
        .map(|phases| run_amplifiers(&program, &phases))
        .collect::<Result<Vec<isize>, Error>>()?
        .into_iter()
        .max()
        .ok_or(Error::NoResults)
}

fn main() -> Result<(), Error> {
    let program = intcode::read_program("day_07_input.txt")?;
    println!("Part1: {}", find_max_thruster_signal(&program)?);
    println!("Part2: {}", find_max_thruster_signal_with_feedback(&program)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() -> Result<(), Error> {
        let program = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(run_amplifiers(&program, &[4,3,2,1,0])?, 43210);
        assert_eq!(find_max_thruster_signal(&program)?, 43210);

        let program = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(run_amplifiers(&program, &[0,1,2,3,4])?, 54321);
        assert_eq!(find_max_thruster_signal(&program)?, 54321);

        let program = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(run_amplifiers(&program, &[1,0,4,3,2])?, 65210);
        assert_eq!(find_max_thruster_signal(&program)?, 65210);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), Error> {
        let program = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(find_max_thruster_signal_with_feedback(&program)?, 139629729);

        let program = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        assert_eq!(find_max_thruster_signal_with_feedback(&program)?, 18216);

        Ok(())
    }
}
