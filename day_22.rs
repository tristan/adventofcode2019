use std::io::{self, BufRead, BufReader};
use std::fs;

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    // https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
    // only works if values are prime
    if modulus == 1 {
        0
    } else {
        let mut result = 1;
        base = base.rem_euclid(modulus);
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base).rem_euclid(modulus);
            }
            exp >>= 1;
            base = (base * base).rem_euclid(modulus);
        }
        result
    }
}

fn mod_inv(a: i128, n: i128) -> i128 {
    // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
    let mut t = 0;
    let mut r = n;
    let mut newt = 1;
    let mut newr = a;

    while newr != 0 {
        let quotient = r.div_euclid(newr);
        let (a, b) = (newt, t - quotient * newt);
        t = a;
        newt = b;

        let (a, b) = (newr, r - quotient * newr);
        r = a;
        newr = b;
    }

    if r > 1 {
        panic!("a is not invertible");
    }
    if t < 0 {
        t += n;
    }
    t
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Step {
    DealWithIncrement(isize),
    Cut(isize),
    DealIntoNewStack
}

fn read_input<R: io::Read>(reader: R) -> Result<Vec<Step>, io::Error> {
    let reader = BufReader::new(reader);
    Ok(reader.lines()
        .map(|line| Ok(line?))
        .collect::<Result<Vec<String>, io::Error>>()?
        .into_iter()
        .filter_map(|line| {
            if line.starts_with("deal with increment") {
                Some(Step::DealWithIncrement(line[20..].parse::<isize>().unwrap()))
            } else if line.starts_with("cut") {
                Some(Step::Cut(line[4..].parse::<isize>().unwrap()))
            } else if line == "deal into new stack" {
                Some(Step::DealIntoNewStack)
            } else {
                None
            }
        }).collect())
}

fn shuffle(steps: &[Step], deck_size: isize) -> Vec<isize> {
    let initial_deck = (0..deck_size).collect::<Vec<isize>>();

    steps.into_iter().fold(initial_deck, |mut current_deck, step| {
        match step {
            Step::DealIntoNewStack => {
                current_deck.into_iter().rev().collect()
            },
            Step::Cut(cut) => {
                if *cut < 0 {
                    current_deck.rotate_right(cut.abs() as _);
                    current_deck
                } else {
                    current_deck.rotate_left(*cut as _);
                    current_deck
                }
            },
            Step::DealWithIncrement(incr) => {
                let mut new_deck = vec![0isize; deck_size as usize];
                for i in 0..deck_size {
                    let offset = i * incr % deck_size;
                    new_deck[offset as usize] = current_deck[i as usize];
                }
                new_deck
            }
        }
    })
}

fn steps_as_linear_function(steps: &[Step], deck_size: i128) -> (i128, i128) {
    // https://en.wikipedia.org/wiki/Linear_function
    // f(x)=ax+b
    // so, figure out a and b for each function

    // f(x)=ax+b
    // g(x)=cx+d
    // g(f(x)) = c * f(x) + d
    // g(f(x)) = c * (ax + b) + d
    // g(f(x)) = cax + cb + d

    steps.into_iter().rev().fold((1, 0), |(a, b), step| {
        match step {
            Step::DealIntoNewStack => {
                // g(x) = cx + d
                // g(0) = 0 + (deck_size - 1)
                // g(1) = -1 + deck_size - 1
                // g(2) = -2 + deck_size - 1
                // c = -1
                // d = deck_size - 1
                // compose with previous a, b
                // g(f(x)) = cax + cb + d
                // a = ca, b = cb + d
                // c = -1, d = deck_size - 1
                // a = -1 * a = -a
                // b = (-1 * b) + deck_size - 1 = -b + deck_size - 1 = deck_size - b - 1
                (-a.rem_euclid(deck_size), deck_size - b - 1)
            },
            Step::Cut(cut) => {
                let cut = *cut as i128;
                // cut = 3, deck_size = 10
                // g(x) = cx + d
                // g(2) = 5
                // c2 + d = 5
                // 2 + cut = 5
                // g(6) = 9
                // 6 + cut = 9
                // g(9) = 2
                // (9 + cut) % deck_size = (9 + 3) % 10 = 12 % 10 = 2
                // cut -4, deck_size = 10
                // g(2) = 8
                // 2 + -4 = -2 % 10 = 8
                // compose with previous a, b
                // g(f(x)) = cax + cb + d
                // a = ca, b = cb + d
                // c = 1, d = cut
                // a = 1 * a = a
                // b = 1 * b + cut
                (a, (b + cut).rem_euclid(deck_size))
            },
            Step::DealWithIncrement(incr) => {
                let incr = *incr as i128;
                // g(x) = cx + d
                // c = mod_inv(incr, deck_size), d = 0
                // compose with previous a, b
                // g(f(x)) = cax + cb + d
                // a = ca, b = cb + d
                // a = mod_inv(incr, deck_size) * a
                // b = mod_inv(incr, deck_size) * b
                let m = mod_inv(incr, deck_size);
                ((a * m).rem_euclid(deck_size), (b * m).rem_euclid(deck_size))
            }
        }
    })
}


fn main() -> Result<(), io::Error> {
    let steps = read_input(fs::File::open("day_22_input.txt")?)?;

    let part1 = shuffle(&steps, 10007)
        .iter().position(|i| *i == 2019).unwrap();
    println!("Part1: {}", part1);
    assert_eq!(part1, 7395);

    let s = std::time::Instant::now();
    let deck_size: i128 = 119315717514047;
    let shuffles: i128 = 101741582076661;
    let (a, b) = steps_as_linear_function(&steps, deck_size);

    // f(x)=ax+b
    // f(f(x)) = a(ax + b) + b
    //         = aax + ab + b
    // f(f(f(x))) = aa(ax + b) + ab + b
    //            = aaax + aab + ab + b
    // f(f(f(f(x)))) = aaa(ax + b) + aab + ab + b
    //               = aaaax + aaab + aab + ab + b
    // f(x) repeated s times = a^s * x + a^(s - 1) * b + a^(s - 2) * b + ... + b
    // a = a ^ s
    let ma = mod_pow(a, shuffles, deck_size);
    // b = a^(s - 1) * b + a^(s - 2) * b + ... + b
    // b = b * (a^0 + a^1 + ... a^s-1)
    // https://en.wikipedia.org/wiki/Geometric_progression#Geometric_series
    // b = b(1 - a^s) / 1 - a)
    // using modular inverse for divide
    let inv_1_minus_a = mod_inv(1 - a, deck_size);
    let mb = ((b * (1 - ma)).rem_euclid(deck_size) * inv_1_minus_a).rem_euclid(deck_size);

    let part2 = (ma * 2020 + mb).rem_euclid(deck_size);
    println!("Part2: {} ({:?})", part2, s.elapsed());

    Ok(())
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_input() -> Result<(), io::Error> {

        let input = "deal with increment 7
deal into new stack
deal into new stack
deal with increment 7
deal with increment 9
cut -2
";
        let reader = io::Cursor::new(input.as_bytes());
        let steps = read_input(reader)?;

        assert_eq!(steps, vec![
            Step::DealWithIncrement(7),
            Step::DealIntoNewStack,
            Step::DealIntoNewStack,
            Step::DealWithIncrement(7),
            Step::DealWithIncrement(9),
            Step::Cut(-2)
        ]);

        Ok(())
    }

    #[test]
    fn test_one() -> Result<(), io::Error> {

        let input = "deal with increment 7
deal into new stack
deal into new stack
";
        let reader = io::Cursor::new(input.as_bytes());
        let steps = read_input(reader)?;
        let result = shuffle(&steps, 10);

        assert_eq!(result, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let (a, b) = steps_as_linear_function(&steps, 10);
        let f = move |x: i128| ((a * x) + b).rem_euclid(10);

        for (pos, expected) in result.iter().enumerate() {
            assert_eq!(f(pos as _), *expected as _);
        }
        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), io::Error> {

        let input = "cut 6
deal with increment 7
deal into new stack
";
        let reader = io::Cursor::new(input.as_bytes());
        let steps = read_input(reader)?;
        let result = shuffle(&steps, 10);

        assert_eq!(result, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let (a, b) = steps_as_linear_function(&steps, 10);
        let f = move |x: i128| ((a * x) + b).rem_euclid(10);

        for (pos, expected) in result.iter().enumerate() {
            assert_eq!(f(pos as _), *expected as _);
        }

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), io::Error> {

        let input = "deal with increment 7
deal with increment 9
cut -2";
        let reader = io::Cursor::new(input.as_bytes());
        let steps = read_input(reader)?;
        let result = shuffle(&steps, 10);

        assert_eq!(result, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let (a, b) = steps_as_linear_function(&steps, 10);
        let f = move |x: i128| ((a * x) + b).rem_euclid(10);

        for (pos, expected) in result.iter().enumerate() {
            assert_eq!(f(pos as _), *expected as _);
        }

        Ok(())
    }

    #[test]
    fn test_four() -> Result<(), io::Error> {

        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
";
        let reader = io::Cursor::new(input.as_bytes());
        let steps = read_input(reader)?;
        let result = shuffle(&steps, 10);

        assert_eq!(result, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);

        let (a, b) = steps_as_linear_function(&steps, 10);
        let f = move |x: i128| ((a * x) + b).rem_euclid(10);

        for (pos, expected) in result.iter().enumerate() {
            assert_eq!(f(pos as _), *expected as _);
        }

        Ok(())
    }

    #[test]
    fn test_cut_minus_1() {
        let result = shuffle(&[Step::Cut(-1)], 3);
        assert_eq!(result, [2, 0, 1]);
    }

    #[test]
    fn test_cut_pos() {
        let result = shuffle(&[Step::Cut(3)], 10);
        assert_eq!(result, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        let (a, b) = steps_as_linear_function(&[Step::Cut(3)], 10);
        let f = move |x: i128| ((a * x) + b).rem_euclid(10);

        assert_eq!(f(2), 5);
        assert_eq!(f(6), 9);
        assert_eq!(f(9), 2);
        assert_eq!(f(0), 3);
    }

    #[test]
    fn test_cut_neg() {
        let result = shuffle(&[Step::Cut(-4)], 10);
        assert_eq!(result, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);

        let (a, b) = steps_as_linear_function(&[Step::Cut(-4)], 10);
        let f = move |x: i128| ((a * x) + b).rem_euclid(10);

        assert_eq!(f(2), 8);
        assert_eq!(f(6), 2);
        assert_eq!(f(5), 1);
    }

    #[test]
    fn test_deal_with_increment() {
        let result = shuffle(&[Step::DealWithIncrement(3)], 10);
        assert_eq!(result, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
