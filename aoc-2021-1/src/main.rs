use std::fs::File;
use std::io::prelude::*;

fn parse(input: &str) -> Vec<i32> { 
    input.lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_1(input: &Vec<i32>) -> usize {
    input.iter().zip(input[1..].iter()).filter(|(a,b)| b > a).count()
}

fn part_2(input: &Vec<i32>) -> usize {
    let averages: Vec<i32> = input.windows(3).map(|a| a.iter().sum()).collect();
    return part_1(&averages)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let input = parse(&contents);

    let solution_1 = part_1(&input);
    println!("Part 1: {}", solution_1);

    let solution_2 = part_2(&input);
    println!("Part 2: {}", solution_2);

    Ok(())
}
