use intcode;

#[derive(Debug)]
enum Error {
    IntCodeError(intcode::Error),
}

impl From<intcode::Error> for Error {
    fn from(err: intcode::Error) -> Error {
        Error::IntCodeError(err)
    }
}




fn main() -> Result<(), Error> {
    let program = intcode::read_program("day_09_input.txt")?;
    println!("Part 1:");
    let mut comp = intcode::IntcodeComputer::new(&program);
    comp.send(1)?;
    comp.run()?;
    comp.output_iter().for_each(|output| println!("{}", output));

    println!("Part 2:");
    let mut comp = intcode::IntcodeComputer::new(&program);
    comp.send(2)?;
    comp.run()?;
    comp.output_iter().for_each(|output| println!("{}", output));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() -> Result<(), Error> {
        let input = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut comp = intcode::IntcodeComputer::new(&input);
        comp.run()?;
        assert_eq!(comp.output_iter().collect::<Vec<_>>(), input);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), Error> {
        let input = [1102,34915192,34915192,7,4,7,99,0];
        let mut comp = intcode::IntcodeComputer::new(&input);
        comp.run()?;
        assert_eq!(format!("{}", comp.recv()?).len(), 16);

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), Error> {
        let input = [104,1125899906842624,99];
        let mut comp = intcode::IntcodeComputer::new(&input);
        comp.run()?;
        assert_eq!(comp.recv()?, 1125899906842624);

        Ok(())
    }
}
