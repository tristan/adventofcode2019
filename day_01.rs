use std::fs;
use std::io::{self, BufReader, BufRead};
use std::num;

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError)
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

fn read_input() -> Result<Vec<u64>, Error> {
    let file = fs::File::open("day_01_input.txt")?;
    let reader = BufReader::new(file);
    let result: Result<Vec<u64>, Error> = reader.lines().map(|line| {
        let line: String = line?;
        Ok(line.parse::<u64>()?)
    }).collect();
    result
}

fn calculate_fuel_needed(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn calculate_fuel_needed_recursive(mut mass: u64) -> u64 {
    let mut result = 0;
    loop {
        let fuel = calculate_fuel_needed(mass);
        if fuel > 0 {
            result += fuel;
            mass = fuel;
        } else {
            break;
        }
    }

    result
}

fn main() -> Result<(), Error> {
    let result: u64 = read_input()?
        .into_iter()
        .map(calculate_fuel_needed)
        .sum();
    println!("Part1: {}", result);

    let result: u64 = read_input()?
        .into_iter()
        .map(calculate_fuel_needed_recursive)
        .sum();
    println!("Part2: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_tests() {
        assert_eq!(calculate_fuel_needed(12), 2);
        assert_eq!(calculate_fuel_needed(14), 2);
        assert_eq!(calculate_fuel_needed(1969), 654);
        assert_eq!(calculate_fuel_needed(100756), 33583);
    }

    #[test]
    fn run_tests_2() {
        assert_eq!(calculate_fuel_needed(2), 0);
        assert_eq!(calculate_fuel_needed_recursive(1969), 966);
        assert_eq!(calculate_fuel_needed_recursive(100756), 50346);
    }
}
