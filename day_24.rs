use std::collections::{HashSet, HashMap};

trait Mutate {
    fn mutate(self) -> Self;
}

impl Mutate for u32 {
    fn mutate(self) -> u32 {
        (0..25).fold(0, |acc, i| {
            let count =
            // top
                if i >= 5 && self & (1 << i - 5) >= 1 {
                    1
                } else {
                    0
                }
            +
            // bottom
                if i < 20 && self & (1 << i + 5) >= 1 {
                    1
                } else {
                    0
                }
            +
            // left
                if i % 5 != 0 && self & (1 << i - 1) >= 1 {
                    1
                } else {
                    0
                }
            +
            // right
                if i % 5 != 4 && self & (1 << i + 1) >= 1 {
                    1
                } else {
                    0
                }
            ;

            let id = 1 << i;
            let bug = self & id >= 1;
            if (bug && count == 1) || (!bug && (count == 1 || count == 2)) {
                acc | id
            } else {
                acc
            }
        })
    }
}

fn mutate_recursive(prev: u32, this: u32, next: u32) -> u32 {
    (0..25).fold(0, |acc, i| {
        if i == 12 {
            acc
        } else {
            let count =
            // top
                if i == 17 {
                    // add up bottom row of next level
                    (next >> 20).count_ones()
                } else if i >= 5 {
                    1 & this >> i - 5
                } else {
                    1 & prev >> 7
                }
            +
            // bottom
                if i == 7 {
                    // add up top row of next level
                    (next & 0b11111).count_ones()
                } else if i < 20 {
                    1 & this >> i + 5
                } else {
                    1 & prev >> 17
                }
            +
            // left
                if i == 13 {
                    // add up right side of next level
                    (next & 0b1000010000100001000010000).count_ones()
                } else if i % 5 != 0 {
                    1 & this >> i - 1
                } else {
                    1 & prev >> 11
                }
            +
            // right
                if i == 11 {
                    // add up left side of next level
                    (next & 0b100001000010000100001).count_ones()
                } else if i % 5 != 4 {
                    1 & this >> i + 1
                } else {
                    1 & prev >> 13
                }
            ;

            let id = 1 << i;
            let bug = this & id >= 1;
            //println!("{} : {} : {}", id, bug, count);
            if (bug && count == 1) || (!bug && (count == 1 || count == 2)) {
                acc | id
            } else {
                acc
            }
        }
    })
}

impl Mutate for HashMap<i32, u32> {
    fn mutate(self) -> HashMap<i32, u32> {
        let mut result = HashMap::new();

        let mut max_level = 0;
        let mut min_level = 0;
        for &level in self.keys() {
            max_level = max_level.max(level);
            min_level = min_level.min(level);
            let prev = self.get(&(level - 1)).unwrap_or(&0);
            let this = self.get(&level).unwrap();
            let next = self.get(&(level + 1)).unwrap_or(&0);

            let mutated = mutate_recursive(*prev, *this, *next);

            result.insert(level, mutated);
        }

        let prev = *self.get(&max_level).unwrap();
        let this = 0;
        let next = 0;
        let mutated = mutate_recursive(prev, this, next);
        if mutated > 0 {
            result.insert(max_level + 1, mutated);
        }

        let prev = 0;
        let this = 0;
        let next = *self.get(&min_level).unwrap();
        let mutated = mutate_recursive(prev, this, next);
        if mutated > 0 {
            result.insert(min_level - 1, mutated);
        }

        result
    }
}

fn parse_map(input: &str) -> u32 {
    input.chars().filter_map(|c| match c {
        '#' => Some(true),
        '.' | '?' => Some(false),
        '\n' => None,
        _ => panic!("invalid char")
    }).enumerate().fold(0_u32, |acc, (i, n)| {
        if n {
            acc | 1 << i
        } else {
            acc
        }
    })
}

fn find_duplicate(mut map: u32) -> u32 {
    let mut cache = HashSet::new();
    loop {
        map = map.mutate();
        //println!("{:025b}", map);
        if !cache.insert(map) {
            break;
        }
    }
    map
}

