use std::fs;

fn run_phases(mut input: Vec<isize>, phases: usize) -> Vec<isize> {
    let mut p = vec![0; input.len()];
    for _phase in 0..phases {
        let mut wave = [1, 0, -1, 0].iter().cycle();
        for i in 0..input.len() {
            let mul = *wave.next().unwrap();
            if mul == 0 {
                continue;
            }
            p[0] += input[i] * mul;
            for j in 1..input.len() {
                let start = (i + 1) * (j + 1) - 1;
                let end = input.len().min(start + j + 1);
                for o in start..end {
                    p[j] += input[o] * mul;
                }
            }
        }
        for i in 0..input.len() {
            input[i] = p[i].abs() % 10;
            p[i] = 0;
        }
    }
    input
}

fn decode_signal(input: Vec<isize>) -> String {
    let offset = input.iter().take(7).enumerate().fold(0, |c, (i, x)| c + (x * 10isize.pow(6 - i as u32))) as usize;
    let length = input.len() * 10000;
    let mut slice = (0..(length - offset))
        .map(|i| input[(offset + i) % input.len()])
        .collect::<Vec<_>>();

    let n = slice.len();
    (0..100).for_each(|_p| {
        let mut sum = 0;
        (0..n).rev().for_each(|i| {
            sum += slice[i];
            slice[i] = sum.abs() % 10
        });
    });

    slice.into_iter()
        .take(8)
        .map(|d| std::char::from_digit(d as _, 10).unwrap())
        .collect()
}

fn read_input(s: &str) -> Vec<isize> {
    s.trim().chars().map(|c| c as isize - 48).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input(&fs::read_to_string("day_16_input.txt")?);
    let s = std::time::Instant::now();
    let part1 = run_phases(input.clone(), 100)
        .into_iter()
        .take(8)
        .map(|d| std::char::from_digit(d as _, 10).unwrap())
        .collect::<String>();
    println!("Part1: {} ({:?})", part1, s.elapsed());
    let s = std::time::Instant::now();
    let part2 = decode_signal(input);
    println!("Part2: {} ({:?})", part2, s.elapsed());
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = "12345678";
        let p0 = read_input(&input);
        let p1 = run_phases(p0.clone(), 1);
        assert_eq!(p1, read_input("48226158"));
        let p2 = run_phases(p0.clone(), 2);
        assert_eq!(p2, read_input("34040438"));
        let p3 = run_phases(p0.clone(), 3);
        assert_eq!(p3, read_input("03415518"));
        let p4 = run_phases(p0.clone(), 4);
        assert_eq!(p4, read_input("01029498"));
    }

    #[test]
    fn test_two() {
        let input = "80871224585914546619083218645595";
        let r = run_phases(read_input(&input), 100);
        assert_eq!(r[..8].to_vec(), read_input("24176176"));
        let input = "19617804207202209144916044189917";
        let r = run_phases(read_input(&input), 100);
        assert_eq!(r[..8].to_vec(), read_input("73745418"));
        let input = "69317163492948606335995924319873";
        let r = run_phases(read_input(&input), 100);
        assert_eq!(r[..8].to_vec(), read_input("52432133"));
    }

    #[test]
    fn test_three() {
        let input = "03036732577212944063491565474664";
        assert_eq!(decode_signal(read_input(&input)), "84462026");
        let input = "02935109699940807407585447034323";
        assert_eq!(decode_signal(read_input(&input)), "78725270");
        let input = "03081770884921959731165446850517";
        assert_eq!(decode_signal(read_input(&input)), "53553731");
    }
}
