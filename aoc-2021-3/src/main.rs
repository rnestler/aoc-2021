use std::fs::File;
use std::io::prelude::*;

fn count_ones(input: &str) -> Vec<u32> {
    let mut ones = vec![];
    for line in input.lines() {
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
    ones
}

fn part_1(input: &str) -> (u32, u32) {
    let mut line_count = input.lines().count();
    let mut ones = count_ones(input);
    let mut epsilon = 0;
    let mut gamma = 0;
    for n in ones {
        epsilon <<= 1;
        gamma <<= 1;
        if n as usize > line_count / 2 {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    (epsilon, gamma)
}

fn part_2(input: &str, epsilon: u32, gamma: u32) -> u32 {
    for line in input.lines() {
        for (index, byte) in line.bytes().enumerate() {
        }
    }
    let mut 
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (epsilon, gamma) = part_1(&contents);
    println!("Part 1: {}", epsilon * gamma);

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
        let (epsilon, gamma) = part_1(input);
        assert_eq!(epsilon * gamma, 198);
    }
}
