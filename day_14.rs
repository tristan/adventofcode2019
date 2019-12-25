use std::io;
use std::fs;
use std::collections::HashMap;

type Inputs = Vec<(String, isize)>;
type InputMap = HashMap<String, (isize, Inputs)>;

fn read_input<R: io::Read>(mut reader: R) -> Result<InputMap, io::Error> {
    let mut lines: String = String::new();
    reader.read_to_string(&mut lines)?;
    Ok(lines.trim().split("\n").map(|line| {
        let mut parts = line.trim().split("=>");
        let inputs = parts.next().unwrap().split(", ").map(|input| {
            let mut input = input.trim().split(" ");
            let amount = input.next().unwrap().parse::<isize>().unwrap();
            let chem = input.next().unwrap().to_string();
            (chem, amount)
        }).collect::<Vec<_>>();
        let mut output = parts.next().unwrap().trim().split(" ");
        let amount = output.next().unwrap().parse::<isize>().unwrap();
        let chem = output.next().unwrap().to_string();
        (chem, (amount, inputs))
    }).collect())
}

fn div_ceil(a: isize, b: isize) -> isize {
    let d = a / b;
    let m = a % b;
    if m > 0 {
        d + 1
    } else {
        d
    }
}

fn ore_for(inputmap: &InputMap, available: &mut HashMap<String, isize>, chem: &str, output_required: isize) -> isize {
    let (output_amount, inputs) = inputmap.get(chem).unwrap();
    // println!("{}: Required: {}",
    //          chem, output_required);
    let output_available = *available.get(chem).unwrap_or(&0);
    let output_to_produce = output_required - output_available;
    let mul = (div_ceil(output_to_produce, *output_amount)).max(1);
    //println!("mul: {}, available: {}",
    //mul, output_available);
    if output_available >= output_required {
        //println!("^^ taken: {}", output_required);
        let output_remaining = output_available - output_required;
        available.insert(chem.to_string(), output_remaining);
        0
    } else {
        // produce!
        let output_produced = output_amount * mul;
        let ore = inputs.iter().map(|(input_chem, input_amount)| {
            //println!("INPUT: {} -> {}", input_chem, input_amount);
            if input_chem == "ORE" {
                //println!("ORE MINED: {}", input_amount * mul);
                input_amount * mul
            } else {
                ore_for(inputmap, available, input_chem, input_amount * mul)
            }
        }).sum();
        let output_remaining = (output_available + output_produced) - output_required;
        //println!("Produced: {} {}, Remaining: {}", chem, output_produced, output_remaining);
        available.insert(chem.to_string(), output_remaining);
        ore
    }
}

fn ore_for_fuel(inputmap: &InputMap, amount: isize) -> isize {
    let mut available = HashMap::new();
    ore_for(inputmap, &mut available, "FUEL", amount)
}

fn solve_part1(inputmap: &InputMap) -> isize {
    ore_for_fuel(inputmap, 1)
}

fn solve_part2(inputmap: &InputMap, ore_per_fuel: isize) -> isize {
    let mut low = 1000000000000 / ore_per_fuel;
    let mut high = 1000000000000;
    loop {
        if high - low <= 1 { return low; }
        let mid = (low + high) / 2;
        let ore = ore_for_fuel(inputmap, mid);
        if ore > 1000000000000 {
            high = mid;
        } else {
            low = mid;
        }
    }
}

fn main() -> Result<(), io::Error> {
    let inputmap = read_input(fs::File::open("day_14_input.txt")?)?;

    let p1 = solve_part1(&inputmap);
    println!("Part1: {}", p1);
    let p2 = solve_part2(&inputmap, p1);
    println!("Part2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_one() -> Result<(), io::Error> {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

        let input = io::Cursor::new(input);
        let inputmap = read_input(input)?;

        let ore = solve_part1(&inputmap);
        assert_eq!(ore, 31);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), io::Error> {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

        let input = io::Cursor::new(input);
        let inputmap = read_input(input)?;

        let ore = solve_part1(&inputmap);
        assert_eq!(ore, 165);

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), io::Error> {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let input = io::Cursor::new(input);
        let inputmap = read_input(input)?;

        let ore = solve_part1(&inputmap);
        assert_eq!(ore, 13312);

        assert_eq!(solve_part2(&inputmap, 13312), 82892753);

        Ok(())
    }

    #[test]
    fn test_four() -> Result<(), io::Error> {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

        let input = io::Cursor::new(input);
        let inputmap = read_input(input)?;

        let ore = solve_part1(&inputmap);
        assert_eq!(ore, 180697);

        assert_eq!(solve_part2(&inputmap, 180697), 5586022);

        Ok(())
    }

    #[test]
    fn test_five() -> Result<(), io::Error> {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

        let input = io::Cursor::new(input);
        let inputmap = read_input(input)?;

        let ore = solve_part1(&inputmap);
        assert_eq!(ore, 2210736);

        assert_eq!(solve_part2(&inputmap, 2210736), 460664);

        Ok(())
    }

}
