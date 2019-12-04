fn is_sequential_part1(int: usize) -> bool {
    let size = (int as f64).log(10.0).ceil() as u32;
    (0..size)
        .map(|i| (int / 10_usize.pow(i)) % 10)
        .try_fold((10, false), |res, x| {
            if res.0 > x {
                Some((x, res.1))
            } else if res.0 == x {
                Some((x, true))
            } else {
                None
            }
        })
        .map(|(_, has_pair)| has_pair)
        .unwrap_or(false)
}

fn is_sequential_part2(int: usize) -> bool {
    let size = (int as f64).log(10.0).ceil() as u32;
    (0..size)
        .map(|i| (int / 10_usize.pow(i)) % 10)
        .try_fold((10, [0; 10]), |mut res, x| {
            if res.0 > x {
                res.1[x] = 1;
                Some((x, res.1))
            } else if res.0 == x {
                res.1[x] += 1;
                Some((x, res.1))
            } else {
                None
            }
        })
        .map(|(_, counts)| counts.iter().any(|x| *x == 2))
        .unwrap_or(false)
}

fn main() {
    let input = "128392-643281";
    let (start, end) = {
        let mut i = input.split("-").map(|n| n.parse::<usize>().unwrap());
        (i.next().unwrap(), i.next().unwrap())
    };

    let answer = (start..=end)
        .filter(|x| is_sequential_part1(*x))
        .count();
    println!("Part1: {}", answer);

    let answer = (start..=end)
        .filter(|x| is_sequential_part2(*x))
        .count();
    println!("Part2: {}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_sequential_part1() {

        assert_eq!(is_sequential_part1(111111), true);
        assert_eq!(is_sequential_part1(111121), false);
        assert_eq!(is_sequential_part1(123455), true);
        assert_eq!(is_sequential_part1(133789), true);
        assert_eq!(is_sequential_part1(128392), false);
        assert_eq!(is_sequential_part1(643281), false);
        assert_eq!(is_sequential_part1(99999999), true);
        assert_eq!(is_sequential_part1(1234567), false);
    }

    #[test]
    fn test_is_sequential_part2() {

        assert_eq!(is_sequential_part2(111122), true);
        assert_eq!(is_sequential_part2(123444), false);
        assert_eq!(is_sequential_part2(112233), true);
        assert_eq!(is_sequential_part2(111333), false);
        assert_eq!(is_sequential_part2(111442), false);
    }
}
