use std::fs;
use std::io::{self, BufReader};
use std::num;
use std::cmp::{max, min};

#[derive(Debug)]
enum Error {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    InvalidInput
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

#[derive(Debug, PartialEq)]
enum Point {
    R(usize),
    L(usize),
    U(usize),
    D(usize)
}

fn read_input<R: io::Read>(mut reader: R) -> Result<(Vec<Point>, Vec<Point>), Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;
    let result: Result<Vec<Vec<Point>>, Error> = lines.trim().split("\n").map(|line| {
        let line: Result<Vec<Point>, Error> = line.trim().split(",")
            .map(|point| point.trim())
            .filter(|point| !point.is_empty())
            .map(|point| {
                match point.chars().nth(0) {
                    Some('R') => Ok(Point::R(point[1..].parse::<usize>()?)),
                    Some('L') => Ok(Point::L(point[1..].parse::<usize>()?)),
                    Some('U') => Ok(Point::U(point[1..].parse::<usize>()?)),
                    Some('D') => Ok(Point::D(point[1..].parse::<usize>()?)),
                    _ => { dbg!(point); Err(Error::InvalidInput) }
                }
            }).collect();
        line
    }).collect();
    let mut result = result?;
    let wire2 = result.pop().ok_or(Error::InvalidInput)?;
    let wire1 = result.pop().ok_or(Error::InvalidInput)?;
    Ok((wire1, wire2))
}

fn find_closest_intersection(
    wire1: &[Point],
    wire2: &[Point],
    board_size: usize,
    origin: (usize, usize)
) -> Option<(usize, usize)> {
    let mut board = vec![vec![0usize; board_size]; board_size];

    let mut x = origin.0;
    let mut y = origin.1;
    let mut step: usize = 1;

    for point in wire1 {
        let a = match point {
            Point::R(dist) => {
                let start = x + 1;
                let end = start + dist;
                for i in start..end {
                    board[y][i] = step;
                    step += 1;
                }
                (end - 1, y)
            },

            Point::L(dist) => {
                let start = x - dist;
                let mut steps = step + dist - 1;
                for i in start..x {
                    board[y][i] = steps;
                    steps -= 1;
                }
                step += dist;
                (start, y)
            },
            Point::U(dist) => {
                let start = y + 1;
                let end = start + dist;
                for i in start..end {
                    board[i][x] = step;
                    step += 1;
                }
                (x, end - 1)
            },

            Point::D(dist) => {
                let start = y - dist;
                let mut steps = step + dist - 1;
                for i in start..y {
                    board[i][x] = steps;
                    steps -= 1;
                }
                step += dist;
                (x, start)
            }
        };
        x = a.0;
        y = a.1;
    }

    step = 1;
    x = origin.0;
    y = origin.1;

    let mut intersections = vec![];
    for point in wire2 {
        let a = match point {
            Point::R(dist) => {
                let start = x + 1;
                let end = start + dist;
                for i in start..end {
                    if board[y][i] > 0 {
                        intersections.push((y, i, board[y][i], step));
                    }
                    step += 1;
                }
                (end - 1, y)
            },

            Point::L(dist) => {
                let start = x - dist;
                let mut steps = step + dist - 1;
                for i in start..x {
                    if board[y][i] > 0 {
                        intersections.push((y, i, board[y][i], steps));
                    }
                    steps -= 1;
                }
                step += dist;
                (start, y)
            },
            Point::U(dist) => {
                let start = y + 1;
                let end = start + dist;
                for i in start..end {
                    if board[i][x] > 0 {
                        intersections.push((i, x, board[i][x], step));
                    }
                    step += 1;
                }
                (x, end - 1)
            },

            Point::D(dist) => {
                let start = y - dist;
                let mut steps = step + dist - 1;
                for i in start..y {
                    if board[i][x] > 0 {
                        intersections.push((i, x, board[i][x], steps));
                    }
                    steps -= 1;
                }
                step += dist;
                (x, start)
            }
        };
        x = a.0;
        y = a.1;
    }

    // for a in board.iter().rev() {
    //     for b in a {
    //         print!("{0:3}", b);
    //     }
    //     println!("");
    // }

    intersections.iter()
        .map(|(x, y, s1, s2)| {
            let x_dist = max(*x, origin.0) - min(*x, origin.0);
            let y_dist = max(*y, origin.1) - min(*y, origin.1);
            (x_dist + y_dist, s1 + s2)
        })
        .fold(None, |cur, next| {
            match cur {
                Some((dist_from_origin, steps)) => {
                    let dist_from_origin = if dist_from_origin > next.0 {
                        next.0
                    } else {
                        dist_from_origin
                    };
                    let steps = if steps > next.1 {
                        next.1
                    } else {
                        steps
                    };
                    Some((dist_from_origin, steps))
                },
                None => Some(next)
            }
        })
}

fn main() -> Result<(), Error> {
    let file = fs::File::open("day_03_input.txt")?;
    let reader = BufReader::new(file);
    let (wire1, wire2) = read_input(reader)?;

    if let Some((part1res, part2res)) = find_closest_intersection(&wire1, &wire2, 20000, (9500, 9500)) {
        println!("Part1: {}", part1res);
        println!("Part2: {}", part2res);
    } else {
        println!("NOT FOUND!");
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() -> Result<(), Error> {
        let input = "R8,U5,L5,D3
U7,R6,D4,L4";
        let reader = io::Cursor::new(input.as_bytes());
        let (wire1, wire2) = read_input(reader)?;

        assert_eq!(find_closest_intersection(&wire1, &wire2, 11, (1, 1)), Some((6, 30)));

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), Error> {

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let reader = io::Cursor::new(input.as_bytes());
        let (wire1, wire2) = read_input(reader)?;

        assert_eq!(wire1, vec![Point::R(75), Point::D(30), Point::R(83), Point::U(83), Point::L(12),
                               Point::D(49), Point::R(71), Point::U(7), Point::L(72)]);
        assert_eq!(wire2, vec![Point::U(62), Point::R(66), Point::U(55), Point::R(34), Point::D(71),
                               Point::R(55), Point::D(58), Point::R(83)]);

        assert_eq!(find_closest_intersection(&wire1, &wire2, 3000, (1499, 1499)), Some((159, 610)));

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), Error> {

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let reader = io::Cursor::new(input.as_bytes());
        let (wire1, wire2) = read_input(reader)?;

        assert_eq!(find_closest_intersection(&wire1, &wire2, 3000, (1499, 1499)), Some((135, 410)));

        Ok(())
    }
}
