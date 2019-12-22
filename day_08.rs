use std::fs;
use std::io;

#[derive(Debug)]
enum Error {
    IoError(io::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

fn decode_image(input: &[u32], size: usize) -> Vec<u32> {
    (0..size)
        .map(|i| {
            *input[i..]
                .iter()
                .step_by(size)
                .skip_while(|&x| *x == 2)
                .next()
                .unwrap()
        })
        .collect()

}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day_08_input.txt")?
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    let p1_result = input
        .chunks(25 * 6)
        .map(|layer| (layer, layer.iter().filter(|&v| *v == 0).count()))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(layer, _)| {
            let ones = layer.iter().filter(|&v| *v == 1).count();
            let twos = layer.iter().filter(|&v| *v == 2).count();
            ones * twos
        })
        .unwrap();
    println!("Part1: {}", p1_result);

    println!("Part2:");
    let image = decode_image(&input, 150);
    image.chunks(25).for_each(|row| {
        row.iter().for_each(|&x| if x == 0 || x == 2 { print!(" ") } else { print!("█") });
        println!("");
    });

    Ok(())
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_input() -> Result<(), Error> {
        let input = "0222112222120000"
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        assert_eq!(decode_image(&input, 4), [0,1,1,0]);
        Ok(())
    }


}
