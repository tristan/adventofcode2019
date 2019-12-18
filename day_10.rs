use std::io;
use std::fs;
use std::mem;
use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    IoError(io::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
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


fn read_input<R: io::Read>(mut reader: R) -> Result<(Vec<bool>, (usize, usize)), Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;
    let lines = lines.trim();
    let y = lines.split("\n").count();
    let x = lines.split("\n").nth(0).unwrap().len();
    Ok((lines.chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|c| c == '#')
        .collect(),
        (x, y)))
}

fn count_direct_line_of_sight(input: &[bool], dimensions: (usize, usize)) -> Vec<usize> {
    input.iter()
        .enumerate()
        .map(|(idx1, p1)| {
            if !p1 {
                0
            } else {
                let y1 = (idx1 / dimensions.0) as isize;
                let x1 = idx1 as isize - y1;
                let points = input.iter()
                    .enumerate()
                    .filter_map(|(idx2, p2)| {
                        if !p2 || idx1 == idx2 {
                            None
                        } else {
                            let y2 = (idx2 / dimensions.0) as isize;
                            let x2 = idx2 as isize - y2;

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
    println!("Part1: {}", result.iter().max().unwrap());
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
        assert_eq!(input.len(), dimensions.0 * dimensions.1);
        assert_eq!(input[0], false);
        assert_eq!(input[1], true);
        assert_eq!(dimensions, (5, 5));

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
        assert_eq!(input.len(), dimensions.0 * dimensions.1);

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
        assert_eq!(input.len(), dimensions.0 * dimensions.1);

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
}
