use std::io;
use std::fs;
use std::mem;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
enum Error {
    IoError(io::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Iterative_version_in_C
fn gcd(mut u: isize, mut v: isize) -> isize {
    if u == 0 { return v.abs() }; // result should always be positive
    if v == 0 { return u.abs() };

    // ignore negatives, because it doesn't matter
    u = u.abs();
    v = v.abs();

    // store common factors of 2
    let shift = (u | v).trailing_zeros();

    // remove all factors of 2 in u
    u >>= u.trailing_zeros();

    loop {
        // remove all factors of 2 in v
        v >>= v.trailing_zeros();
        if u > v {
            mem::swap(&mut u, &mut v);
        }
        v -= u;

        if v == 0 { break; }
    };

    // restore common factors of 2
    u << shift
}

#[derive(Clone)]
struct AsteroidDestructionIterator {
    data: VecDeque<(Point, f32, f32)>,
    next_round: VecDeque<(Point, f32, f32)>,
    position: Point,
    previous: Option<(Point, f32, f32)>
}

impl AsteroidDestructionIterator {
    fn new(input: &[bool], dimensions: Point, position: Point) -> AsteroidDestructionIterator {
        let mut data = input.iter()
            .enumerate()
            .filter_map(|(idx, p)| {
                if *p {
                    let y = idx as isize / dimensions.x;
                    let x = idx as isize - (y * dimensions.x);

                    let xd = x - position.x;
                    let yd = y - position.y;

                    let angle = 180.0 * f32::atan2(xd as _, yd as _) / std::f32::consts::PI;
                    let dist = f32::sqrt(((xd * xd) + (yd * yd)) as _);

                    Some((Point::new(x, y), dist, angle))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        data.sort_unstable_by(|a, b| {
            match b.2.partial_cmp(&a.2).unwrap() {
                Ordering::Equal => { a.1.partial_cmp(&b.1).unwrap() },
                other => other
            }
        });
        AsteroidDestructionIterator {
            data: data.into(),
            next_round: VecDeque::new(),
            position,
            previous: None
        }
    }
}

impl Iterator for AsteroidDestructionIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.data.pop_front() {
                Some(next) => {
                    match self.previous {
                        Some(prev) => {
                            if prev.2 == next.2 {
                                self.next_round.push_back(next);
                                continue;
                            }
                            self.previous = Some(next);
                            return Some(next.0);
                        },
                        None => {
                            self.previous = Some(next);
                            return Some(next.0);
                        }
                    }
                },
                None => {
                    if self.next_round.is_empty() {
                        return None;
                    } else {
                        mem::swap(&mut self.data, &mut self.next_round);
                        self.previous = None;
                        continue;
                    }
                }
            }
        }
    }
}

fn read_input<R: io::Read>(mut reader: R) -> Result<(Vec<bool>, Point), Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;
    let lines = lines.trim();
    let y = lines.split("\n").count() as isize;
    let x = lines.split("\n").nth(0).unwrap().len() as isize;
    Ok((lines.chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|c| c == '#')
        .collect(),
        Point::new(x, y)))
}

fn count_direct_line_of_sight(input: &[bool], dimensions: Point) -> Vec<usize> {
    input.iter()
        .enumerate()
        .map(|(idx1, p1)| {
            if !p1 {
                0
            } else {
                let y1 = idx1 as isize / dimensions.x;
                let x1 = idx1 as isize - (y1 * dimensions.x);
                let points = input.iter()
                    .enumerate()
                    .filter_map(|(idx2, p2)| {
                        if !p2 || idx1 == idx2 {
                            None
                        } else {
                            let y2 = idx2 as isize / dimensions.x;
                            let x2 = idx2 as isize - (y2 * dimensions.x);

                            let xd = x2 - x1;
                            let yd = y2 - y1;

                            let div = gcd(xd, yd);
                            let uv = (xd / div, yd / div);
                            Some(uv)
                        }
                    })
                    .collect::<HashSet<_>>();
                points.len()
            }
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let (input, dimensions) = read_input(fs::File::open("day_10_input.txt")?)?;
    let result = count_direct_line_of_sight(&input, dimensions);
    let (i, v) = result.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    println!("Part1: {}", v);

    let y = i as isize / dimensions.x;
    let x = i as isize - (y * dimensions.x);

    let adi = AsteroidDestructionIterator::new(&input, dimensions, Point::new(x, y));
    let point = adi.skip(199).next().unwrap();
    println!("Part2: {}", point.x * 100 + point.y);


    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() -> Result<(), Error> {
        let input = "
.#..#
.....
#####
....#
...##
";
        let (input, dimensions) = read_input(io::Cursor::new(input))?;
        assert_eq!(input.len() as isize, dimensions.x * dimensions.y);
        assert_eq!(input[0], false);
        assert_eq!(input[1], true);
        assert_eq!(dimensions, Point::new(5, 5));

        let result = count_direct_line_of_sight(&input, dimensions);
        // for y in 0..5 {
        //     for x in 0..5 {
        //         print!("{}", result[y * dimensions.0 + x]);
        //     }
        //     println!("");
        // }
        assert_eq!(result, [0,7,0,0,7,0,0,0,0,0,6,7,7,7,5,0,0,0,0,7,0,0,0,8,7]);
        assert_eq!(*result.iter().max().unwrap(), 8);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), Error> {
        let input = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
        let (input, dimensions) = read_input(io::Cursor::new(input))?;
        assert_eq!(input.len() as isize, dimensions.x * dimensions.y);

        let result = count_direct_line_of_sight(&input, dimensions);
        assert_eq!(*result.iter().max().unwrap(), 33);

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), Error> {
        let input = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
        let (input, dimensions) = read_input(io::Cursor::new(input))?;
        assert_eq!(input.len() as isize, dimensions.x * dimensions.y);

        let result = count_direct_line_of_sight(&input, dimensions);
        assert_eq!(*result.iter().max().unwrap(), 210);

        Ok(())
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(-6, 9), 3);
        assert_eq!(gcd(13, 13), 13);
        assert_eq!(gcd(37, 600), 1);
        assert_eq!(gcd(20, 100), 20);
        assert_eq!(gcd(624129, 2061517), 18913);
    }

    #[test]
    fn test_destroy() -> Result<(), Error> {
        let input = "
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##
";

        let (input, dimensions) = read_input(io::Cursor::new(input))?;
        let pos = Point::new(8, 3);

        let adi = AsteroidDestructionIterator::new(&input, dimensions, pos);
        adi.for_each(|p| println!("{},{}", p.x, p.y));

        Ok(())
    }

    #[test]
    fn test_destroy_two() -> Result<(), Error> {
        let input = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

        let (input, dimensions) = read_input(io::Cursor::new(input))?;
        let pos = Point::new(11, 13);

        let adi = AsteroidDestructionIterator::new(&input, dimensions, pos);
        //adi.enumerate().for_each(|(i, p)| println!("{}: [{}, {}]", i, p.x, p.y));
        assert_eq!(adi.skip(199).next().unwrap(), Point::new(8, 2));
        Ok(())
    }
}
