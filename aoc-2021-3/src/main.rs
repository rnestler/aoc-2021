use std::fs::File;
use std::io::prelude::*;

fn part_1(input: &str) -> u32 {
    let mut line_count = 0;
    let mut ones = vec![];
    for line in input.lines() {
        line_count += 1;
        for (index, byte) in line.bytes().enumerate() {
            let bit: bool = match byte {
                b'0' => false,
                b'1' => true,
                v => panic!("Unexpected byte {}", v),
            };
            if index + 1 > ones.len() {
                ones.resize(index + 1, 0);
            }
            if bit {
                ones[index] += 1;
            }
        }
    }
    let mut epsilon = 0;
    let mut gamma = 0;
    for n in ones {
        epsilon <<= 1;
        gamma <<= 1;
        if n > line_count / 2 {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    epsilon * gamma
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let solution_1 = part_1(&contents);
    println!("Part 1: {}", solution_1);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "00100\n\
                     11110\n\
                     10110\n\
                     10111\n\
                     10101\n\
                     01111\n\
                     00111\n\
                     11100\n\
                     10000\n\
                     11001\n\
                     00010\n\
                     01010\n";
        let result_1 = part_1(input);
        assert_eq!(result_1, 198);
    }
}