fn count_bugs(maps: HashMap<i32, u32>) -> u32 {
    maps.values().copied().map(u32::count_ones).sum()
}

fn part1(input: &str) -> u32 {
    let map = parse_map(input);
    let dup = find_duplicate(map);
    dup
}

fn part2(input: &str) -> u32 {
    let mut maps = HashMap::new();
    maps.insert(0, parse_map(input));
    count_bugs((0..200).fold(maps, |acc, _| {
        acc.mutate()
    }))
}

fn main() {
    let input = "##.#.##.#.##.##.####.#...";

    let s = std::time::Instant::now();
    let part1 = part1(input);
    println!("Part1: {} ({:?})", part1, s.elapsed());

    let s = std::time::Instant::now();
    let part2 = part2(input);
    println!("Part2: {} ({:?})", part2, s.elapsed());
}


#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
    fn print_map(map: u32) {
        println!("==== {} ====", map);
        println!("{:05b}", map.reverse_bits() >> 27);
        println!("{:05b}", (map >> 5).reverse_bits() >> 27);
        println!("{:05b}", (map >> 10).reverse_bits() >> 27);
        println!("{:05b}", (map >> 15).reverse_bits() >> 27);
        println!("{:05b}", (map >> 20).reverse_bits() >> 27);
    }

    #[test]
    fn test_parse_map() {
        let start = parse_map("#####....#....#...#.#.###");
        assert_eq!(start, 0b1110101000100001000011111);

        let start = parse_map("....##..#.#.?##..#..#....");
        assert_eq!(start, 0b0000100100110010100110000);
    }

    #[test]
    fn test_mutate() {
        let start = parse_map("....##..#.#..##..#..#....");
        let step1 = parse_map("#..#.####.###.###.##.##..");
        let step2 = parse_map("#####....#....#...#.#.###");
        let step3 = parse_map("#....####....###.##..##.#");
        let step4 = parse_map("####.....###..#.....##...");

        assert_eq!(start.mutate(), step1);
        assert_eq!(step1.mutate(), step2);
        assert_eq!(step2.mutate(), step3);
        assert_eq!(step3.mutate(), step4);
    }

    #[test]
    fn test_part1() {
        let input = "....##..#.#..##..#..#....";
        assert_eq!(part1(input), 2129920);
    }

    #[test]
    fn test_find_duplicate() {
        let input = "....##..#.#..##..#..#....";
        let duplicate = "...............#.....#...";
        let duplicate = parse_map(duplicate);
        assert_eq!(find_duplicate(parse_map(input)), duplicate);
    }

    #[test]
    fn test_behaviour() {
        assert_eq!((0b1110101000100001000011111u32 >> 20).count_ones(), 4);
    }

    #[test]
    fn test_recursive() {
        let start = parse_map("....##..#.#.?##..#..#....");
        let step1 = parse_map("#..#.####.##?.###.##.##..");

        assert_eq!(mutate_recursive(0, start, 0), step1);

        let level_0 = start;

        let new_m1 = mutate_recursive(level_0, 0, 0);
        let new_0 = mutate_recursive(0, level_0, 0);
        let new_p1 = mutate_recursive(0, 0, level_0);

        assert_eq!(new_m1, 33047056);
        assert_eq!(new_0, 7196137);
        assert_eq!(new_p1, 139392);

        let mut maps = HashMap::new();
        maps.insert(0, start);

        let result = (0..10).fold(maps, |acc, _| {
            acc.mutate()
        });

        let exp_m5 = parse_map("..#...#.#...?.#.#.#...#..");
        let exp_0 = parse_map(".#....#.##.#?............");
        let exp_p5 = parse_map("####.#..#.#.?#.####......");
        assert_eq!(*result.get(&-5).unwrap(), exp_m5);
        assert_eq!(*result.get(&0).unwrap(), exp_0);
        assert_eq!(*result.get(&5).unwrap(), exp_p5);
    }
}
